use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

// I'm just learning all the things. Like how Rust handles custom sorting orders, and efficient ways
// to preprocess inputs to speed up execution. The custom sorting rules have a list of pairs of
// integers, (X|Y), where if both X and Y exist in a list of pages, X must appear at some point
// before Y. To make this simpler, I converted that into a hashmap with X as a key, and as a 
// value, a hashset containing all Y values that correspond with the given X in any of our rules.

// This hashset gets passed to a function that defines the custom sorting rules, such that if
// we're evaluating a specific pair of values (A, B) in a list of pages, if A exists as an X in
// the rules map and B is one of its corresponding Y values, we consider A to be less than B for
// purposes of the sort, and vice versa. If only A or B exist as an X or Y, we consider the two
// to be equal.

// Then we just run through each set of pages, filter down to the ones that aren't initially
// sorted, sort them, and sum their middle values. 

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 5)?;
    let (rules, mut pages) = process_input(input);

    let result = get_sorted_result(rules, &mut pages);
    println!("{}", result);

    Ok(())
}

fn get_sorted_result(rules: HashMap<u32, HashSet<u32>>, pages: &mut Vec<Vec<u32>>) -> u32 {

    // Iterate over each set of pages, getting only the ones that are not initially sorting using
    // our custom ordering rules
    pages.iter_mut()
        .filter(|page| !page.is_sorted_by(|a, b| {
            let ordering = custom_sort(a, b, &rules);
            ordering == Ordering::Less
        }))

        // For each of the filtered page lists, sort it using our custom rules, then add the middle
        // element to the handy built-in accumulator that Rust gives us. Then return it.
        .map(|page| {
            page.sort_by(|a, b| custom_sort(a, b, &rules));

            let middle = page.len() / 2;
            page[middle]
        })
        .sum()
}

// Set up the custom sorting rules, evaluating a specific (a, b) pair from a list of
// pages to determine whether we consider a to be <, >, or = b.
fn custom_sort(a: &u32, b: &u32, rules: &HashMap<u32, HashSet<u32>>) -> Ordering {
    
    // If (a, b) corresponds to an (X, Y) pair in a rule, a < b
    if let Some(set) = rules.get(a) {
        if set.contains(b) {
            return Ordering::Less;
        }
    }

    // If (b, a) corresponds to an (X, Y) pair in a rule, a > b
    if let Some(set) = rules.get(b) {
        if set.contains(a) {
            return Ordering::Greater;
        }
    }

    // Otherwise, a == b
    Ordering::Equal
}

// Process the input into a preprocessed hashmap of X and Y rule values, and a vector of
// vectors of integers to be checked and sorted.
fn process_input(input: Vec<String>) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let split_index = input.iter().position(|s| s.is_empty()).unwrap();
    let (r, p) = input.split_at(split_index);

    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    r.iter()
        .for_each(|s| {
            let mut split = s.split('|').into_iter();
            let x = split.next().unwrap().parse::<u32>().expect("Failed to parse string to int");
            let y = split.next().unwrap().parse::<u32>().expect("Failed to parse string to int");

            rules_map.entry(x).or_insert_with(HashSet::new).insert(y);
        });

    let pages: Vec<Vec<u32>> = p
        .iter()
        .skip(1)
        .map(|s| {
            let split = s.split(',').into_iter();
            split.map(|n| n.parse::<u32>().expect("Failed to parse string to int"))}
            .collect())
        .collect();

    (rules_map, pages)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example.txt");
        let (rules, mut pages) = process_input(input);

        let result = get_sorted_result(rules, &mut pages);
        Ok(assert_eq!(result, 123))
    }
}