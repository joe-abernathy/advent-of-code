use std::fs::File;
use std::io::{ self, prelude::*, BufReader };
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

pub fn get_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let session_token = env::var("AOC_TOKEN")
        .expect("Environment variable AOC_TOKEN not set");

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = Client::new();

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to get input: {}", response.status()).into());
    }

    Ok(response.text()?)
}

pub fn read_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Failed"));
    }

    lines
}
