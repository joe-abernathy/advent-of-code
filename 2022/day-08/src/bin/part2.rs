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
    // let input = read_from_file_as_lines("example.txt");
    let grid = input_to_grid(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut score_grid: Vec<Vec<u32>> = vec![vec![0; cols]; rows];

    // Iterate across all entries in the grid, checking whether each entry is visible. If so,
    // increment the running total.
    for y in 0..rows {
        for x in 0..cols {
            score_grid[y][x] = check_position(grid.clone(), x, y);
        }
    }

    let max_score = score_grid.clone()
        .into_iter()
        .flat_map(|inner| inner.into_iter())
        .max()
        .unwrap();

    println!("{}", max_score);
    Ok(())
}

// Calculates the scenic score for a given position in the grid by checking visibility
// in each direction and multiplying the four scores together
fn check_position(grid: Vec<Vec<u8>>, x: usize, y: usize) -> u32 {

    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;

    // If the tree is on the edge of the map, the scenic score will be 0, so we don't
    // need to do any more processing.
    if x == 0 || y == 0 || x == max_x || y == max_y {
        return 0;
    }

    let mut scenic_score: u32 = 1;

    // Check each direction for the given position, multiplying scenic_score by the
    // result returned for each direction
    for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
        let current_score = check_dir(grid.clone(), x, y, dir);
        scenic_score *= current_score;
    }

    scenic_score
}

// This checks the number of trees visible from a given position in a given
// direction. We again don't need to check the case where the position is on
// the edge, because the calling function (check_position()) handles that case.
fn check_dir(grid: Vec<Vec<u8>>, mut x: usize, mut y: usize, dir: Dir) -> u32 {

    // Record the height of the tree at the current position
    let current_height = grid[y][x];

    // Get the max possible x and y values to prevent overflows
    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;

    let mut distance = 1;

    // For each possible direction, iterate from the current position until
    // we reach a taller tree (or the edge of the grid), returning the number
    // of trees visible before we can't see any more
    match dir {
        Dir::U => {
            while y != 0 {
                y -= 1;

                if grid[y][x] >= current_height || y == 0 {
                    return distance;
                }

                distance += 1;
            }

            return distance;
        },

        Dir::D => {
            while y != max_y {
                y += 1;

                if grid[y][x] >= current_height || y == max_y {
                    return distance;
                }

                distance += 1;
            }

            return distance;
        },

        Dir::L => {
            while x != 0 {
                x -= 1;

                if grid[y][x] >= current_height || x == 0 {
                    return distance;
                }

                distance += 1;
            }

            return distance;
        },

        Dir::R => {
            while x != max_x {
                x += 1;

                if grid[y][x] >= current_height || x == max_x {
                    return distance;
                }

                distance += 1;
            }

            return distance;
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

        let mut score_grid: Vec<Vec<u32>> = vec![vec![0; cols]; rows];

        for y in 0..rows {
            for x in 0..cols {
                score_grid[y][x] = check_position(grid.clone(), x, y);
            }
        }
    
        let max_score = score_grid
            .into_iter()
            .flat_map(|inner| inner.into_iter())
            .max()
            .unwrap();

        Ok(assert_eq!(max_score, 8))
    }
}