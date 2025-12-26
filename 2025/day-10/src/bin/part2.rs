use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{ HashSet, HashMap };
use std::time::Instant;
use itertools::Itertools;

// Not ashamed to admit I got some help on this one. I went through 4 or 5 different algorithms that worked great on the example
// input but ballooned out of control on the real input. After about a week of banging my head against the wall, I went searching
// for a good algorithm for solving this, and found this brilliant implementation from u/tenthmascot on Reddit:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
//
// Essentially, this builds on the part 1 solution by correlating each target joltage to an indicator light state based on whether
// the given target joltage number is even or odd. It iterates through all possible combinations of single button presses and finds
// all of those which result in the indicator status lights being in the correct states, decrements each affected counter, and 
// runs the algorithm again recursively using the updated target joltages, returning the result of that recursive function call plus
// the number of button presses in the first set of button presses. Because the indicator lights toggle, an even number of presses 
// will result in the affected lights ending up in their original state. Because of that fact, we know that when the lights are in
// their target state, we need some even number of button presses to get to the target joltage. So we can save some execution cycles
// by cutting the remaining joltages in half (since they're all even), recursing on the halved joltages, and then multiply the result
// result by 2. 
//
// This runs in ~14 seconds on my machine, which I call a win.

fn main() -> Result<(), Box<dyn Error>> {
    let mut machines = parse_input()?;

    let now = Instant::now();

    let result: u32 = machines
        .iter_mut()
        .map(|m| m.solve())
        .sum();

    println!("Result: {}\nExecution time: {:?}", result, now.elapsed());
    Ok(())
}

fn parse_input() -> Result<Vec<Machine>, Box<dyn Error>> {
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 10)?
    };

    let machines = input
        .iter()
        .map(|line| Machine::from_line(line))
        .collect();

    Ok(machines)
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<u16>,
    joltage_target: Vec<u32>,
}

impl Machine {
    
    // Creates a new instance of Machine from a string from the puzzle input
    fn from_line(line: &str) -> Self {
        let mut spl: Vec<&str> = line.split(']').collect();

        spl = spl[1].split('{').collect();

        let buttons_str: Vec<&str> = spl[0]
            .trim()
            .split_ascii_whitespace()
            .collect();

        // Store each button as a u16 bitmask representing the lights/bits to be toggled
        let buttons: Vec<u16> = buttons_str
            .iter()
            .map(|s| s[1..s.len() - 1].split(',')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|ch| ch.parse::<u16>().unwrap())
                .fold(0u16, |acc, n| acc | (1 << n)))
            .collect();

        let joltage_target: Vec<u32> = spl[1][..spl[1].len() - 1]
            .split(',')
            .collect::<Vec<_>>()
            .into_iter()
            .map(|ch| ch.parse::<u32>().unwrap())
            .collect();
        
        Self { buttons, joltage_target }

    }

    fn solve(&mut self) -> u32 {
        let target = self.joltage_target.clone();

        self.solve_next(&target, 0)
    }

    fn solve_next(&mut self, target: &Vec<u32>, current_count: u32) -> u32 {
        // If the current target is all zeros, we've hit the end of recursion
        if target.iter().all(|&c| c == 0) {
            return 0;
        }

        // Get the parity from the current target
        let parity = self.get_parity(&target);
        
        // Get all combinations of single button presses and filter them down to the ones
        // that result in the correct target parity
        let valid_combinations: HashSet<Vec<u16>> = self.buttons.iter()
            .powerset()
            .filter(|c| self.check_combination(c, &parity))
            .map(|c| c.into_iter().copied().collect())
            .collect();
    
        let mut best = u32::MAX;

        // Check the number of presses for each valid combination and return the lowest number
        for combo in valid_combinations {
            if let Some(next_target) = self.get_next_target(target, &combo) {
                let subcost = self.solve_next(&next_target, current_count + combo.len() as u32);
                if subcost != u32::MAX {
                    let cost = 2 * subcost + combo.len() as u32;
                    best = u32::min(best, cost);
                } 
            }
        }

        best
    }

    // Checks whether a combination of button presses is valid by XORing the state with
    // each button. The combination is valid if the result matches the target parity
    fn check_combination(&self, combination: &Vec<&u16>, target_parity: &u16) -> bool {
        let mut state = 0u16;

        for button in combination {
            state ^= *button;
        }

        state == *target_parity
    }

    // Convert the current target joltage to a parity integer, where 0 in a given bit indicates that
    // the joltage in that index is even, and 1 indicates that the joltage is odd.
    fn get_parity(&self, target: &Vec<u32>) -> u16 {
        target.iter().rev().fold(0u16, |acc, val| (acc << 1) | (val % 2) as u16)
    }

    // Get the next set of target joltages by subtracting the total presses from the previous step and 
    // cutting the result (which will always be even) in half. This return None if the button presses
    // from the previous step cause any joltage to drop below zero, which indicates that that was not a 
    // valid combination of presses.
    fn get_next_target(&mut self, current_target: &Vec<u32>, combinations: &Vec<u16>) -> Option<Vec<u32>> {
        let mut next_target = current_target.clone();

        for &combo in combinations {
            for i in 0usize..16 {
                if (combo >> i) & 1 != 0 {
                    if let Some(new) = next_target[i].checked_sub(1) {
                        next_target[i] = new;
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(next_target.iter().map(|n| n / 2).collect())
    }
}