use aoc_helpers::{ * };
use std::error::Error;
use std::collections::HashMap;

// My original idea was to iterate over every valid position in the grid checking if it
// matched the pattern. But I figured we could save some time by doing a little bit of
// preprocessing. A valid pattern has to have an 'A' in the center, so when we generate
// the grid, we check for all 'A' characters that aren't on the outer edge of the grid
// and add their positions to a list of candidates. Then we can just iterate over the
// list of candidates instead of going through all characters. Seems to work all right.

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 4)?;
    let (grid, candidates) = input_to_hashmap(input);

    let result = word_search(&grid, candidates);

    println!("{}", result);
    Ok(())
}

// Iterate through each coordinate in the candidate list, checking if it matches the pattern,
// then return the total number of matches
fn word_search(grid: &HashMap<(i32, i32), char>, candidates: Vec<(i32, i32)>) -> usize {
      candidates
        .iter()
        .filter(|p| {
            check_for_x(&grid, **p)
        })
        .count()
}

// Check a specific X from the candidate list (list of all 'A's in the grid that
// aren't on the outer edge of the grid) and check if it matches the pattern
fn check_for_x(grid: &HashMap<(i32, i32), char>, pos: (i32, i32)) -> bool {
    let (x, y) = pos;

    // Relative (x, y) positions in the left and right diagonals. We don't care
    // about the center since we already know it's an 'A'.
    let l_diag: [(i32, i32); 2] = [(x - 1, y - 1), (x + 1, y + 1)];
    let r_diag: [(i32, i32); 2] = [(x + 1, y - 1), (x - 1, y + 1)];

    // In order to match the pattern, both diagonals have to be "MAS" or "SAM".
    // We already know the center is 'A', so we're checking for "MS" or "SM"
    let check_diagonal = |diag: &[(i32, i32); 2]| -> bool {
        let chars: Vec<char> = diag.iter()
            .filter_map(|&p| grid.get(&p))
            .cloned()
            .collect();
        chars == vec!['S', 'M'] || chars == vec!['M', 'S']
    };

    check_diagonal(&l_diag) && check_diagonal(&r_diag)
}

// Convert the input from a vector of strings into a hashmap with the (x, y)
// coordinates as the key, and the character at those coords as the value, then
// generate a list of all coordinates that could be the center of an X-mas to
// speed up processing later.
fn input_to_hashmap(input: Vec<String>) -> (HashMap<(i32, i32), char>, Vec<(i32, i32)>) {
    let max_x = input.len() - 1;
    let max_y = input[0].len() - 1; 

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut candidates: Vec<(i32, i32)> = Vec::new();

    input
        .iter()
        .enumerate()
        .for_each(|(y, s)| {
            s.chars()
                .enumerate()
                .for_each(|(x, ch)| {
                    grid.insert((x as i32, y as i32), ch);

                    // If the current char is 'A' and it's not on the exterior ring
                    // of the grid, the current position is a candidate for an X-mas,
                    // so add it to the list.
                    if ch == 'A' && x != 0 && x != max_x && y != 0 && y != max_y {
                        candidates.push((x as i32, y as i32));
                    }
                })
        });
        (grid, candidates)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example.txt");

        let (grid, candidates) = input_to_hashmap(input);
        let result = word_search(&grid, candidates);

        Ok(assert_eq!(result, 9))
    }
}