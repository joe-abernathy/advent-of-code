use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{HashMap, HashSet};

// This one took some thinking. I had to track down several edge cases that were tripping me up
// and just generally ruining my life, but we're here now.

// Similar to part 1, we have a Guard struct to model the guard's movements. We let the guard
// patrol once without any extra obstacles to get her normal path to narrow down the pool of
// positions where we can add an obstacle to change the guard's path. We then iterate across
// all positions in the path, adding a single obstacle at each and letting the guard patrol
// to check for a cycle. Then we just count the number of cycles.

// Had to update the Guard struct a bit to keep track of some more stuff this time to check for 
// cycles. Instead of just putting coordinates into the 'visited' set, we also add the current 
// direction. If we hit the same position and direction again, we've found a cycle, so just
// return.

#[derive(Debug)]
struct Guard {
    pos: (i32, i32),
    start_pos: (i32, i32),
    dir: Dir,
    start_dir: Dir,
    grid: HashMap<(i32, i32), char>,
    visited: HashSet<(i32, i32, Dir)>,
    added_obstacle: (i32, i32),
}

impl<'a> Guard {
    fn new(ch: char, start_pos: (i32, i32), grid: HashMap<(i32, i32), char>) -> Self {
        let pos = start_pos;

        // The initial guard character determines the initial direction
        let start_dir = match ch {
            '^' => Dir::N,
            'v' => Dir::S,
            '>' => Dir::E,
            '<' => Dir::W,
            _ => panic!("Invalid direction: {}", ch),
        };

        let dir = start_dir;

        let visited: HashSet<(i32, i32, Dir)> = HashSet::new();
        let added_obstacle = (-1, -1);

        Guard { pos, start_pos, dir, start_dir, grid, visited, added_obstacle }
    }

    // Start the guard patrolling and check for a cycle. This returns true if
    // a cycle is found, or false if it finishes by leaving the grid without
    // finding a cycle.
    fn patrol(&mut self) -> bool {
        
        self.pos = self.start_pos;
        self.dir = self.start_dir;

        // Clear the visited hashset for a new run
        self.visited.clear();

        // This will loop until we get a None value, or until we detect a cycle.
        // If we get a cycle, stop and return true immediately, or if we end up
        // leaving the grid, there is no cycle, so return false.
        while let Some(ch) = self.next_step() {
            
            let coords = (self.pos.0, self.pos.1, self.dir);

            // If we've already been to this position moving in this direction,
            // we've hit a cycle, so return true
            if self.visited.contains(&coords) {
                return true;
            }

            // Add the current position/direction to the list
            self.visited.insert(coords);

            // If the next position holds an obstacle, turn right until 
            if ch == '#' {
                self.turn();

                // This catches an edge case where there are obstacles 
                // making a corner requiring us to turn more than once
                while self.next_step() == Some('#') {
                    self.turn();
                }
            }
            
            // Take a step in the current direction
            self.step();
        }

        // If we exit the grid, add that last valid position to the list,
        // then return false indicating that we don't have a cycle
        self.visited.insert((self.pos.0, self.pos.1, self.dir));

        false
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

    // Restore the grid to its original state (if applicable) and add an obstacle
    // at the indicated position.
    fn add_obstacle(&mut self, pos: (i32, i32)) {

        if self.added_obstacle != (-1, -1) {
            self.grid.insert(self.added_obstacle, '.');
        }

        // Add an obstacle to the listed position, then store that position so we
        // can restore it next time this is called.
        self.grid.insert(pos, '#');
        self.added_obstacle = pos;
    }

}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W
}

fn main() -> Result<(), Box<dyn Error>> {
    // let input = read_from_file_as_lines("example.txt");
    let input = get_puzzle_input_as_lines(2024, 6)?;
    let (grid, start_pos) = input_to_grid(input);

    let result = count_cycles(grid, start_pos);
    println!("{}", result);
    Ok(())
}

fn count_cycles(grid: HashMap<(i32, i32), char>, start_pos: (i32, i32)) -> usize {
    
    // Instantiate the guard and let her patrol once to get her path, in order to populate
    // a list of positions where we could add an obstacle to change the path
    let mut guard = Guard::new(*grid.clone().get(&start_pos).unwrap(), start_pos, grid);
    guard.patrol();

    let candidates: HashSet<(i32, i32)> = guard.visited
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .filter(|&pos| pos != start_pos)
        .collect();
    
    candidates.iter()
        .filter(|coords| {
            let pos = (coords.0, coords.1);
            guard.add_obstacle(pos);
            guard.patrol()
        })
        .count()
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

        let result = count_cycles(grid, start_pos);
        Ok(assert_eq!(result, 6))
    }
}