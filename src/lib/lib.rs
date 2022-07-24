use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use thirtyfour::prelude::*;
use std::{env, fmt};
use std::error::Error;
use std::fmt::Formatter;
use diesel;
use crate::lib::models::User;

#[derive(Debug, Clone)]
struct InsertError;

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot insert values")
    }
}

impl Error for InsertError {}

#[derive(Debug, Clone)]
struct QueryError;

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot query values")
    }
}

impl Error for QueryError {}

pub async fn build_chrome_driver() -> WebDriverResult<WebDriver> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--headless")?;
    caps.add_chrome_arg("--disable-gpu")?;
    caps.add_chrome_arg("--no-sandbox")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;

    let driver = WebDriver::new(
        &*env::var("SELENIUM_URL")
            .unwrap_or(String::from("http://localhost:4444")), caps
    ).await?;
    Ok(driver)
}

pub fn establish_connection() -> Option<SqliteConnection> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or(String::from("scrapper.db"));
    SqliteConnection::establish(&database_url).ok()
}

pub fn register_user(user_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
    use crate::lib::schema::user;
    let connection = establish_connection();
    if let Some(connection) = connection {
        let insert_result = diesel::insert_into(user::table)
            .values(User { id: user_id })
            .execute(&connection);
        match insert_result {
            Ok(_) => Ok(()),
            Err(_) => Err(Box::new(InsertError {}))
        }
    } else {
        Err(Box::new(InsertError {}))
    }
}

pub fn get_all_users() -> Result<Box<Vec<User>>, Box<dyn Error + Send + Sync>> {
    use crate::lib::schema::user::dsl::*;
    let connection = establish_connection();
    if let Some(connection) = connection {
        Ok(Box::new(user.load::<User>(&connection)?))
    } else {
        Err(Box::new(QueryError {}))
    }
}
