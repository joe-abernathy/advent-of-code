use std::fs::File;
use std::io::{ self, prelude::*, BufReader };

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut input: Vec<_> = vec![];
    let mut total: u32 = 0;
    
    // read each line into a vector of vectors of characters
    for line in reader.lines() {
        let chars: Vec<char> = line?.chars().collect();
        input.push(chars);
    }

    // outer loop iterates through the lines
    for (row, line) in input.iter().enumerate() {

        // this will hold each number we come across
        let mut num_str: String = "".to_string();

        // set to true if the current number is found to be a part number
        let mut is_part = false;

        // inner loop iterates through the characters in each line
        for (col, ch) in line.iter().enumerate() {

            // if the current character is not a number, we skip it
            if ch.is_numeric() {

                // append the current digit to the number string
                num_str += &ch.to_string();

                // if we haven't yet determined that the current number is a part number, check the surrounding characters to see if they contain a symbol
                if !is_part {
                    let surrounding = get_surrounding(&input, row, col);
                    if contains_symbol(surrounding) { 
                        is_part = true;
                    }
                }

                // if we've reached the end of the line, or the next character is NOT a number, we've finished reading the current number
                if col == line.len() - 1 || !line[col + 1].is_numeric() {

                    // if we've determined this is a part number, parse it from a string to u32 and add it to the running total
                    if is_part {
                        let num = num_str.parse::<u32>().unwrap();
                        total += num;
                        is_part = false;
                    }

                    // reset the number string for the next one
                    num_str = "".to_string();
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}

// checks a given vector to see if it contains a symbol
fn contains_symbol(chars: Vec<char>) -> bool {
    for ch in chars {
        if !ch.is_numeric() && ch != '.' { return true; }
    }

    false
}

// get the characters surrounding the current index
fn get_surrounding(input: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
    let rows = input.len();
    let cols = input[0].len();

    let deltas = vec![-1, 0, 1];
    let mut surrounding = Vec::new();

    // check the indices -1, +0, and +1 from the current index
    for &i in &deltas {
        for &j in &deltas {

            // skip the current index
            if i == 0 && j == 0 { continue; }

            // ensure we don't underflow when we try to subtract 1 if row==0 or col==0
            let r = match (row as isize).checked_add(i) {
                Some(x) => x as usize,
                None => { continue; }
            };
            let c = match (col as isize).checked_add(j) {
                Some(x) => x as usize,
                None => { continue; }
            };

            // ensure we don't try to read past the end of the input
            if r < rows && c < cols {
                surrounding.push(input[r][c]);
            }
        }
    }

    surrounding
}
