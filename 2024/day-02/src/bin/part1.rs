use aoc_helpers::{ * };
use std::error::Error;

// Rust is wild, ya know? But I'm slowly starting to get the hang of this iter() nonsense, and it's
// actually kinda cool.

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 2)?;
    let reports = parse_lines_to_vecs(input);

    let result = count_safe_reports(reports);
    println!("{}", result);
    
    Ok(())
}

// Count the number of reports in the report vector that meet the criteria of being "safe"
fn count_safe_reports(reports: Vec<Vec<i8>>) -> u32 {

    // Iterate over the elements in the outer vector
    reports
        .iter()

        // Look at each slice of 2 entries in the current inner vec, checking
        .map(|v| {

            // Get the direction (increasing, decreasing, or static) of the first pair in the vector for comparison
            let direction = (v[0] - v[1]).signum();

            // Look at each 2-entry slice in the vector to ensure that the absolute value of the difference is between
            // 1 and 3, and the direction doesn't change
            v.windows(2).all(|w| {
                let diff = w[0] - w[1];
                diff.abs() > 0 && diff.abs() < 4 
                && diff.signum() == direction
            })
        })

        // Count the reports where all those criteria are true and return the total
        .filter(|x| *x == true)
        .count() as u32
    }

// Iterate over a vector of strings, split each string by whitespace and parse each entry
// into a signed 8-bit integer, and slap them into a vector of vectors of ints.
fn parse_lines_to_vecs(input: Vec<String>) -> Vec<Vec<i8>> {
    input.iter().map(|line| {
        let parts = line.split_whitespace();
        parts
            .into_iter()
            .map(|s| s.parse::<i8>().expect("Failed to parse string to int"))
            .collect()
    })
    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = read_from_file_as_lines("example.txt");
        let reports = parse_lines_to_vecs(input);

        let result = count_safe_reports(reports);
        assert_eq!(result, 2)
    }
}