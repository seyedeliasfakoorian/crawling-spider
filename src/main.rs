use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use std::{env, process};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: crawling-spider <url>");
        process::exit(1);
    }

    let url = &args[1];
    fetch_data_and_print(url).await
}

async fn fetch_data_and_print(url: &str) -> Result<(), Box<dyn Error>> {
    // Fetch the url content
    let content = reqwest::get(url).await?.text().await?;

    // Parse the document
    let document = Html::parse_document(&content);
    let body_selector = Selector::parse("body").unwrap();

    // Find and print the body of the document
    if let Some(body) = document.select(&body_selector).next() {
        println!("Body: \n{}", body.inner_html());
    }

    Ok(())
}