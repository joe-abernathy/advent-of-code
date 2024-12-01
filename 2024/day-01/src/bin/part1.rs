use aoc_helpers::{ * };
use std::error::Error;

// Normally I'd try to make this a lot more polished than this, but I actually had a handle on the
// problem within a few minutes of it dropping, so I was focused on speed rather than polish. I'll
// clean this up later, but the important thing is that it works, okay??

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 1)?;
    let (mut left, mut right) = get_vecs_from_input(input);
    
    // Sort the two vectors
    left.sort();
    right.sort();

    // Running total of difference values
    let mut total: u32 = 0;

    // Iterate across each entry in both vectors and get the absolute value of the difference
    // between the two, then add it to the running total
    for i in 0..left.len() {
        total += left[i].abs_diff(right[i]);
    }

    println!("{}", total);

    Ok(())
}

// Converts our original vector of strings to a pair of vectors of ints (left and right)
fn get_vecs_from_input(input: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input {
        let split: Vec<&str> = line.split("   ").collect();
    
        left.push(split[0].parse::<u32>().expect("Failed to parse string to u32"));
        right.push(split[1].parse::<u32>().expect("Failed to parse string to u32"));
    }

    (left, right)
}
