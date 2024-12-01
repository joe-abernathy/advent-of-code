use aoc_helpers::{ * };
use std::error::Error;

// Normally I'd try to make this a lot more polished than this, but I actually had a handle on the
// problem within a few minutes of it dropping, so I was focused on speed rather than polish. I'll
// clean this up later, but the important thing is that it works, okay??

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 1)?;
    let (mut left, mut right) = get_vecs_from_input(input);
    
    // Sort the two vectors
    // Probably unnecessary since this was plenty fast enough using stable sort, but optimizing a 
    // bit using unstable sort since we don't care about the order of identical numbers
    left.sort_unstable();
    right.sort_unstable();

    // Found a more efficient way of summing the differences between each vector entry than the
    // way I was doing it with a for loop or whatever
    let total: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("{}", total);

    Ok(())
}

// Converts our original vector of strings to a pair of vectors of ints (left and right)
fn get_vecs_from_input(input: Vec<String>) -> (Vec<u32>, Vec<u32>) {

    // Used some Rust magic to make this more efficient than my previous method of iterating
    // across each line, splitting by whitespace, and pushing to a couple of vectors.
    input.iter().map(|line| {
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap().parse::<u32>().expect("Failed to parse string to int (left)"),
            parts.next().unwrap().parse::<u32>().expect("Failed to parse string to int (right)"),
        )
    }).unzip()
}
