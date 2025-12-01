use aoc_helpers::{ * };
use std::error::Error;

// Custom enum for handling directions. Overkill? Probably, but here we are.
#[derive(Debug)]
enum Dir { L, R }

// Implementing TryFrom to allow conversion from char ('L' or 'R') to Dir
impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'L' => Ok(Dir::L),
            'R' => Ok(Dir::R),
            _ => Err(()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_lines = get_puzzle_input_as_lines(2025, 1)?;
    let turns = parse_input(input_lines);

    println!("{}", count_zeros(turns));

    Ok(())
}

fn parse_input(input: Vec<String>) -> Vec<(Dir, u32)> {
    input.iter()
         .map(|line| {
            let (dir, ticks) = line.split_at(1);
            (Dir::try_from(dir.chars().next().unwrap()).expect("Invalid direction character"), ticks.parse::<u32>().unwrap())
         })
         .collect()
}

fn count_zeros(input: Vec<(Dir, u32)>) -> u32 {
    let mut dial: i32 = 50;
    let mut zero_count: u32 = 0;

    for (dir, ticks) in input {
        
        // Either add or subtract the current number of ticks to dial, and do some modulo math to keep the result between 0 and 99
        match dir {
            Dir::L => dial = (((dial - ticks as i32) % 100) + 100) % 100,
            Dir::R => dial = (dial + ticks as i32) % 100,
        }

        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input_lines = read_from_file_as_lines("example1.txt");
        let turns = parse_input(input_lines);

        let zeros = count_zeros(turns);
        Ok(assert_eq!(zeros, 3))
    }
}