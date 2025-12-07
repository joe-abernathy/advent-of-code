use aoc_helpers::{ * };
use std::error::Error;
use std::time::Instant;

// This one was a wild ride. I completely misunderstood the format at first. My misunderstanding worked
// fine on part 1, but part 2 screwed me. I had to start from scratch when I realized it was about the
// specific column the character was in and not the digit position within the number. But the important
// thing is that we got there.

fn main() -> Result<(), Box<dyn Error>> {
    let (numbers, operators) = parse_input()?;

    // Take the sum or product of the numbers in each column and then sum them together
    let result: u64 = operators
        .iter()
        .zip(numbers.iter())
        .map(|(op, nums)| -> u64 {
            match *op {
                '+' => nums.iter().map(|&n| n as u64).sum(),
                '*' => nums.iter().map(|&n| n as u64).product(),
                _ => panic!("{}", format!("Invalid operator: {}", *op as char)),
            }
        })
        .sum();

    let now = Instant::now();
    println!("Result: {}, execution time: {:?}", result, now.elapsed());

    Ok(())
}

// Parses puzzle input from a vector of strings into a vector of vectors of u32s
fn parse_input() -> Result<(Vec<Vec<u32>>, Vec<char>), Box<dyn Error>> {
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 6)?
    };

    // The puzzle says numbers are delineated by a column of whitespace all the way down, so
    // we find those whitespace columns and add their indices to a list.
    let separators: Vec<usize> = (0..input[0].len())
        .filter(|&i| input.iter().all(|s| s.as_bytes()[i].is_ascii_whitespace()))
        .collect();

    // Splits the strings at the indices we just calculated and collect them into a vector of
    // vectors. Because we're looking at it column-wise, we'll need to transpose this vector
    // in order to get the numbers we need. We keep the numbers as strings here so we can retain
    // the whitespace for figuring out the alignment.
    let n_strings_transposed: Vec<Vec<String>> = input[0..input.len() - 1]
        .iter()
        .map(|s| split_mult(s.as_str(), &separators)
            .iter()
            .map(|s| s.to_string())
            .collect()
        )
        .collect();

    // Transpose that vector we just created
    let n_strings = transpose(&n_strings_transposed);

    // Get the operators for each column by grabbing the last row in the input
    let operators: Vec<char> = input[input.len() - 1].split_whitespace().map(|s| s.chars().nth(0).unwrap()).collect();
    
    // Convert to the cephalapod notation from part 2
    let numbers = convert_to_cephalopod(&n_strings);

    Ok((numbers, operators))
}

// This takes the 2D vector of strings (with whitespace retained), calculates alignment,
// and parses the numbers read column-wise into ints
fn convert_to_cephalopod(n_strings: &Vec<Vec<String>>) -> Vec<Vec<u32>> {
    n_strings
        .iter()
        .map(|group| {
            (0..group[0].len())
                .map(|i| {
                    let mut num: String = String::new();
                    for s in group {
                        let ch = s.as_bytes()[i] as char;
                        if !ch.is_whitespace() {
                            num.push(ch);
                            }
                    }
                    num.parse::<u32>().expect("Failed to parse string to int")
                })
            .collect()
            })
            .collect()
}

// Transpose a 2D vector
fn transpose(input: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let rows = input.len();
    let cols = input.first().map_or(0, |r| r.len());

    let mut result = vec![Vec::with_capacity(rows); cols];

    for row in input {
        for (i, val) in row.iter().enumerate() {
            result[i].push(val.clone());
        }
    }

    result
}

fn split_mult<'a>(s: &'a str, separators: &[usize]) -> Vec<&'a str> {
    let mut parts = Vec::new();
    let mut prev = 0;

    for &i in separators {
        parts.push(&s[prev..i]);
        prev = i + 1;
    }

    parts.push(&s[prev..]);
    parts
}