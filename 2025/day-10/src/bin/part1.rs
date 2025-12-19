use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{ VecDeque, HashSet };
use std::time::Instant;

// This one was fun. The current state of the lights, the target state, and the buttons can all be represented as integers, with
// each light representing a single bit. That makes toggling lights just a matter of XORing the current state with the button.

fn main() -> Result<(), Box<dyn Error>> {
    let machines = parse_input()?;

    let now = Instant::now();

    let result: u32 = machines
        .iter()
        .map(|m| m.push_buttons())
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
    target: u16,
    buttons: Vec<u16>,
    joltage: Vec<u32>
}

impl Machine {
    
    // Creates a new instance of Machine from a string from the puzzle input
    fn from_line(line: &str) -> Self {
        let mut spl: Vec<&str> = line.split(']').collect();

        // Store the desired state as a u16 bitmask where each bit represents a light
        let target: u16 = spl[0][1..]
            .chars()
            .rev()
            .fold(0u16, |acc, ch| {
                (acc << 1) | match ch {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Invalid character found when parsing target"),
                }
            });

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

        // We don't need the joltage for this part, but it's here anyway
        let joltage: Vec<u32> = spl[1][..spl[1].len() - 1]
            .split(',')
            .collect::<Vec<_>>()
            .into_iter()
            .map(|ch| ch.parse::<u32>().unwrap())
            .collect();

        Self { target, buttons, joltage }
    }

    // BFS algorithm to find the smallest number of button presses to reach the target state
    fn push_buttons(&self) -> u32 {
        let mut visited: HashSet<u16> = HashSet::from([0]);
        let mut queue: VecDeque<(u16, u32)> = VecDeque::from([(0u16, 0u32)]);

        while let Some((state, depth)) = queue.pop_front() {
            if state == self.target {
                return depth;
            }

            for button in &self.buttons {
                let next = state ^ button;

                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, depth + 1));
                }
            }
        }
        0
    }
}