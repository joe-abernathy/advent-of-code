use aoc_helpers::{ * };
use std::error::Error;

// My approach here was to convert each number to a string so I could easily split it into two parts. I ignore
// any numbers with an odd number of digits, since by definition those can't fit the pattern of having being 
// made up of two identical numbers with no leading zeros. If the number has an even number of digits, split it
// at the midpoint and compare the two slices. If they match, add the original number to a running counter.

// Now let's see how bad part 2 screws me up.

fn main() -> Result<(), Box<dyn Error>> {
    let input_str = get_puzzle_input_as_string(2025, 2)?;
    let pairs = parse_input(input_str);

    println!("{}", count_invalid(pairs));
    Ok(())
}

fn count_invalid(pairs: Vec<(u64, u64)>) -> u64 {
    let mut count: u64 = 0;
    
    for (start, end) in pairs {
        for n in start..=end {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let mid = s.len() / 2 as usize;
                let (a, b) = s.split_at(mid);

                if a == b {
                    count += n;
                }
            }
        }
    }

    count
}

fn parse_input(input: String) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|pair| {
            let (a, b) = pair.split_once('-').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect()
}