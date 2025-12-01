use aoc_helpers::{ * };
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let input_lines = get_puzzle_input_as_lines(2025, 1)?;
    let turns = parse_input(input_lines);

    println!("{}", count_zeros(turns));

    Ok(())
}

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

fn count_zeros(input: Vec<i32>) -> u32 {
    let mut dial: i32 = 50;
    let mut zero_count: u32 = 0;

    for ticks in input {
        dial += ticks;

        if dial > 99 {
            dial %= 100;

        } else if dial < 0 {
            dial = ((dial % 100) + 100) % 100
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