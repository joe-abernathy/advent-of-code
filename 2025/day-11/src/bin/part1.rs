use aoc_helpers::{ * };
use std::error::Error;
use std::collections::HashMap;
use std::time::Instant;

// This was a nice little break after the hell that was day 10. Basically a recursive graph traversal problem.

fn main() -> Result<(), Box<dyn Error>> {
    let graph = parse_input()?;

    let now = Instant::now();
    let result = traverse_graph(&graph);

    println!("Result: {}\nExecution time: {:?}", result, now.elapsed());

    Ok(())
}

// Start the graph traversal
fn traverse_graph(graph: &HashMap<String, Vec<String>>) -> u32 {
    let outputs = graph.get("you").unwrap();
    
    let mut count = 0u32;

    for output in outputs {
        count += next_level(graph, output);
    }

    count
}

// Recursively check the next level in the graph
fn next_level(graph: &HashMap<String, Vec<String>>, root: &String) -> u32 {
    
    // If we've reached the final output, return 1 to record a path being found
    if root == "out" {
        return 1;
    }

    // Record the number of paths found from this level
    let mut count = 0u32;

    // If there are additional outputs from here, continue recursing
    if let Some(next_outputs) = graph.get(root) {
        for output in next_outputs {
            count += next_level(graph, output);
        }

    // If there are no more outputs from here, this is a leaf, so return 0
    } else {
        return 0;
    }

    count
}

// Get the puzzle (or example) input and parse it into a hashmap to form the graph
fn parse_input() -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 11)?
    };

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let mut spl: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
        let device = spl[0].clone();

        let outputs: Vec<String> = spl[1].split_ascii_whitespace().map(|s| s.to_string()).collect();

        graph.insert(device, outputs);
    }

    Ok(graph)
}