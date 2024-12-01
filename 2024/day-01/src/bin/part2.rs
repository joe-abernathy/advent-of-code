use aoc_helpers::{ * };
use std::error::Error;

// Normally I'd try to make this a lot more polished than this, but I actually had a handle on the
// problem within a few minutes of it dropping, so I was focused on speed rather than polish. I'll
// clean this up later, but the important thing is that it works, okay??

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 1)?;
    let (left, right) = get_vecs_from_input(input);
    
    // Running total
    let mut total = 0;

    // For each entry in the left vector, count the number of times that value appears in the right 
    // vector and then add it to the running total

    // There's definitely a better way to do this, but that's gonna take some more Rust research,
    // and honestly I don't care at this point. It works, shut up.
    for i in 0..left.len() {
        let current_count = right
            .iter()
            .filter(|&&x| x == left[i])
            .count();

        total += left[i] * current_count as u32;
    }

    println!("{}", total);

    Ok(())
}

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
