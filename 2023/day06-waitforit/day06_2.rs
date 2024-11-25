use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let input: Vec<_> = get_input("./input.txt");
    
    let mut time_str: String = input[0].split(':').skip(1).collect();
    time_str.retain(|c| !c.is_whitespace());
    let time = time_str.parse::<u64>().unwrap();

    let mut dist_str: String = input[1].split(':').skip(1).collect();
    dist_str.retain(|c| !c.is_whitespace());
    let dist = dist_str.parse::<u64>().unwrap();

    let mut total = 0;

    for holdtime in 1..time {
        if holdtime * (time - holdtime) > dist {
            total += 1;
        }
    }

    println!("{}", total);
}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File doesn't exist");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}