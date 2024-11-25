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
        map.insert('J', 1);     // in part 2, J has the lowest rank
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

// Evaluates a hand by its type, accounting for wildcards
fn evaluate_hand(hand: &str) -> u8 {
    let count_j = hand.chars().filter(|&c| c == 'J').count();

    // Get the hand's type by checking the number of unique cards in the hand
    let unique_cards = count_unique_cards(hand);

    // If there's only 1 unique card, it's 5 of a kind. No further checking needed.
    if unique_cards == 1 {
        return 7;                   // 5 of a kind

    // If there are 2 unique cards, it's either 4 of a kind or a full house -- we have
    // to count the occurrences of any one card to find out which. We'll use the first for simplicity.
    } else if unique_cards == 2 {
        let first_count = hand.chars().filter(|&c| c == hand.chars().nth(0).unwrap()).count();
        
        // If there are 1 or 4 occurrences of the first card, it's 4 of a kind
        if first_count == 1 || first_count == 4 {

            // ...but if any character is a J, it becomes 5 of a kind
            if count_j == 0 {
                return 6;           // 4 of a kind
            }
            return 7;               // 5 of a kind (with wildcards)

        // 2 or 3 instances of the first char --> full house 
        } else {
            // ...unless any card is wild, in which case it becomes 5 of a kind
            if count_j == 0 {
                return 5;           // full house
            }
            return 7;               // 5 of a kind (with wildcards)
        }

    // With 3 unique cards, we may have to look at more than one card to find the type.
    } else if unique_cards == 3 {   
        for i in 0..HAND_SIZE {
            let count = hand.chars().filter(|&c| c == hand.chars().nth(i).unwrap()).count();

            // If a card only occurs once in the hand, it doesn't tell us anything, so go to the next one
            if count == 1 { continue; }

            // If any card occurs twice, we have two pair
            if count == 2 {
                
                // ...assumimng there are no wildcards
                if count_j == 0 {
                    return 3;       // 2 pair

                // 
                } else {
                    match count_j {
                        1 => return 5,    // 1 wildcard makes this a full house
                        2 => return 6,    // 2 wildcards makes this 4 of a kind
                        _ => return 0,    // It shouldn't be possible to have any other counts
                    };
                }

            // If any card occurs three times, we have 3 of a kind
            } else if count == 3 {

                // ...assuming there are no wildcards
                if count_j == 0 {
                    return 4;       // 3 of a kind

                // If there are any wildcards in this hand, it becomes 4 of a kind
                } else {
                    return 6;       // 4 of a kind (with wildcards)
                }
            }
        }

    // If there are 4 unique cards, it's a pair
    } else if unique_cards == 4 {

        // ...assuming there are no wildcards
        if count_j == 0 {
            return 2;               // 1 pair
        
        // A wildcard in this hand makes it 3 of a kind
        } else {
            return 4;               // 3 of a kind (with wildcards)
        }

    // If there are 5 unique cards, we've just got a high card
    } else {

        // ...assuming there are no wildcards
        if count_j == 0 {
            return 1;               // high card

        // A wildcard would turn this into a pair
        } else {
            return 2;               // 1 pair (with wildcards)
        }
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

fn count_unique_cards(input: &str) -> usize {
    let unique_cards: HashSet<char> = input.chars().collect();
    unique_cards.len()
}