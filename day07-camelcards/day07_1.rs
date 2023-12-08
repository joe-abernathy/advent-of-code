use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use lazy_static::lazy_static;

const HAND_SIZE: usize = 5;

// Assigns a value to each card rank for evaluation later
lazy_static! {
    static ref CARD_VAL: HashMap<char, u32> = {
        let mut map = HashMap::new();
        for ch in '2'..'9' {
            map.insert(ch, ch.to_digit(10).unwrap());
        }
        map.insert('9', 9);
        map.insert('T', 10);
        map.insert('J', 11);
        map.insert('Q', 12);
        map.insert('K', 13);
        map.insert('A', 14);
        map    
    };
}

fn main() {

    let input = get_input("./input.txt");
    let mut hands: Vec<Vec<String>> = Vec::new();
    for line in input {
        let hand: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        hands.push(hand);
    }

    hands.sort_by(|a, b| compare_cards(&a[0], &b[0]));

    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        let value = hand[1].parse::<u64>().unwrap();
        total += value * ((i + 1) as u64);
    }

    println!("{}", total);
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File doesn't exist");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}

fn compare_cards(h1: &str, h2: &str) -> Ordering {
    let v1 = evaluate_hand(h1);
    let v2 = evaluate_hand(h2);
    
    if v1 > v2 { return Ordering::Greater; }
    else if v1 < v2 { return Ordering::Less; }
    else {
        return check_same_type(h1, h2);
    }
}

// Evaluates a hand by its type
fn evaluate_hand(hand: &str) -> u8 {
    let unique_chars = count_unique_chars(hand);

    if unique_chars == 1 {
        return 7;               // 5 of a kind
    } else if unique_chars == 2 {
        let first_count = hand.chars().filter(|&c| c == hand.chars().nth(0).unwrap()).count();
        if first_count == 1 || first_count == 4 {
            return 6;           // 4 of a kind
        } else {
            return 5;           // full house
        }
    } else if unique_chars == 3 {   
        for i in 0..HAND_SIZE {
            let count = hand.chars().filter(|&c| c == hand.chars().nth(i).unwrap()).count();
            if count == 1 { continue; }
            if count == 2 {
                return 3;       // 2 pair
            } else if count == 3 {
                return 4;       // 3 of a kind
            }
        }
    } else if unique_chars == 4 {
        return 2;               // 1 pair
    } else {
        return 1;               // high card
    }
    0
}

// Evaluates a hand card by card if they have the same type
fn check_same_type(h1: &str, h2: &str) -> Ordering {
    for i in 0..HAND_SIZE {
        if h1.chars().nth(i) == h2.chars().nth(i) { 
            continue; 
        }
        let v1 = CARD_VAL.get(&h1.chars().nth(i).unwrap());
        let v2 = CARD_VAL.get(&h2.chars().nth(i).unwrap());
        if v1 > v2 {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    println!("uh oh");
    Ordering::Equal
}

fn count_unique_chars(input: &str) -> usize {
    let unique_chars: HashSet<char> = input.chars().collect();
    unique_chars.len()
}