use aoc_helpers::{ * };
use regex::Regex;
use std::error::Error;
use lazy_static::lazy_static;

// The hardest part about this one was learning how regex works. And how Rust handles it.

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 3)?;

    let result = find_matches(input)?;
    println!("{}", result);
    Ok(())
}

fn find_matches(input: Vec<String>) -> Result<u32, Box<dyn Error>> {

    lazy_static!(
        static ref RE: Regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    );
    
    let mut total: u32 = 0;

    // Iterate through the strings
    for line in input {

        // Run the regex and store the matches in a vector
        let ans: Vec<_> = RE.find_iter(&line).collect();

        // For each capture in the regex response:
        for cap in ans {

            // Get the match ("mul(xx,yy)"), get the substring to narrow it down to just the 
            // digits and comma, then split it, parse the results into u32s, and multiply them.
            let s = cap.as_str();
            let substr = &s[4..s.len() - 1];
            let mut split = substr.split(',');
            
            let x = split.next().ok_or("Missing first number")?.trim().parse::<u32>()?;
            let y = split.next().ok_or("Missing second number")?.trim().parse::<u32>()?;

            total += x * y;
        }
    }
    Ok(total)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example1.txt");
        let result = find_matches(input)?;
        
        Ok(assert_eq!(result, 161))
    }
}