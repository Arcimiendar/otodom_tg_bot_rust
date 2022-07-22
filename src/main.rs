#[macro_use]
extern crate diesel;
extern crate dotenv;


pub mod lib;
pub mod models;
pub mod schema;

use diesel::RunQueryDsl;
use thirtyfour::prelude::*;
use tokio;
use dotenv::dotenv;

use self::models::{User, Appartment, NewAppartment};
use self::lib::{establish_connection, build_chrome_driver};


#[tokio::main]
async fn main() -> WebDriverResult<()>{
    dotenv().ok();

    use schema::appartment::dsl::*;
    use schema::appartment;
    let connection = establish_connection();

    let app = NewAppartment {
        price: Some(5),
        czynsz: Some(5),
        name: Some(String::from("hello")),
        rooms: Some(5),
        scrapped_at: None
    };
    diesel::insert_into(appartment::table)
        .values(&app)
        .execute(&connection)
        .expect("error");

    let apps = appartment.load::<Appartment>(&connection).expect("error!!");
    for app in apps {
        println!("{:?}", app);
    }
    // let driver = build_chrome_driver().await?;


    // driver.close().await?;
    Ok(())
}
