use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use thirtyfour::prelude::*;
use std::env;



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

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or(String::from("scrapper.db"));
    SqliteConnection::establish(&database_url).unwrap()
}
