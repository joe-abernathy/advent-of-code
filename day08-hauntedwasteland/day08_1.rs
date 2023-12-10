use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::HashMap;    

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

fn main() {
    let input = get_input("./input.txt");
    let sequence = &input[0];

    let mut graph = HashMap::<&str, Vec<&str>>::new();

    // Parse the graph from the input lines
    for (i, line) in input[2..].iter().enumerate() {
        let l: Vec<_> = line.split('=').collect();
        let current = l[0].trim();
        let mut dir: Vec<_> = l[1].trim().split(',').collect();
        dir[0] = &dir[0][1..];
        dir[1] = &dir[1].trim();
        dir[1] = &dir[1][..dir[1].len() - 1];

        graph.insert(current, dir);
    }

    let mut steps = 0;
    let mut node = START_NODE;
    
    // Iterate through the graph until we reach the end node
    'end: while true {
        for ch in sequence.chars() {
            steps += 1;

            match ch {
                'L' => node = graph.get(node).unwrap()[0],
                'R' => node = graph.get(node).unwrap()[1],
                _ => continue,
            };
            if node == END_NODE {
                break 'end;
            }
        }
    }

    println!("{}", steps);
}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}