use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{ HashSet, VecDeque };

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_string(2022, 6)?;
    
    let result = parse_input(input);
    println!("{}", result.expect("Returned a None value"));
    Ok(())
}

fn parse_input(input: String) -> Option<usize> {

    // Initialize a VecDeque to act as a buffer for parsing 4 chars at a time
    let mut buffer: VecDeque<char> = VecDeque::new();
    for (i, ch) in input.chars().enumerate() {

        // Push chars to fill the buffer initially
        if buffer.len() < 4 {
            buffer.push_back(ch);

        } else {

            // Check if all elements in the buffer are unique (i.e., collect them into a
            // HashSet and check the length of the set against the length of the buffer).
            // If so, we've reached the start-of-packet marker, so return the current index.
            if buffer.iter().cloned().collect::<HashSet<_>>().len() == buffer.len() {
                return Some(i);
            }

            // Otherwise, drop the first char in the buffer and push the next char in the string      
            buffer.pop_front();
            buffer.push_back(ch);          

        }
    }

    // Return None if no start-of-packet marker is found in the string
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() -> Result<(), Box<dyn Error>> {
        let test_cases = vec![
            ("example1.txt", 7),
            ("example2.txt", 5),
            ("example3.txt", 6),
            ("example4.txt", 10),
            ("example5.txt", 11),
        ];

        for (filename, expected) in test_cases {
            let input = read_from_file_as_string(filename);
            let result = parse_input(input).expect("Returned a None value");

            assert_eq!(result, expected, "Test failed for file {}", filename);
        }

        Ok(())
    }
}