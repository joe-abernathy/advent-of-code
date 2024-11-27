use aoc_helpers::{ * };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = get_puzzle_input_as_lines(2022, 3)?;
    let groups = get_groups_from_lines(lines);

    let result = calculate_total(groups);
    println!("{}", result);

    Ok(())
}

// Iterates through all groups of 3 in the input, finds the duplicate value in each, and 
// adds its priority to a running total. Then returns the total.
fn calculate_total(groups: Vec<Vec<String>>) -> u32 {
    let mut running_total = 0;

    for group in groups {
        let dup = find_duplicate_item(group);
        running_total += char_to_priority(dup.expect("No duplicate items :("));
    }

    running_total
}

// Returns the duplicate character that exists in all three strings in the group, or None if none exists
fn find_duplicate_item(group: Vec<String>) -> Option<char> {
    for ch in group[0].chars() {
        if group[1].contains(ch) && group[2].contains(ch) {
            return Some(ch);
        }
    }
    
    None
}

// Split the puzzle input (as lines) into groups of 3
fn get_groups_from_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    lines
    .chunks(3)
    .map(|chunk| chunk.to_vec())
    .collect()
}

// Calculate a given character's priority
fn char_to_priority(ch: char) -> u32 {
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
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let lines = read_from_file("example.txt");
        let groups = get_groups_from_lines(lines);
        let result = calculate_total(groups);

        Ok(assert_eq!(result, 70))
    }
}