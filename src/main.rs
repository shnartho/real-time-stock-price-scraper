mod fetch;

use std::thread::{self};
use serde::Deserialize;
use std::env::{self};
use std::fs;
use fetch::fetch_price;

#[derive(Debug, Deserialize)]
struct Config {
    urls: Vec<String>,
}

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join("env/config.yml");

    if !config_path.exists() {
        eprint!("Config file not found: {:?}", config_path);
        return;
    }

    let config_file = fs::read_to_string(config_path).expect("Failed to read config.yml");
    let config: Config = serde_yaml::from_str(&config_file).expect("Failed to parse config.yml");

    let mut handles = vec![];

    for url in config.urls {
        let handle = thread::spawn(move || {
            fetch_price(&url);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
