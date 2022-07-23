#[macro_use]
extern crate diesel;
extern crate dotenv;
mod lib;
mod commands;

use std::env;
// use diesel::RunQueryDsl;
// use thirtyfour::prelude::*;
use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};
use self::commands::{Command, start_answer};

// use tokio;

// use self::models::{User, Appartment, NewAppartment};
// use self::lib::{establish_connection, build_chrome_driver};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN should be provided");
    env::set_var("TELOXIDE_TOKEN", token);
    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, start_answer, Command::ty()).await;
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
