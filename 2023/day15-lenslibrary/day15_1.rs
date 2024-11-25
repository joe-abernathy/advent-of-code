use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

fn main() {
    let input = get_input("./input.txt");

    let total: usize = input.iter().map(|step| hash(step)).sum();
    println!("Total: {}", total);
}


fn hash(step: &[u8]) -> usize {
    step.iter().fold(0, |acc, &ch| (acc + ch as usize) * 17 % 256)
}


// Get the input lines from the input file
fn get_input(filename: impl AsRef<Path>) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.split(b',').map(|s| s.expect("Failed to parse string")).collect()
}