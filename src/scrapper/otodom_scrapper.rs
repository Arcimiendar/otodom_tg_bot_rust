use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use chrono::Utc;
use teloxide::prelude::*;
use thirtyfour::prelude::*;
use tokio::time::sleep;
use crate::lib::models::{Appartment, NewAppartment};
use crate::lib::lib::{
    get_all_users, build_chrome_driver, appartment_already_exists,
    AnyResult, save_appartment
};

#[derive(Debug, Clone)]
struct ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error")
    }
}

impl Error for ParseError {}

pub fn parse_url_file(filename: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let urls = content.split("\n")
        .filter(|url| !url.trim().starts_with("#"))
        .filter(|url| url.trim().len() > 0)
        .map(|url| String::from(url))
        .collect();
    Ok(urls)
}

pub async fn parse_continuous(url: String) {
    let bot = Bot::from_env().auto_send();
    loop {
        println!("start scrapper for {}", &url);
        let appartments = parse_appartments_and_save(&url).await;
        match appartments {
            Ok(appartments) => {
                for app in appartments.iter() {
                    for user in get_all_users().unwrap_or_default().iter() {
                        let _ = bot.send_message(
                            ChatId(user.id as i64), format!("{}", app)
                        ).await;  // ignore any error here
                    }
                }
            },
            Err(_) => println!("failed scrapper for {}", &url)
        };
        println!("finish scrapper for {}", &url);
        sleep(Duration::from_secs(60)).await;
    }
}

async fn parse_appartment(
    url: &String, driver: &WebDriver
) -> AnyResult<NewAppartment> {
    let name = url.clone();
    driver.get(&name).await?;

    let parse_int = |text: String| -> AnyResult<i32> {
        Ok(
            text.split(" ")
                .filter(|price| price.parse::<i32>().is_ok())
                .map(|price| String::from(price))
                .collect::<Vec<String>>()
                .join("")
                .parse::<i32>()?
        )
    };

    let elems = driver.find_elements(
        By::XPath("//strong[@aria-label=\"Cena\"]")
    ).await?;
    if elems.len() == 0 {
        return Err(Box::new(ParseError {}));
    }
    let price_var = parse_int(elems.get(0).unwrap().text().await?)?;

    let elems = driver.find_elements(
        By::XPath("//div[@aria-label=\"Czynsz\"]/div[2]")
    ).await?;
    let mut czynsz_var: i32 = 0;
    if elems.len() > 0 {
        let parsed_czynsz = parse_int(
            elems.get(0).unwrap().text().await.unwrap_or(String::from(""))
        );
        czynsz_var = match parsed_czynsz {
            Ok(val) => val,
            Err(_) => 0,
        };
    }

    let elems = driver.find_elements(
        By::XPath("//div[@aria-label=\"Liczba pokoi\"]/div[2]")
    ).await?;
    if elems.len() == 0 {
        return Err(Box::new(ParseError {}));
    }
    let rooms_var: i32 = parse_int(elems.get(0).unwrap().text().await?)?;

    Ok(NewAppartment {
        price: Some(price_var),
        czynsz: Some(czynsz_var),
        rooms: Some(rooms_var),
        name: Some(name),
        scrapped_at: Some(Utc::now().naive_utc())
    })
}

async fn _parse_appertments_and_save(
    url: &String, driver: &WebDriver
) -> AnyResult<Box<Vec<Appartment>>> {
    driver.get(url).await?;
    let elems = driver.find_elements(
        By::XPath("//a[@data-cy=\"listing-item-link\"]")
    ).await?;
    let mut links: Vec<Option<String>> = Vec::new();
    links.reserve(elems.len());
    for el in elems.iter() {
        let attr = el.get_attribute("href").await;
        match attr {
            Ok(value) => links.push(value),
            Err(_) => { },
        };
    }
    let mut appartments: Vec<Appartment> = Vec::new();
    appartments.reserve(links.len());
    for link in links {
        if let Some(link) = link {
            let mut full_link = String::from("https://otodom.pl");
            full_link.push_str(&link);
            if !appartment_already_exists(&full_link)? {
                for _ in 0..5 {  // try to parse 5 time
                    let appartment =
                        parse_appartment(&full_link, &driver).await;
                    if let Ok(app) = appartment {
                        appartments.push(save_appartment(app)?);
                        break;
                    } else {
                        println!("error parse: {}", link);
                    }
                }
            }
        }
    }
    Ok(Box::new(appartments))
}


async fn parse_appartments_and_save(url: &String) -> AnyResult<Box<Vec<Appartment>>> {
    let driver = build_chrome_driver().await?;
    let res = _parse_appertments_and_save(url, &driver).await;
    driver.quit().await?;
    res
}