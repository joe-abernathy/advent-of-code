use std::fs::{self, File};
use std::io::{ prelude::*, BufReader };
use std::env;
use reqwest::blocking::Client;
use std::error::Error;


// Gets the puzzle input from AoC's website and returns it as a string
pub fn get_puzzle_input_as_string(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let session_token = env::var("AOC_TOKEN")?;

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

// Gets the puzzle input from AoC's website and returns it as a vector of strings, split by line
pub fn get_puzzle_input_as_lines(year: u16, day:u8) -> Result<Vec<String>, Box<dyn Error>> {
    let raw_input = get_puzzle_input_as_string(year, day)?;
    
    Ok(raw_input.lines().map(String::from).collect())
}

pub fn read_from_file_as_string(filename: &str) -> String {
    fs::read_to_string(filename).expect("Error opening file")
}

// Reads input from a file and returns it as a vector of strings, split by line
pub fn read_from_file_as_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Failed"));
    }

    lines
}
