#[macro_use]
extern crate diesel;
extern crate dotenv;
mod lib;
mod commands;
mod scrapper;

use std::{env, iter};
use std::future::Future;
use std::pin::Pin;
// use diesel::RunQueryDsl;
// use thirtyfour::prelude::*;
use dotenv::dotenv;
use futures::future::{join_all};
use teloxide::{prelude::*, utils::command::BotCommands};
use self::commands::{Command, start_answer};
use crate::scrapper::otodom_scrapper::{parse_url_file, parse_continuous};

// use tokio;

// use self::models::{User, Appartment, NewAppartment};
// use self::lib::{establish_connection, build_chrome_driver};

fn future_to_dynamic_future<'a, T>(
    future: impl Future<Output = T> + 'a + 'static
) -> Pin<Box<dyn Future<Output = T>>> {
    Box::pin(future)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let urls = parse_url_file("urls_to_scrap.txt")
        .expect("Urls to scrap must be provided");
    println!("Got next urls to parse:");
    urls.iter().for_each(|str| println!("{}", str));

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN should be provided");
    env::set_var("TELOXIDE_TOKEN", token);
    let bot = Bot::from_env().auto_send();
    let tg_future = teloxide::commands_repl(
        bot, start_answer, Command::ty()
    );
    let urls_futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = urls
        .into_iter()
        .map(|url| future_to_dynamic_future(parse_continuous(url)))
        .chain(iter::once(future_to_dynamic_future(tg_future)))
        .collect();
    join_all(urls_futures).await;
    // use schema::appartment::dsl::*;
    // use schema::appartment;
    // let connection = establish_connection();
    //
    // let app = NewAppartment {
    //     price: Some(5),
    //     czynsz: Some(5),
    //     name: Some(String::from("hello")),
    //     rooms: Some(5),
    //     scrapped_at: None
    // };
    // diesel::insert_into(appartment::table)
    //     .values(&app)
    //     .execute(&connection)
    //     .expect("error");
    //
    // let apps = appartment.load::<Appartment>(&connection).expect("error!!");
    // for app in apps {
    //     println!("{:?}", app);
    // }
    // let driver = build_chrome_driver().await?;

    // driver.close().await?;
    // Ok(())
}
