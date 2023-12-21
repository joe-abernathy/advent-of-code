use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::HashMap;

/* 
Wow. This was a trip. After a ton of research about dynamic programming and memoization, numerous solve attempts that were either completely
wrong or were on track to take multiple days to complete, and a bunch of bashing my face against the keyboard, this one is done. This solution
involves iterating through the string recursively to determine the possible valid arrangements from any given point in the string to the end
of the string. Rather than attempting to bruteforce this and evaluating every possible combination, whenever we check a substring that we haven't
checked yet, we put the substring into a hashmap along with the number of possible solutions from that point in the string to the end. Then, when
we come across the same substring again in the future, we can just pull its value out of the hashmap instead of having to recursively calculate
it over again. Pretty classic DP stuff, and it took my execution time from >24 hours down to less than a second. 

I can't take full credit for this one -- I got some general algorithm advice from the geniuses on the AoC subreddit, but as a pretty n00by 
programmer, I'm still super proud of this solve.

This solution works for parts 1 and 2. Currently set up for part 2, but to make it work for part 1, just change the call to unfold() in the
main function to a call to parse_input().
*/


fn main() {
    let input: Vec<String> = get_input("./input.txt");

    let mut total = 0;

    for line in input {
        let mut solutions: HashMap<(&str, Vec<usize>, usize), usize> = HashMap::new();

        let (springs, arrangement) = unfold(line.clone());
        let substr_split: Vec<_> = springs.splitn(2, '?').collect();
        let first_substr = substr_split[0].to_string();
        let next_substr = substr_split[1].to_string();

        let (running_arr_opt, running_count) = get_next_arrangement(None, first_substr.clone(), arrangement.clone(), 0, false);
        let running_arr = running_arr_opt.unwrap();
      
        let next = next_spring(running_arr.clone(), running_count);
        
        let current_total = evaluate_substring(&next_substr, running_arr, next, running_count, &mut solutions);
        total += current_total;
    }

    println!("Total: {}", total);
}


// Determine which spring or springs are valid for the next unknown based on the current state
fn next_spring(arrangement: Vec<usize>, running_count: usize) -> Vec<char> {

    if running_count == 0 {
        return vec!['#', '.'];
    }

    if running_count == arrangement[0] {
        return vec!['.'];
    }

    if running_count < arrangement[0] {
        return vec!['#', '.'];
    }

    vec!['#', '.']
}


// Check the substring currently being evaluated. Each substring passed here will be an unknown which has been parsed as either '.' or '#', followed
// by the following characters up to (and not including) the next '?'. As we iterate through the string, we end up discarding previous string segments 
// and entries in the arrangement vector as they go out of scope. This function parses the current substring and returns the remaining arrangement vector 
// entries and the running count of #'s (or None and 0 if the substring being evaluated ends up being invalid). 

// For example, if we're evaluating the string '##.###..' and the arrangement vector that's currently in scope is (2, 3, 3, 1), this function will ensure
// that this configuration stays within the bounds of the given arrangement (in this case it does). Since this substring uses up the first two entries
// in the arrangement vector, those are removed, causing the function to return (3, 1) and running count 0.
fn get_next_arrangement(next: Option<char>, mut substr: String, arrangement: Vec<usize>, mut running_count: usize, eol: bool) -> (Option<Vec<usize>>, usize) {
    let mut i = 0;

    if !next.is_none() {
        substr.insert(0, next.unwrap());
    }

    for ch in substr.chars() {
        match ch {
            '#' => {
                running_count += 1;

                // If this character caused the string to become invalid, return None.
                if i >= arrangement.len() || running_count > arrangement[i] {
                    return (None, 0);
                }
            },

            '.' => {
                if running_count != 0 {

                    // Same here -- if the current string is now invalid, return None.
                    if running_count < arrangement[i] {
                        return (None, 0);
                    }

                    i += 1;
                    running_count = 0;
                }
            },

            _ => continue,
        }
    }

    // We need to do a bit of extra checking if we've reached the end of the line to ensure we don't
    // finish the line without satisfying all the entries of the arrangement vector. For example, if we
    // finish the line, but our arrangement vector still contains (1, 2), we haven't satisfied the 
    // requirements, so this configuration is invalid and we return None. The only exception is if there is
    // one entry left in the arrangement vector, and it's equal to the running count. This can be the case
    // if the line ends with a '#'.
    if eol && arrangement[i..].len() != 0 {
        if arrangement[i..].len() > 1 || *arrangement.last().unwrap() != running_count {
            return (None, 0);
        }
    }

    (Some(arrangement[i..].to_vec()), running_count)
}


