use aoc_helpers::{ * };
use std::error::Error;
use std::time::Instant;


fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_puzzle_input_as_lines(2025, 3)?;
    let result = parse_banks(lines);

    // Monitoring execution time
    let now = Instant::now();
    println!("Result: {}, execution time: {:?}", result, now.elapsed());

    Ok(())
}

fn parse_banks(input: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    
    input
        .iter()
        .for_each(|bank| count += get_best_joltage(bank.to_string()));

    count
}

fn get_best_joltage(bank: String) -> u32 {
    let max1 = bank.bytes().max().unwrap();
    let i = bank.bytes().position(|x| x == max1).unwrap();

    if i == bank.len() - 1 {
        let max2: char = bank[..i].bytes().max().unwrap() as char;
        return format!("{}{}", max2, max1 as char).parse::<u32>().expect("Failed to parse value to u32");
    }

    let max2: char = bank[i+1..].bytes().max().unwrap() as char;
    return format!("{}{}", max1 as char, max2).parse::<u32>().expect("Failed to parse value to u32");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example.txt");
        let result = parse_banks(input);

        Ok(assert_eq!(result, 357))
    }
}