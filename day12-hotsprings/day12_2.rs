use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input: Vec<String> = get_input("./example.txt");

    let mut combinations: HashMap<String, (Vec<usize>, usize, bool)> = HashMap::new();
    let mut substr = "".to_string();
    let mut total = 0;

    for line in input {
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
    }

    println!("{}", total);
}


fn next_substring(orig_string: String, substring: String, arrangement: Vec<usize>, combinations: &mut HashMap<String, (Vec<usize>, usize, bool)>, total: &mut usize) {

    println!("orig_str: {}, initial substr: {}, arr: {:?}", orig_string, substring, arrangement);
    let i = substring.len();
    
    let c = combinations.get(&substring).unwrap().clone();

    let mut arr_vec = c.0.clone();
    let mut running_count = c.1;

    let mut next_unk = false;

    'outer: for opt in vec!["#", "."] {
        let mut running_str = substring.clone();
        running_str += opt;

        println!("{}: substr: {}, arr: {:?}", opt, running_str, arrangement);
        match opt {
            "#" => {
                running_count += 1;

                if running_count > arrangement[arr_vec.len()] {
                    combinations.insert(running_str.clone(), (c.0.clone(), running_count, false));
                    break;
                }
            },

            "." => {
                arr_vec.push(running_count);
                running_count = 0;
                if arr_vec.len() > arrangement.len() || *arr_vec.last().unwrap() != arrangement[arr_vec.len()] {
                    combinations.insert(running_str.clone(), (c.0.clone(), running_count, false));
                }
            },

            _ => continue,
        }

        for ch in orig_string[i..].chars() {
            if ch == '?' {
                println!("FINAL SUBSTRING: {}", running_str);
                next_unk = true;
                break;
            }

            running_str.push(ch);

            if ch == '#' {
                running_count += 1;
                let mut cur = 0;

                if arr_vec.len() == 0 {
                    cur = 0;
                } else {
                    cur = arr_vec.len() - 1;
                }

                println!("# : substr: {}, running: {}, arr_vec: {:?}, cur: {}, arrangement: {:?}, arrangement[cur]: {}", running_str, running_count, arr_vec, cur, arrangement, arrangement[cur]);
                if running_count > arrangement[cur] {
                    println!("# : BREAK");
                    running_str = substring.clone() + opt;
                    combinations.insert(running_str.clone(), (c.0.clone(), c.1, false));
                    break 'outer;
                }
                combinations.insert(running_str.clone(), (arr_vec.clone(), running_count, true));
            
            } else if ch == '.' {
                arr_vec.push(running_count);
                running_count = 0;
                let cur = arr_vec.len() - 1;
                println!(". : running: {}, arr_vec: {:?}, cur: {}, arrangement: {:?}, arrangement[cur]: {}", running_count, arr_vec, cur, arrangement, arrangement[cur]);

                if cur > arrangement.len() || arr_vec[cur] != arrangement[cur] {
                    println!(". : {} fails, breaking", running_str);
                    running_str = substring.clone() + opt;
                    combinations.insert(running_str.clone(), (c.0.clone(), c.1, false));
                    break 'outer;
                }


                combinations.insert(running_str.clone(), (arr_vec.clone(), running_count, true));
            }
        }
        println!("does this even work? {}", running_str.clone());
        combinations.entry(running_str.clone()).or_insert((arr_vec.clone(), running_count, true));
        next_substring(orig_string.clone(), running_str.clone(), arrangement.clone(), combinations, total);
    }

    if !next_unk {
        *total += 1;
        return;
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