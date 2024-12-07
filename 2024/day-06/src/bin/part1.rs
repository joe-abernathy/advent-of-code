use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{HashMap, HashSet};

// This one was fun. I decided to make a Guard struct with methods for moving, turning,
// parsing the next spot in the grid, etc. We create an instance of Guard andset her off
// patrolling, and the step(), next_step(), and turn() functions handle everything else.
// On each step, add the current position to a hashset attribute that will end up containing 
// every unique position visited.

#[derive(Debug)]
struct Guard {
    pos: (i32, i32),
    dir: Dir,
    grid: HashMap<(i32, i32), char>,
    visited: HashSet<(i32, i32)>,
}

// This one was fun. I decided to make a Guard struct with methods for moving, turning,
// parsing the next spot in the grid, etc. We create an instance of Guard andset her off
// patrolling, and the step(), next_step(), and turn() functions handle everything else.
// On each step, add the current position to a hashset attribute that will end up containing 
// every unique position visited.

impl<'a> Guard {
    fn new(ch: char, start_pos: (i32, i32), grid: HashMap<(i32, i32), char>) -> Self {
        let pos = start_pos;

        // The initial guard character determines the initial direction
        let dir = match ch {
            '^' => Dir::N,
            'v' => Dir::S,
            '>' => Dir::E,
            '<' => Dir::W,
            _ => panic!("Invalid direction: {}", ch),
        };

        let visited: HashSet<(i32, i32)> = HashSet::new();

        Guard { pos, dir, grid, visited }
    }

    // Start the guard patrolling. She will keep going according to the puzzle instructions
    // until next_step() (the method that gets the character at the next position in the current
    // direction) returns None, meaning we've left the grid, at which point we're done.
    fn patrol(&mut self) {

        // This will loop until we get a None value, meaning we're done. 
        while let Some(ch) = self.next_step() {
            
            // Add the current position to the hashset
            self.visited.insert(self.pos);

            // If the next position holds an obstacle, turn right
            if ch == '#' {
                self.turn();
            }

            // Take a step in the current direction
            self.step();
        }

        self.visited.insert(self.pos);
    }

    // Return the character value at the next step in the current direction, or None if
    // we've left the input grid.
    fn next_step(&self) -> Option<char> {

        let next_pos: (i32, i32) = match self.dir {
            Dir::N => {
                (self.pos.0, self.pos.1 - 1)
            },

            Dir::S => {
                (self.pos.0, self.pos.1 + 1)
            },

            Dir::E => {
                (self.pos.0 + 1, self.pos.1)
            },

            Dir::W => {
                (self.pos.0 - 1, self.pos.1)
            }
        };

        self.grid.get(&next_pos).copied()
    }

    // Update the guard's current position by 1 step in the current direction
    fn step(&mut self) {

        match self.dir {
            Dir::N => self.pos.1 -= 1,
            Dir::S => self.pos.1 += 1,
            Dir::E => self.pos.0 += 1,
            Dir::W => self.pos.0 -= 1,
        }
    }

    // Update the guard's current direction 45 degrees to the right
    fn turn(&mut self) {
        match self.dir {
            Dir::N => self.dir = Dir::E,
            Dir::S => self.dir = Dir::W,
            Dir::E => self.dir = Dir::S,
            Dir::W => self.dir = Dir::N,
        }
    }
}

#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W
}

fn main() -> Result<(), Box<dyn Error>> {
    // let input = read_from_file_as_lines("example.txt");
    let input = get_puzzle_input_as_lines(2024, 6)?;
    let (grid, pos) = input_to_grid(input);

    let mut guard = Guard::new(*grid.get(&pos).unwrap(), pos, grid);
    guard.patrol();

    let result = guard.visited.len();
    println!("{}", result);
    Ok(())
}

// Take the input vector and return a hashmap representing the map grid, and the guard's
// initial position as identified by the carat char in the input
fn input_to_grid(input: Vec<String>) -> (HashMap<(i32, i32), char>, (i32, i32)) {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut start_pos: (i32, i32) = (0, 0);

    input
        .iter()
        .enumerate()
        .for_each(|(y, s)| {
            s.chars()
                .enumerate()
                .for_each(|(x, ch)| {

                    // Save the current index and corresponding character to the grid
                    grid.insert((x as i32, y as i32), ch);

                    // If the current index contains the guard's starting position, save
                    // that as start_pos
                    if ['^', '>', '<', 'v'].contains(&ch) {
                        start_pos = (x as i32, y as i32);
                    }
                })
            });
    (grid, start_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example.txt");
        let (grid, start_pos) = input_to_grid(input);

        let mut guard = Guard::new(*grid.get(&start_pos).unwrap(), start_pos, grid);
        guard.patrol();

        let result = guard.visited.len();
        Ok(assert_eq!(result, 41))
    }
}