use aoc_helpers::{ * };
use std::error::Error;
use itertools::Itertools;

// This one took some research. Started with a brute force solution, iterating across all possible
// permutations of '*' and '+' for each pair of numbers. I ended up optimizing a bit by pruning any
// branches that exceed the result. Still seems like this would be pretty inefficient, but it runs
// essentially instantly on the puzzle input, so I'll take it.

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 7)?;
    let in_vec = parse_input(input);

    let result = in_vec
        .iter()
        .fold(0, |acc, (target, nums)| {
            if let Some(res) = evaluate(*target, nums) {
                acc + res
            } else {
                acc
            }
        });

    println!("{}", result);

    Ok(())
}

fn evaluate(target: u64, nums: &[u64]) -> Option<u64> {
    let operators = vec!['+', '*'];
    let permutations = std::iter::repeat(&operators)
        .take(nums.len() - 1)
        .multi_cartesian_product();

    for perm in permutations {
        let mut result = nums[0];

        for (op, &num) in perm.iter().zip(&nums[1..]) {
            match op {
                '+' => result += num,
                '*' => result *= num,
                _ => unreachable!(),
            }

            if result > target {
                break;
            }
        }

        if result == target {
            return Some(target);
        }
    }
    None

}

fn parse_input(input: Vec<String>) -> Vec<(u64, Vec<u64>)> {
    input
        .iter()
        .map(|s| {
            let (left, right) = s.split_once(':').expect("Input format is invalid");
            let left = left.trim().parse::<u64>().expect("Failed to parse left side string to int");
            let right = right
                .split_whitespace()
                .map(|s| s.parse::<u64>().expect("Failed to parse right side string to int"))
                .collect();
            (left, right)
        })
        .collect()
}