use aoc_helpers::{ * };
use std::error::Error;
use std::collections::HashMap;

// Part 2 was a bit tougher, but not as bad as I was expecting once I started working on the problem. Instead of splitting
// the numbers in half, I checked the first n digits in the number, where n is a factor of the overall length, to see if the
// entire number is made up of repeats of those digits. For example, for a 10-digit number, with factors 10, 5, 2, and 1, I
// checked the first 5 digits, then the first 2, then the first 1. If any of those substrings was repeated throughout the
// whole number, stop parsing and add the original number to the running count.  

fn main() -> Result<(), Box<dyn Error>> {
    let input_str = get_puzzle_input_as_string(2025, 2)?;
    let pairs = parse_input(input_str);

    println!("{}", count_invalid(pairs));
    Ok(())
}

fn count_invalid(pairs: Vec<(u64, u64)>) -> u64 {
    let mut count: u64 = 0;
    
    // Hashmap to hold the factors for each possible number of digits in the puzzle input (excluding the number
    // itself, since there's no need to check that)
    let factors: HashMap<usize, Vec<usize>> = HashMap::from([
        (1, vec![]),
        (2, vec![1]),
        (3, vec![1]),
        (4, vec![2, 1]),
        (5, vec![1]),
        (6, vec![3, 2, 1]),
        (7, vec![1]),
        (8, vec![4, 2, 1]),
        (9, vec![3, 1]),
        (10, vec![5, 2, 1])
    ]);

    for (start, end) in pairs {
        // Iterate through all numbers between each set of pairs
        for num in start..=end {
            
            // Convert each number into a string so we can work with its digits more easily
            let num_str = num.to_string();

            // Get the number of digits in each number, as well as that number's factors
            let len = num_str.len();

            // Iterate over the relevant factors for the current number's length from the factors hashmap
            for factor in factors.get(&len).unwrap() {

                // Get the number of times we need to repeat this substring to match the number of digits in the original number,
                // then repeat the substring that many times and compare it with the number
                let repeats = (len / factor) as usize;
                let spl = num_str.split_at(*factor).0;

                // If they match, this entry is invalid, so add it to the running count
                if num_str == spl.repeat(repeats) {
                    count += num;
                    break;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input_str = read_from_file_as_string("example.txt");
        let pairs = parse_input(input_str);

        let invalid = count_invalid(pairs);
        Ok(assert_eq!(invalid, 4174379265))
    }
}