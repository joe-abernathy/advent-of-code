use aoc_helpers::{ * };
use std::error::Error;

// This is probably unnecessary, but I had a whole plan in my head before I realized this would be a bit
// simpler than I thought. This just enumerates the possible directions (up/down/left/right).
enum Dir {
    U,
    D,
    L,
    R,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2022, 8)?;
    let grid = input_to_grid(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut running_total = 0;

    // Iterate across all entries in the grid, checking whether each entry is visible. If so,
    // increment the running total.
    for y in 0..rows {
        for x in 0..cols {
            if check_position(grid.clone(), x, y) {
                running_total += 1;
            }
        }
    }

    println!("{}", running_total);

    Ok(())
}

// Checks whether a position in the grid (given by (x, y)) is visible from the
// outside. If so, return true, otherwise return false.
fn check_position(grid: Vec<Vec<u8>>, x: usize, y: usize) -> bool {

    // Calculate the max possible x and y values for the grid to prevent overflowing
    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;

    // If the current position is on the exterior of the grid, it's visible by definition,
    // so just return true.
    if x == 0 || x == max_x || y == 0 || y == max_y {
        return true;
    }

    // Check each direction for the given position. If check_dir() returns true for any
    // direction, this function immediately returns true without further processing.
    for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
        if check_dir(grid.clone(), x, y, dir) {
            return true;
        }
    }

    // If we've evaluated all directions without returning true, this position is not visible.
    return false;
}

// This checks a position for visibility from a specific direction. Returns true if 
// the position is visible from that direction, otherwise false.
fn check_dir(grid: Vec<Vec<u8>>, mut x: usize, mut y: usize, dir: Dir) -> bool {

    // Record the height of the tree at the current position
    let current_height = grid[y][x];

    // Get the max possible x and y values to prevent overflows
    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;

    // In general, we don't need to check whether we're on the exterior of the grid because
    // that check is done in check_position() before this function is called. So ignore that case.

    // For whichever direction we're checking, iterate across that row/column from the current
    // position to the end, immediately returning false if we come across a tree of equal or
    // greater height than the tree we're evaluating. If we get to the end without returning,
    // the tree must be visible, so return true.

    // Is there a better way to do this? I'm almost positive there is. But we're here now.
    match dir {
        Dir::U => {
            while y != 0 {
                y -= 1;
                if grid[y][x] >= current_height {
                    return false;
                }
            }
            return true;
        },

        Dir::D => {
            while y != max_y {
                y += 1;
                if grid[y][x] >= current_height {
                    return false;
                }
            }
            return true;
        },

        Dir::L => {
            while x != 0 {
                x -= 1;
                if grid[y][x] >= current_height {
                    return false;
                }
            }
            return true;
        },

        Dir::R => {
            while x != max_x {
                x += 1;
                if grid[y][x] >= current_height {
                    return false;
                }
            }
            return true;
        }
    }
}

// Parses the input string vector to a grid, in the form of a vector of vectors of u8's
fn input_to_grid(input: Vec<String>) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in input {
        let row: Vec<u8> = line
            .chars()
            .into_iter()
            .map(|ch| ch.to_digit(10).expect("Expected a digit, got something else"))
            .map(|d| d as u8)
            .collect();

        grid.push(row);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn Error>> {
        let input = read_from_file_as_lines("example.txt");
        let grid = input_to_grid(input);

        let cols = grid.len();
        let rows = grid[0].len();

        let mut running_total = 0;

        for y in 0..rows {
            for x in 0..cols {
                if check_position(grid.clone(), x, y) {
                    running_total += 1;
                }
            }
        }

        Ok(assert_eq!(running_total, 21))
    }
}