use aoc_helpers::{ * };
use regex::Regex;
use std::error::Error;
use lazy_static::lazy_static;

// This one was pretty straightforward once I actually figured out how Rust does regex. 
// We iterate over the regex matches in order from the beginning of the string to the end, 
// using a bool to determine whether multiplication is enabled. If it's not, ignore any
// mul() matches.

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 3)?;

    let result = find_matches(input)?;
    println!("{}", result);
    Ok(())
}

fn find_matches(input: Vec<String>) -> Result<u32, Box<dyn Error>> {

    // The regex adds "do()" and "don't()" as match strings
    lazy_static!(
        static ref RE: Regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3})\)|(do\(\))|(don\'t\(\))").unwrap();
    );
    
    let mut total: u32 = 0;
    let mut enabled: bool = true;

    for line in input {
        let ans: Vec<_> = RE.find_iter(&line).collect();

        for cap in ans {
            match cap.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                
                _ => {
                    if enabled {
                        total += parse_and_multiply(cap.as_str())?;
                    }
                }
            }
        }
    }
    Ok(total)
}

fn parse_and_multiply(s: &str) -> Result<u32, Box<dyn Error>> {
    let substr: String = s.chars().skip(4).take(s.chars().count() - 5).collect();
    let mut split = substr.split(',');

    let x = split.next().ok_or("Missing first number")?.trim().parse::<u32>()?;
    let y = split.next().ok_or("Missing second number")?.trim().parse::<u32>()?;

    Ok(x * y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example2.txt");
        let result = find_matches(input)?;
        
        Ok(assert_eq!(result, 48))
    }
}