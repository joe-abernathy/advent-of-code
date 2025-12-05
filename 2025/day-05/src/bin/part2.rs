use aoc_helpers::{ * };
use core::num;
use std::error::Error;
use std::time::Instant;

// Merging the ranges in part 1 really paid off here. Just a matter of summing the number of IDs in each range. The bulk of this code is for
// reading in and parsing the ranges, the actual counting is basically a one-liner.

fn main() -> Result<(), Box<dyn Error>> {
    let ranges = parse_input()?;

    let now = Instant::now();
    let result = count_fresh(&ranges);
    
    println!("Result: {}, execution time: {:?}", result, now.elapsed());

    Ok(())
}

// Count the number of IDs considered fresh by summing the total number of IDs in each range. Just (end - start + 1), since the ranges are inclusive
fn count_fresh(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

fn parse_input() -> Result<Vec<(u64, u64)>, Box<dyn Error>> {
    
    // Allows me to run this on the sample input rather than the real puzzle input by adding '--features test' to the cargo run command
    let input = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 5)?
    };

    // Split the input vector by the empty line that marks the move from ranges to numbers
    let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();
    
    Ok(parse_ranges(sections[0]))
}

// Takes the vector of range strings and parses it into a vector of u64s with the form (start, end). This also sorts the ranges
// and merges any overlapping vectors to hopefully make this whole thing a little more efficient.
fn parse_ranges(input: &[String]) -> Vec<(u64, u64)> {
    let mut ranges: Vec<_> = input
        .iter()
        .map(|s| s.split_once('-').unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    ranges.sort_by(|(a, _), (b, _)| a.cmp(&b));

    let mut merged: Vec<(u64, u64)> = vec![ranges[0]];

    for current_range in &ranges[1..] {
        let last = merged.len() - 1;

        if current_range.0 <= merged[last].1 {
            if current_range.1 < merged[last].1 {
                continue;
            } else {
                merged[last] = (merged[last].0, current_range.1);
            }
        } else {
            merged.push(*current_range);
        }
    }

    merged
}
