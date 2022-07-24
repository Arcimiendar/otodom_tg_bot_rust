use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use thirtyfour::prelude::*;
use std::{env, fmt};
use std::error::Error;
use std::fmt::Formatter;
use diesel;
use crate::lib::models::{Appartment, NewAppartment, User};

pub type AnyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

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

pub fn register_user(user_id: i32) -> AnyResult<()> {
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

pub fn get_all_users() -> AnyResult<Box<Vec<User>>> {
    use crate::lib::schema::user::dsl::*;
    let connection = establish_connection();
    if let Some(connection) = connection {
        Ok(Box::new(user.load::<User>(&connection)?))
    } else {
        Err(Box::new(QueryError {}))
    }
}

pub fn appartment_already_exists(url: &String) -> AnyResult<bool> {
    use crate::lib::schema::appartment::dsl::*;
    let connection = establish_connection();
    if let Some(connection) = connection {
        Ok(
            appartment
                .filter(name.eq(url))
                .load::<Appartment>(&connection)?
                .len() != 0
        )
    } else {
        Err(Box::new(QueryError {}))
    }
}

pub fn save_appartment(new_app: NewAppartment) -> AnyResult<Appartment> {
    use crate::lib::schema::appartment;
    let connection = establish_connection();
    if let Some(connection) = connection {
        let insert_result = diesel::insert_into(appartment::table)
            .values(&new_app)
            .execute(&connection);
        match insert_result {
            Ok(id) => Ok(Appartment {
                id: id as i32,  // id is not critical. Could be skipped
                price: new_app.price,
                czynsz: new_app.czynsz,
                scrapped_at: new_app.scrapped_at,
                rooms: new_app.rooms,
                name: new_app.name
            }),
            Err(_) => Err(Box::new(InsertError {}))
        }
    } else {
        Err(Box::new(InsertError {}))
    }
}
