use aoc_helpers::{ * };
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_lines = get_puzzle_input_as_lines(2025, 1)?;
    let turns = parse_input(input_lines);

    println!("{}", count_zeros(turns));

    Ok(())
}

// Make the moves indicated in the input and count the number of times the dial crosses 0. I'm sure there's a more efficient
// way to do this, and I may try to simplify it later, but we're here now.
fn count_zeros(input: Vec<i32>) -> u32 {
    let mut dial: i32 = 50;
    let mut zero_count: u32 = 0;

    for ticks in input {

        // This will hold the number of times the dial crossed 0 per turn
        let mut zeros: i32 = 0;

        // If we're turning the dial to the left:
        if ticks < 0 {

            // When turning left, if |ticks| is at least the current dial value, we've crossed zero at least once
            if ticks.abs() >= dial {

                // This equation gives the number of times we crossed zero
                zeros = (ticks.abs() + (100 - dial)).div_euclid(100) as i32;

                // This handles the edge case where the dial started at zero and the number of crosses needs to be reduced by 1.
                // This is only an issue when turning left
                if dial == 0 {
                    zeros -= 1;
                }

                // Increment the zero counter
                zero_count += zeros as u32;
            }

        // If we're turning the dial to the right:
        } else {

            // When turning right, if ticks is at least 100 - the current dial value, we've crossed zero at least once
            if ticks >= 100 - dial {
                zeros = (ticks + dial).div_euclid(100) as i32;
                zero_count += zeros as u32;
            }
        }

        // Increment the dial by the specified number of ticks
        dial += ticks;

        // If the new dial position is outside the range [0, 99], convert it so that it's back in that range
        if dial < 0 {
            dial = (((dial + zeros * 100) % 100) + 100) % 100;

        } else if dial > 99 {
            dial = (dial - zeros * 100) % 100;
        }
    }
    zero_count
}

// Parse the input into a vector of positive or negative integers, depending on the direction of the turn (L => negative, R => positive)
fn parse_input(input: Vec<String>) -> Vec<i32> {
    input.iter()
        .map(|line| {
            let (dir, ticks) = line.split_at(1);
            let sign = match dir.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid direction character: {}", dir),
            };
            sign * ticks.parse::<i32>().unwrap()
         })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input_lines = read_from_file_as_lines("example1.txt");
        let turns = parse_input(input_lines);

        let zeros = count_zeros(turns);
        Ok(assert_eq!(zeros, 6))
    }
}