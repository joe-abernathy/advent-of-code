use aoc_helpers::{ * };
use std::error::Error;
use std::collections::HashMap;
use std::time::Instant;

// This part adds the requirement to track a state (have we seen "dac" and "fft"?), and seems to add a ton more possible paths to traverse. The state
// thing was pretty simple, just track whether we've seen either of those nodes in the recursion. The extra paths required doing some memoization to
// limit the search area.

fn main() -> Result<(), Box<dyn Error>> {
    let graph = parse_input()?;
    let mut seen: HashMap<(String, bool, bool), u64> = HashMap::new();

    let now = Instant::now();
    let result = traverse_graph(&graph, &mut seen);

    println!("Result: {}\nExecution time: {:?}", result, now.elapsed());

    Ok(())
}

// Start the graph traversal
fn traverse_graph<'a>(graph: &'a HashMap<String, Vec<String>>, seen: &mut HashMap<(String, bool, bool), u64>) -> u64 {
    next_level(graph, seen, "svr".to_string(), false, false)
}

fn next_level<'a>(graph: &'a HashMap<String, Vec<String>>, seen: &mut HashMap<(String, bool, bool), u64>, root: String, prev_dac: bool, prev_fft: bool) -> u64 {
    
    // If we've reached the final output, return 1 to record a path being found if we've hit "dac" and "fft". If we haven't hit both of those, reject it
    if root == "out" {
        if prev_dac && prev_fft {
            return 1;
        }

        return 0;
    }

    // Check whether we've reached the two tracked nodes
    let fft = prev_fft || root == "fft";
    let dac = prev_dac || root == "dac";

    // If we've already seen this node with these DAC/FFT states, return the saved path count from here
    if let Some(paths) = seen.get(&(root.clone(), dac, fft)) {
        return *paths;
    }

    let mut count = 0u64;

    // If there are additional outputs from here, continue recursing
    if let Some(next_outputs) = graph.get(&root) {
        for output in next_outputs {
            count += next_level(graph, seen, output.clone(), dac, fft);
        }

    // If there are no more outputs from here, this is a leaf, so return 0
    } else {
        return 0;
    }

    // Cache the number of counts from this node with these DAC/FFT states
    seen.insert((root, dac, fft), count);

    count
}

// Get the puzzle (or example) input and parse it into a hashmap to form the graph
fn parse_input() -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example2.txt")
    } else {
        get_puzzle_input_as_lines(2025, 11)?
    };

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let spl: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
        let device = spl[0].clone();

        let outputs: Vec<String> = spl[1].split_ascii_whitespace().map(|s| s.to_string()).collect();

        graph.insert(device, outputs);
    }

    Ok(graph)
}