// Main recursive function. Evaluates any part of the string recursively and returns the total number of valid configurations from that point in the string forward.
fn evaluate_substring<'a>(substr: &'a str, arrangement: Vec<usize>, next: Vec<char>, running_count: usize, solutions: &mut HashMap<(&'a str, Vec<usize>, usize), usize>) -> usize {  
    let mut valid_count: usize = 0;

    // Have we reached the end of the string?
    let mut eol = false;

    let mut nxt_substr = "";

    // Get the current substring and the next substring up to the next '?' or the end of the string.
    // If we reach the end of the line, set the eol bool
    let substr_split: Vec<_> = substr.splitn(2, '?').collect();
    if substr_split.len() > 1 {
        nxt_substr = substr_split[1];
    } else {
        eol = true;
    }

    let cur_substr = substr_split[0];
 
    // For each potentially valid character (either '#', '.', or both -- determined by next_spring()), get the next arrangement.
    for ch in next {
        let (arr, ct) = get_next_arrangement(Some(ch), cur_substr.to_string(), arrangement.clone(), running_count, eol);
        
        // If arr returns None, the substring we're evaluating is invalid, so move on.
        if arr.is_none() {
            continue;
        }

        // If the substring is valid so far, and we've reached the end of the string, we have a valid arrangement, so count it
        if eol {
            valid_count += 1;
        }
        
        // If we're not at the end of the string, check to see if the remaining substring and arrangement are 
        // stored in the hashmap, and if so, get the count of valid configurations from there without doing any more iteration. 
        // Otherwise, call the algorithm recursively and save its result in the hashmap.
        else {
            if let Some(sol) = solutions.get(&(nxt_substr, arr.clone().unwrap(), ct)) {
                valid_count += sol;
            } else {
                let next_spring = next_spring(arr.clone().unwrap(), ct);
                let sol = evaluate_substring(nxt_substr, arr.clone().unwrap(), next_spring, ct, solutions);
                valid_count += sol;
                solutions.insert((nxt_substr, arr.unwrap(), ct), sol);
            }
        }
    }

    valid_count
}


// Parse the input normally for Part 1
fn parse_input(line: String) -> (String, Vec<usize>) {
    let parts: Vec<_> = line.split_whitespace().collect();

    let springs = parts[0].to_string();

    let arr_str: Vec<_> = parts[1].split(',').collect();
    let arrangement: Vec<usize> = arr_str.iter().map(|s| s.parse::<usize>().unwrap()).collect();

    (springs, arrangement)
}


// Unfold the input for Part 2
fn unfold(line: String) -> (String, Vec<usize>) {
    let parts: Vec<_> = line.split_whitespace().collect();

    let mut springs = parts[0].to_string() + "?";
    springs = springs.repeat(5);
    springs = springs[..springs.len() - 1].to_string();
    
    let arr_str: Vec<_> = parts[1].split(',').collect();
    let arrangement_folded: Vec<usize> = arr_str.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    let mut arrangement: Vec<usize> = vec![];

    for _ in 0..5 {
        for arr in arrangement_folded.clone() {
            arrangement.push(arr);
        }
    }
    (springs, arrangement)
}


// Get the input lines from the input file
fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}
