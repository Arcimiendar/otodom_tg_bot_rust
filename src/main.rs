use thirtyfour::CapabilitiesHelper;
use thirtyfour::prelude::*;
use tokio;
use dotenv::dotenv;
use std::env;

async fn build_chrome_driver() -> WebDriverResult<WebDriver> {
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


#[tokio::main]
async fn main() -> WebDriverResult<()>{
    dotenv().ok();

    let driver = build_chrome_driver().await?;


    driver.close().await?;
    Ok(())
}
