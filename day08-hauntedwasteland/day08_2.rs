use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::HashMap;    
use num::Integer;

fn main() {
    let input = get_input("./input.txt");
    let sequence = &input[0];
    
    let mut graph = HashMap::<&str, Vec<&str>>::new();

    // Build the graph from the input
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
    let mut paths: Vec<&str> = vec![];
    let mut start_nodes: Vec<&str> = vec![];

    let mut first_z: Vec<u64> = vec![];

    // Get the start nodes by finding all nodes ending in A
    for node in graph.keys() {
        if node.chars().nth(2).unwrap() == 'A' {
            paths.push(node);
            start_nodes.push(node);
        }
    }

    for i in 0..paths.len() {

        let mut fast = sequence.chars().cycle();
        let mut slow = sequence.chars().cycle();

        let mut pos: &str = start_nodes[i];
        let mut steps = 0;

        // Iterate through the loop to find the number of steps before a Z-node is reached
        'end: loop {
            for ch in sequence.chars() {
                steps += 1;
                match ch {
                    'L' => pos = graph.get(pos).unwrap()[0],
                    'R' => pos = graph.get(pos).unwrap()[1],
                    _ => continue,
                };

                // If the current node ends in Z, record the step count and break out of the loop
                if pos.chars().nth(2).unwrap() == 'Z' {
                    first_z.push(steps);
                    break 'end;
                }
            }
        }
    }

    // Find the LCM of the number of steps required for each path to reach a Z-node to find
    // the step count when all paths will reach Z-nodes at the same time
    let lcm = first_z.iter().fold(1, |acc, &x| acc.lcm(&x));
    println!("{}", lcm);
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}