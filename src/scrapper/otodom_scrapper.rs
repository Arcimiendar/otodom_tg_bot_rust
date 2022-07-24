use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use chrono::Utc;
use teloxide::prelude::*;
use tokio::time::sleep;
use crate::lib::models::Appartment;
use crate::lib::lib::{get_all_users, build_chrome_driver};

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
        let appartments = parse_appartments(&url).await;
        match appartments {
            Ok(appartments) => {
                for app in appartments.iter() {
                    for user in get_all_users().unwrap().iter() {
                        bot.send_message(
                        ChatId(user.id as i64), format!("{}", app)
                        ).await.unwrap();
                    }
                }
            },
            Err(_) => println!("scrapper failed for {}", url)
        };
        sleep(Duration::from_secs(60)).await;
    }
}

async fn parse_appartments(url: &String) -> Result<Box<Vec<Appartment>>, Box<dyn Error>> {
    Ok(Box::new(vec![Appartment {
        id: 1,
        price: Some(5),
        czynsz: Some(5),
        name: Some(String::from("asd")),
        rooms: Some(5),
        scrapped_at: Some(Utc::now().naive_utc())
    }]))
}