use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let input: Vec<String> = get_input("./input.txt");

    // Use Rayon to process the input strings in parallel.
    // This takes forever to run sequentially. It also takes forever to run in parallel (like, for real),
    // but it's at least manageable this way.
    let total: usize = input
        .par_iter()
        .enumerate()
        .map(|(i, line)| {
            let (springs, arrangement) = unfold(line.clone());

            let mut substr = String::new();
            let mut current_total = 0;
            let mut count = 0;
            let mut arr_vec: Vec<usize> = vec![];

        // Build the initial substring
        for ch in springs.chars() {
            match ch {
                '?' => break,
                '#' => {
                    substr.push(ch);
                    count += 1;
                },
                '.' => {
                    substr.push(ch);
                    if count != 0 {
                        arr_vec.push(count);
                        count = 0;
                    }
                },
                _ => continue,
            }
        }

        // Call the first instance of the recursive algorithm
        next_substring(&springs, &substr, &arrangement, &arr_vec, count, &mut current_total);
        println!("{} of {}\ncurrent total: {}", i + 1, input.len(), current_total);
        current_total
    })
    .sum();

    println!("\n{}", total);
}


// Starts processing at a '?' and tries both '#' and '.', checks the substring from that point to the next '?' to see if the current '#' or '.'
// causes the substring to become invalid (outside the bounds of the given arrangement). If it doesn't, recursively call this function and check
// both options for the next substring. If this option does cause the substring to become invalid, go back and try the next option.
fn next_substring(orig_string: &str, substring: &str, arrangement: &Vec<usize>, running_arr: &Vec<usize>, count: usize, total: &mut usize) {
    let i = substring.len();
    
    let mut arr_vec = running_arr.clone();
    let mut running_count = count;

    if i >= orig_string.len() {
        if arr_vec == *arrangement {
            *total += 1;
        }
        return;
    }

    let mut next_unk = false;

    // Check both '#' and '.'
    'outer: for opt in vec!["#", "."] {
        arr_vec = running_arr.clone();
        running_count = count;
     
        let mut running_str = substring.to_owned() + opt;
                
        match opt {
            "#" => {
                running_count += 1;

                if arr_vec.len() >= arrangement.len() || running_count > arrangement[arr_vec.len()] {
                    continue 'outer;
                }
            },

            "." => {
                if running_count != 0 {
                    arr_vec.push(running_count);
                    running_count = 0;
                    if arr_vec.len() > arrangement.len() || *arr_vec.last().unwrap() != arrangement[arr_vec.len() - 1] {
                        continue 'outer;
                    }
                }
            },

            _ => continue,
        }

        next_unk = false;

        // Iterate through the current substring until reaching another '?'
        for ch in orig_string[i+1..].chars() {

            match ch {
                '?' => {
                    next_unk = true;
                    break;
                },

                '#' => {
                    running_str.push(ch);
                    
                    running_count += 1;
                    if arr_vec.len() >= arrangement.len() || running_count > arrangement[arr_vec.len()] {
                        continue 'outer;
                    }
                },

                '.' => {
                    running_str.push(ch);

                    if running_count == 0 { continue; }
                    arr_vec.push(running_count);
                    running_count = 0;

                    if arr_vec.len() > arrangement.len() || *arr_vec.last().unwrap() != arrangement[arr_vec.len() - 1] {
                        continue 'outer;
                    }
                },

                _ => continue,
            }
        }

        // If we've reached the end of the string, check if the running arrangement vector matches the given arrangement vector,
        // and if so, the current permutation is valid, so we increment the total counter
        if !next_unk {
            if running_count != 0 {
                arr_vec.push(running_count);
            }
            if arr_vec == *arrangement {
                *total += 1;
            }
            return;
        }

        // Recursively call the function for the next substring
        next_substring(&orig_string, &running_str, &arrangement, &arr_vec, running_count, total);
    }
}


fn parse_input(line: String) -> (String, Vec<usize>) {
    let parts: Vec<_> = line.split_whitespace().collect();

    let springs = parts[0].to_string();

    let arr_str: Vec<_> = parts[1].split(',').collect();
    let arrangement: Vec<usize> = arr_str.iter().map(|s| s.parse::<usize>().unwrap()).collect();

    (springs, arrangement)
}


fn unfold(line: String) -> (String, Vec<usize>) {
    let parts: Vec<_> = line.split_whitespace().collect();

    let mut springs = parts[0].to_string() + "?";
    springs = springs.repeat(5);
    springs = springs[..springs.len() - 1].to_string();
    
    let arr_str: Vec<_> = parts[1].split(',').collect();
    let mut arrangement_folded: Vec<usize> = arr_str.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    let mut arrangement: Vec<usize> = vec![];

    for _ in 0..5 {
        for arr in arrangement_folded.clone() {
            arrangement.push(arr);
        }
    }
    (springs, arrangement)
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}
