use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {

    // Parse the input
    let input_str = get_input("./input.txt");
    let mut input: Vec<Vec<i32>> = vec![];

    for line in input_str {
        let num: Vec<_> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        input.push(num);
    }

    let mut total = 0;

    // Parse the current history into a list of sequences
    for hist in input {
        let mut predictions: Vec<Vec<i32>> = vec![hist.clone()];
        let mut current: Vec<i32> = hist.clone();

        while !current.iter().all(|&x| x == 0) {
            let mut next: Vec<i32> = vec![];

            for (i, entry) in current[..current.len() - 1].iter().enumerate() {
                next.push(current[i + 1] - entry);
            }
            predictions.push(next.clone());
            current = next;
        }
        
        // Iterate through the sequences backwards and calculate the addend for each
        // sequence by adding the previous addend to the last entry in the current line
        let mut addends: Vec<i32> = vec![0];
        for line in predictions[..predictions.len() - 1].iter().rev() {
            let addend = line.last().unwrap() + addends.last().unwrap();
            addends.push(addend);
        }
        
        // Add the addend from the original history to the running total
        total += addends.last().unwrap();        
    }

    println!("{}", total);
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}