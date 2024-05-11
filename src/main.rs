use std::thread;
use std::time::Duration;

use reqwest;
use scraper::{Html, Selector};
use flate2::read::GzDecoder;
use std::io::Read;
use chrono::Local;

fn main() {
    let url = "https://finance.yahoo.com/quote/BTC-USD";

    loop {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                match response.headers().get("content-encoding") {
                    Some(encoding) if encoding == "gzip" => {
                        let mut decoder = GzDecoder::new(response);
                        let mut decompressed_data = String::new();
                        decoder.read_to_string(&mut decompressed_data).unwrap();

                        let document = Html::parse_document(&decompressed_data);

                        let price_selector = Selector::parse("fin-streamer[data-field=\"regularMarketPrice\"]").unwrap();
                        let price_element = document.select(&price_selector).next();

                        match price_element {
                            Some(element) => {
                                let price = element.value().attr("data-value").unwrap_or_else(|| {
                                    println!("Price element found, but 'data-value' attribute is missing!");
                                    "" 
                                }); 
                                let current_time = Local::now();
                                println!("{}: BTC-USD price {}$", current_time, price);
                            }
                            None => {
                                println!("Failed to find price element on the page.");
                                let snippet: String = decompressed_data.chars().take(500).collect();
                                println!("HTML Snippet:\n{}", snippet); 
                            }
                        }

                    },
                    _ => {
                        // Handle responses that are not gzip encoded (or encoding header is missing)
                        println!("Response is not gzip encoded or encoding header is missing.");
                        match response.text() {
                            Ok(text) => {
                                println!("{}", text)
                            },
                            Err(e) => println!("Error getting response text: {}", e),
                        }
                    }
                }                
            }
            Err(e) => println!("Error making request: {}", e),
        }

        thread::sleep(Duration::from_secs(1));
    }
}