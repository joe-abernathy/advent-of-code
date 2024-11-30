use aoc_helpers::{ * };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = get_puzzle_input_as_lines(2022, 3)?;
    let result = calculate_total(lines);
    
    println!("{}", result);

    Ok(())
}

fn calculate_total(lines: Vec<String>) -> u32 {
    let mut running_total = 0;

    // Iterate over all lines in the input vector, separate each line into a separate rucksack,
    // find the duplicate character, calculate its priority, and add it to the running total
    for line in lines {
        let rucksack = get_rucksack_from_lines(&line);
        let dup = find_duplicate_item(rucksack.0, rucksack.1);
        running_total += char_to_priority(dup.expect("Duplicate item returned a None value"));
    }

    running_total
}

// Split a line into a rucksack with two equally sized compartments (substrings)
fn get_rucksack_from_lines(line: &str) -> (&str, &str) {
    let mid = line.len() / 2;
    (&line[..mid], &line[mid..])
}

// Return the duplicate character across two strings, or None if one isn't found
fn find_duplicate_item(entry1: &str, entry2: &str) -> Option<char> {
    for ch in entry1.chars() {
        if entry2.contains(ch) {
            return Some(ch);
        }
    }

    None
}

// Calculate a given character's priority
fn char_to_priority(ch: char) -> u32 {

    // If the character is lowercase, we need to subtract 96 from its ASCII value to get the required
    // value. If it's uppercase, subtract 38.
    match ch {
        'a'..='z' => (ch as u32) - ('a' as u32) + 1,
        'A'..='Z' => (ch as u32) - ('A' as u32) + 27,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = read_from_file_as_lines("example.txt");
        let result = calculate_total(lines);

        assert_eq!(result, 157)
    }
}