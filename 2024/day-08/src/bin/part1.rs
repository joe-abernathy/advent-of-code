use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{ HashMap, HashSet };

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 8)?;
    // let input = read_from_file_as_lines("example1.txt");
    let rows = input.len();
    let cols = input[0].len();
    
    let grid = input_to_grid(input);
    let result = find_antinodes(&grid, rows, cols);

    println!("{}", result);

    Ok(())
}

fn find_antinodes(grid: &HashMap<(i32, i32), char>, rows: usize, cols: usize) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // Generate a list of all possible slopes for lines in the given grid
    let slopes = get_slopes(cols as i32, rows as i32);

    // Iterate across all valid spots in the grid, tracing a line from each spot with each possible
    // slope. Is there a way to optimize this and pare it down? Probably. But I'm honestly just 
    // shocked that I got this far. I can always look into optimizing later.
    for y in 0..rows {
        for x in 0..cols {
            let pos = (x as i32, y as i32);
            for slope in slopes.clone() {

                // Trace a line from the given start position with the given slope, generating a
                // list of duplicate characters with their indices within the line
                let dup = trace_line(&grid, pos, slope);
                if dup.is_empty() {
                    continue;
                }

                // For each set of offsets in the dup hashmap, perform some checks to see if valid
                // antinodes exist in the line we've generated
                for (_, offsets) in dup {

                    // Calculate the distance across the line between the duplicate chars
                    let delta = offsets[0].abs_diff(offsets[1]);

                    // In order to be a valid antinode, one of the towers has to be twice as far
                    // from the given position than the other. From the puzzle input it seems like
                    // we can assume that every set of towers on a line will have 2 valid antinodes,
                    // but I'm still doing a bit of validation to ensure that both possible spots
                    // are within the grid by checking whether getting the char from the grid 
                    // hashmap doesn't return None, which would indicate that we're outside the grid.
                    let index = offsets[0] as i32 - delta as i32;
                    let new_pos = get_position(pos, slope, index);

                    if grid.get(&new_pos).is_some() {
                        antinodes.insert(new_pos);
                    }

                    let index = offsets[1] as i32 + delta as i32;
                    let new_pos = get_position(pos, slope, index);

                    if grid.get(&new_pos).is_some() {
                        antinodes.insert(new_pos);
                    }
                }
            }
        }
    }
    antinodes.len()
}

// Return the (x, y) coordinates of a spot based on the slope of the line it's on, the start position,
// and the index along the line
fn get_position(pos: (i32, i32), slope: (i32, i32), index: i32) -> (i32, i32) {
    (pos.0 + (slope.0 * index), pos.1 + (slope.1 * index))
}

// Trace a single line, putting the chars along that line into a buffer, and then returning a hashmap of
// any duplicate values in the line and their respective indices.
fn trace_line(grid: &HashMap<(i32, i32), char>, start_pos: (i32, i32), slope: (i32, i32)) -> HashMap<char, Vec<usize>> {
    let mut pos = start_pos;
    let buf = fill_buffer(&grid, &mut pos, slope);
    
    find_duplicates(buf)
}

// For a given start position and slope, fill a buffer with the characters in the resulting line
fn fill_buffer(grid: &HashMap<(i32, i32), char>, pos: &mut (i32, i32), slope: (i32, i32)) -> Vec<char> {
    let mut buf: Vec<char> = Vec::new();

    while let Some(ch) = grid.get(&pos) {
        buf.push(*ch);
        slope_step(pos, slope);
    }

    buf
}

// Takes a buffer of chars representing a single line, and return a hashmap of any duplicate chars and
// their respective indices along the line
fn find_duplicates(buf: Vec<char>) -> HashMap<char, Vec<usize>> {
    let mut indices: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, &ch) in buf.iter().enumerate() {
        if ch != '.' {
            indices.entry(ch).or_insert_with(Vec::new).push(i);
        }
    }

    indices.into_iter()
        .filter(|(_, indices)| indices.len() > 1)
        .collect()
}

// Increment a position pointer along a line with a given slope
fn slope_step(pos: &mut (i32, i32), slope: (i32, i32)) {
    pos.0 += slope.0;
    pos.1 += slope.1;
}

// Populate a hashmap with all possible slope values for a grid of a given size
fn get_slopes(max_x: i32, max_y: i32) -> HashSet<(i32, i32)> {
    let mut slopes: HashSet<(i32, i32)> = HashSet::new();

    for dy in -max_y..max_y {
        for dx in -max_x..max_x {
            if dy == 0 && dx == 0 {
                continue;
            }
            let divisor = gcd(dy, dx);
            slopes.insert((dy / divisor, dx / divisor));
        }
    }

    slopes
}

// Get the greatest common demoninator of two numbers to help with building a list of slopes
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

// Parse the input from a vector of strings into a hashmap holding the grid
fn input_to_grid(input: Vec<String>) -> HashMap<(i32, i32), char> {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    input
        .iter()
        .enumerate()
        .for_each(|(y, s)| {
            s.chars()
                .enumerate()
                .for_each(|(x, ch)| {
                    let c = match ch {
                        '#' => '.',
                        _ => ch,
                    };

                    grid.insert((x as i32, y as i32), c);
        });
    });
    
    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn Error>> {
        let input = read_from_file_as_lines("example1.txt");
        let rows = input.len();
        let cols = input[0].len();

        let grid = input_to_grid(input);
        let result = find_antinodes(&grid, rows, cols);

        Ok(assert_eq!(result, 14))
    }
}