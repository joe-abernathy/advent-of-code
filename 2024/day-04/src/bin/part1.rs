use aoc_helpers::{ * };
use std::error::Error;
use std::collections::HashMap;


// Set up enums with a built-in step() method to simplify moving across the board in any given direction
enum Dir {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW
}

// Dir::step() expects a position in the form of a tuple of i32s. Our grid only has positive x and y coordinates, but
// I've implemented my own bounds checking within the grid, and using an unsigned value like usize complicates that.
impl Dir {
    fn step(&self, pos: &mut (i32, i32)) {

        match self {
            Dir::N => pos.1 -= 1,

            Dir::S => pos.1 += 1,

            Dir::E => pos.0 += 1,

            Dir::W => pos.0 -= 1,

            Dir::NE => {
                pos.0 += 1;
                pos.1 -= 1;
            },

            Dir::NW => {
                pos.0 -= 1;
                pos.1 -= 1;
            },

            Dir::SE => {
                pos.0 += 1;
                pos.1 += 1;
            },

            Dir::SW => {
                pos.0 -= 1;
                pos.1 += 1;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 4)?;
    let rows = input.len();
    let cols = input[0].len();

    let grid = input_to_hashmap(input);
    let n = word_search(&grid, rows, cols);

    println!("{}", n);
    Ok(())
}

fn word_search(grid: &HashMap<(i32, i32), char>, rows: usize, cols: usize) -> u32 {
    let mut total: u32 = 0;
    
    // Iterate across the top row, checking each vertical row and as many diagonal lines as
    // we can reach starting from this row
    for x in 0..cols {
        let pos = (x as i32, 0);
        total += check_dir(&grid, pos, Dir::S);
        total += check_dir(&grid, pos, Dir::SW);
        total += check_dir(&grid, pos, Dir::SE);
    }

    // Iterate across the left column, checking each horizontal column and the diagonal
    // lines that we couldn't reach from the top row
    for y in 0..rows {
        let pos = (0, y as i32);
        total += check_dir(&grid, pos, Dir::E);

        // We've already counted the SE diagonal row starting from (0, 0), so skip that
        if y != 0 {
            total += check_dir(&grid, pos, Dir::SE);
            
            // There are some SW diagonal lines that we need to get from the bottom row,
            // excluding the row where y == 0, which we already checked
            let pos = ((cols - 1) as i32, y as i32);
            total += check_dir(&grid, pos, Dir::SW);
        }
    }

    total
}

// Get the chars starting from start_pos in the direction dir, put them into a vector, then
// count the number of times "XMAS" or "SAMX" (XMAS backwards) appear in the vector. This 
// lets us check for both forward and backward words at the same time.
fn check_dir(grid: &HashMap<(i32, i32), char>, start_pos: (i32, i32), dir: Dir) -> u32 {
    let mut buf: Vec<char> = Vec::new();

    let mut pos = start_pos;

    // Iterate across the grid in the given direction until getting the value at pos returns
    // a None value, which means we've reached the edge of the grid. Push each character we
    // reach into a buffer to be checked when we're done.
    while let Some(ch) = grid.get(&pos) { 
        buf.push(*ch);
        dir.step(&mut pos);
    }

    // Once we have a fully populated buffer, count the number of times these words appear
    let words = vec!["XMAS", "SAMX"];
    count_words_in_buffer(buf, words)
}

// Takes a vector of chars and a vector of words to check for, and returns the number of 
// times any of those words appear in the buffer.
fn count_words_in_buffer(buf: Vec<char>, words: Vec<&str>) -> u32 {
    let mut total: usize = 0;

    for word in words {
        let word_chars: Vec<char> = word.chars().collect();

        let count = buf
            .windows(word_chars.len())
            .filter(|w| *w == word_chars)
            .count();
        total += count;
    }
    total as u32
}

// Convert the input from a vector of strings into a hashmap with the (x, y)
// coordinates as the key, and the character at those coords as the value
fn input_to_hashmap(input: Vec<String>) -> HashMap<(i32, i32), char> {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    input
        .iter()
        .enumerate()
        .for_each(|(y, s)| {
            s.chars()
                .enumerate()
                .for_each(|(x, ch)| {
                    grid.insert((x as i32, y as i32), ch);
                })
        });
        grid
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example.txt");
        let rows = input.len();
        let cols = input[0].len();

        let grid = input_to_hashmap(input);
        let result = word_search(&grid, rows, cols);

        Ok(assert_eq!(result, 18))
    }
}