use aoc_helpers::{ * };
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let (numbers, operators) = parse_input()?;

    let now = Instant::now();

    let result: u64 = operators
        .iter()
        .zip(numbers.iter())
        .map(|(op, nums)| -> u64 {
            match *op {
                '+' => nums.iter().sum(),
                '*' => nums.iter().product(),
                _ => panic!("Invalid operator"),
            }
        })
        .sum();

    println!("Result: {}, execution time: {:?}", result, now.elapsed());
        
    Ok(())
}

fn parse_input() -> Result<(Vec<Vec<u64>>, Vec<char>), Box<dyn Error>> {
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 6)?
    };

    let num_str = &input[..input.len() - 1];
    let op_str = &input[input.len() - 1];

    let numbers_transposed: Vec<Vec<u64>> = num_str
        .iter()
        .map(|s| s.split_whitespace().map(|s| s.parse::<u64>().expect("Failed to parse string to int")).collect())
        .collect();

    let numbers = transpose(&numbers_transposed);
    let operators: Vec<char> = op_str.split_whitespace().map(|s| s.chars().next().unwrap()).collect();

    Ok((numbers, operators))
}

fn transpose(input: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let rows = input.len();
    let cols = input.first().map_or(0, |r| r.len());

    let mut result = vec![Vec::with_capacity(rows); cols];

    for row in input {
        for (i, val) in row.iter().enumerate() {
            result[i].push(*val);
        }
    }

    result
}
