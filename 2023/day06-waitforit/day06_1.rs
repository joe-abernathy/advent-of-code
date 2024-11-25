use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let input: Vec<_> = get_input("./input.txt");
    
    let mut time_str: Vec<_> = input[0].split(':').collect();
    time_str = time_str[1].split_whitespace().collect();
    let times: Vec<u32> = time_str.iter().map(|s| s.parse::<u32>().unwrap()).collect();

    let mut dist_str: Vec<_> = input[1].split(':').collect();
    dist_str = dist_str[1].split_whitespace().collect();
    let dists: Vec<u32> = dist_str.iter().map(|s| s.parse::<u32>().unwrap()).collect();

    let mut result = 1;

    for (i, time) in times.iter().enumerate() {
        let mut total = 0;

        for holdtime in 1..*time {
            if holdtime * (time - holdtime) > dists[i] {
                total += 1;
            }
        }
        result *= total;
    }

    println!("{}", result);
}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File doesn't exist");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}