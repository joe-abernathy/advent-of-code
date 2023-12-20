use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input: Vec<String> = get_input("./input2.txt");

    let mut total = 0;

    let num_lines = input.len();
    let mut i = 0;

    for line in input {
        i += 1;

        let mut combinations: HashMap<String, (Vec<usize>, usize, bool)> = HashMap::new();

        let mut substr = "".to_string();
        let mut count = 0;
        let (springs, arrangement) = unfold(line);

        // Build the initial substring
        for ch in springs.chars() {
            if ch == '?' {
                break;
            } else {
                substr.push(ch);
            }
        }

        combinations.insert(substr.clone(), (vec![], 0, true));

        next_substring(springs, substr.clone(), arrangement, &mut combinations, &mut count);
        total += count;
        println!("{} of {}\ncount: {}\nrunning total: {}", i, num_lines, count, total);
    }

    println!("\n{}", total);
}


fn next_substring(orig_string: String, substring: String, arrangement: Vec<usize>, combinations: &mut HashMap<String, (Vec<usize>, usize, bool)>, total: &mut usize) {
    let i = substring.len();
    
    let c = combinations.get(&substring).unwrap().clone();

    let mut arr_vec = c.0.clone();
    let mut running_count = c.1;

    if i >= orig_string.len() {
        if arr_vec == arrangement {
            *total += 1;
        }
        return;
    }

    let mut next_unk = false;

    'outer: for opt in vec!["#", "."] {
        arr_vec = c.0.clone();
        running_count = c.1;
     
        let mut running_str = substring.clone() + opt;
                
        match opt {
            "#" => {
                running_count += 1;

                if arr_vec.len() >= arrangement.len() || running_count > arrangement[arr_vec.len()] {
                    combinations.insert(running_str.clone(), (c.0.clone(), running_count, false));
                    continue 'outer;
                }
            },

            "." => {
                if running_count != 0 {
                    arr_vec.push(running_count);
                    running_count = 0;
                    if arr_vec.len() > arrangement.len() || *arr_vec.last().unwrap() != arrangement[arr_vec.len() - 1] {
                        combinations.insert(running_str.clone(), (c.0.clone(), running_count, false));
                        continue 'outer;
                    }
                }
            },

            _ => continue,
        }

        next_unk = false;

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
                        running_str = substring.clone() + opt;
                        combinations.insert(running_str.clone(), (c.0.clone(), c.1, false));
                        continue 'outer;
                    }
                    combinations.insert(running_str.clone(), (arr_vec.clone(), running_count, true));
                },

                '.' => {
                    running_str.push(ch);

                    if running_count == 0 { continue; }
                    arr_vec.push(running_count);
                    running_count = 0;

                    if arr_vec.len() > arrangement.len() || *arr_vec.last().unwrap() != arrangement[arr_vec.len() - 1] {
                        running_str = substring.clone() + opt;
                        combinations.insert(running_str.clone(), (c.0.clone(), c.1, false));
                        continue 'outer;
                    }
                    combinations.insert(running_str.clone(), (arr_vec.clone(), running_count, true));
                },

                _ => continue,
            }
        }

        if !next_unk {
            if running_count != 0 {
                arr_vec.push(running_count);
            }
            if arr_vec == arrangement {
                *total += 1;
            }
            return;
        }

        combinations.entry(running_str.clone()).or_insert((arr_vec.clone(), running_count, true));
        //println!("recursive call");
        next_substring(orig_string.clone(), running_str.clone(), arrangement.clone(), combinations, total);
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
