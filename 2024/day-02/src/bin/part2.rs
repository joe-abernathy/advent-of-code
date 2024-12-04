use aoc_helpers::{ * };
use std::error::Error;

// Well this is alarmingly inefficient. I'm researching ways to make it more 
// efficient, but it works for now, so don't @ me.

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 2)?;
    // let input = read_from_file_as_lines("example.txt");
    let reports = parse_lines_to_vecs(input);

    let result = count_safe_reports(reports.clone());
    println!("{}", result);
    // println!("{:?}", reports);
    Ok(())
}

// Count the number of reports in the report vector that meet the criteria of being "safe"
fn count_safe_reports(reports: Vec<Vec<i8>>) -> u32 {
    let mut result: u32 = 0;

    for report in reports {

        // Here's the scary part. Check the given report without removing anything. If it's
        // safe, we're done, increment result and move on. Otherwise, iterate through all
        // entries in the report, run check_report() again removing the current entry. If 
        // it's safe, we're done, increment result and move on to the next report, otherwise 
        // keep going. Yeah, I know. 
        if !check_report(&mut report.clone(), None) {
            for i in 0..report.len() {
                if check_report(&mut report.clone(), Some(i)) {
                    result += 1;
                    break;
                }
            }
        } else {
            result += 1;
        }        
    }

    result
}

// Checks a single report with, removing 0 or 1 entries from the vector
// beforehand based on the skip parameter. 
fn check_report(report: &mut Vec<i8>, skip: Option<usize>) -> bool {
    if !skip.is_none() {
        report.remove(skip.unwrap());
    }

    let dir = (report[1] - report[0]).signum();
    if dir == 0 {
        return false;
    }

    report
        .windows(2)
        .all(|w| {
            let diff = w[1] - w[0];
            (1..4).contains(&diff.abs()) && diff.signum() == dir
    })
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
        assert_eq!(result, 4)
    }
}