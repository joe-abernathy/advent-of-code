use aoc_helpers::{ * };
use std::error::Error;
use std::time::Instant;

// My approach here was pretty straightforward: just check each given number against the given ranges to see if it was included in any.
// I decided to sort and merge the given ranges here to make computation a bit less expensive, especially because I figured part 2 would
// require more expensive computations and might make it infeasible without a bunch of optimization. That ended up not being the case,
// but merging the ranges here still made part 2 wildly easy.   

fn main() -> Result<(), Box<dyn Error>> {
    let (ranges, numbers) = parse_input()?;

    let now = Instant::now();
    let result = check_values(&ranges, &numbers);
    
    println!("Result: {}, execution time: {:?}", result, now.elapsed());

    Ok(())
}

// Counts the number of fresh ingredients by checking each number against the given ranges of values. This runs in about 2ms on my end,
// but I imagine part 2 is gonna throw some curveball that makes this approach not feasible. But we'll see.
// **UPDATE: nope, part 2 was way easier. The only curveball was that the given ranges have some overlap, which wasn't an issue since I
// merged the ranges in this part. Winning.
fn check_values(ranges: &Vec<(u64, u64)>, numbers: &Vec<u64>) -> u32 {
    numbers
        .iter()
        .filter(|n| ranges.iter().any(|(start, end)| (start ..= end).contains(n)))
        .count().try_into().unwrap()
}

fn parse_input() -> Result<(Vec<(u64, u64)>, Vec<u64>), Box<dyn Error>> {
    
    // Allows me to run this on the sample input rather than the real puzzle input by adding '--features test' to the cargo run command.
    // Game changer.
    let input = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 5)?
    };

    // Split the input vector by the empty line that marks the move from ranges to numbers
    let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();
    
    let ranges = parse_ranges(sections[0]);
    let numbers = parse_numbers(sections[1]);

    Ok((ranges, numbers))
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

// Parses the number strings into u64s
fn parse_numbers(input: &[String]) -> Vec<u64> {
    input
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}