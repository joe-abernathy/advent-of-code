use aoc_helpers::{ * };
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 5)?;
    let (rules, pages) = process_input(input);

    let result = count_correct_pages(rules, pages);

    println!("{}", result);
    Ok(())
}

// I'm slowly getting the hang of iterators in Rust. Pretty impressed tbh. 
fn count_correct_pages(rules: Vec<(u32, u32)>, pages: Vec<Vec<u32>>) -> u32 {

    // Iterate over each set of pages in the list, then iterate over each rule in
    // the rules list. If both entries in the current rule exist in the current 
    // set of pages, check their order within the current set, and if it's correctly
    // sorted, add the middle element from that vector to an accumulator that Rust
    // so conveniently provides for us. Then return the accumulator.
    pages
        .into_iter()
        .filter(|page| {
            rules
                .iter()
                .filter(|(x, y)| page.contains(x) && page.contains(y))
                .all(|(x, y)| check_order(page.clone(), *x, *y))
        })
        .map(|page| {
            let middle = page.len() / 2;
            page[middle]
        })
        .sum()
}

// Checks if two entries in a page list are correctly sorted. This should get passed the
// current page list, and an x and y corresponding to a specific rule from the rule list,
// where x is the first number and y is the second, indicating that y should come after
// x in the page list.
fn check_order(page: Vec<u32>, x: u32, y: u32) -> bool {
    let x_pos = page.iter().position(|n| *n == x).unwrap();
    let y_pos = page.iter().position(|n| *n == y).unwrap();

    y_pos > x_pos
}

// Process puzzle input into the appropriate rules and pages vectors
fn process_input(input: Vec<String>) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let split_index = input.iter().position(|s| s.is_empty()).unwrap();
    let (rules, pages) = input.split_at(split_index);

    let r: Vec<(u32, u32)> = rules
        .iter()
        .map(|s| {
            let mut split = s.split('|').into_iter();
            let x = split.next().unwrap().parse::<u32>().expect("Failed to parse string to int");
            let y = split.next().unwrap().parse::<u32>().expect("Failed to parse string to int");
            (x, y)
        })
        .collect();

    let p: Vec<Vec<u32>> = pages
        .iter()
        .skip(1)
        .map(|s| {
            let split = s.split(',').into_iter();
            split.map(|n| n.parse::<u32>().expect("Failed to parse string to int"))}
            .collect())
        .collect();

    (r, p)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example.txt");
        let (rules, pages) = process_input(input);

        let result = count_correct_pages(rules, pages);
        Ok(assert_eq!(result, 143))
    }
}