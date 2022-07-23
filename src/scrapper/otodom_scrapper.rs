use std::fs::File;
use std::io::Read;
use std::time::Duration;
use tokio::time::sleep;

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
    loop {
        println!("hello! {}", url);
        sleep(Duration::from_secs(60)).await;
    }
}