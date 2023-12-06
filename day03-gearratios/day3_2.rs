use std::fs::File;
use std::io::{ self, prelude::*, BufReader };

const ROWS: usize = 140;
const COLS: usize = 140;

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

        // inner loop iterates through the characters in each line
        for (col, ch) in line.iter().enumerate() {

            if *ch == '*' {
                let mut is_gear = false;

                // Get a vector of bools that indicate whether the squares surrounding 
                // the current square contain a number
                let surrounding = get_surrounding(&input, row, col);

                // If there are no numbers at all, skip it, this isn't a gear
                if !surrounding.contains(&true) { continue; }

                // Now we have to account for the fact that there could be two digits
                // in the surrounding spaces, but just one number (if there's a 2+ digit
                // number in the three spaces above or below the asterisk)

                // I'm confident there's a better and more efficient way to do this, but here we are
                
                // If there are no digits directly above or below the *, we're not dealing with the
                // case we just talked about, so just check for multiple digits in the surroundings
                if !surrounding[1] && !surrounding[6] {
                    if surrounding.iter().filter(|&t| *t == true).count() > 1 {
                        is_gear = true;
                    }
                }

                // If the space directly above or below the * contains a digit, we ignore the other 
                // spaces above or below the * (because they can't be part of a separate number) and
                // check for at least one digit in the remaining spaces 
                if surrounding[1] {
                    if surrounding[3..].contains(&true) {
                        is_gear = true;
                    }
                }
                if surrounding[6] {
                    if surrounding[..4].contains(&true) {
                        is_gear = true;
                    }
                }

                // Now it gets awkward and cumbersome. Again -- definitely a better way to do this,
                // but this is what I've got.
                if is_gear {
                    let mut num_str: String;
                    let mut num: u32 = 1;

                    // i and j will be used to iterate left and right, respectively
                    let mut i: usize;
                    let mut j: usize;

                    // If there's a digit directly above the *, we have to process the spaces to the
                    // left and right of that first digit to get the full number
                    if surrounding[1] {
                        num_str = "".to_string();
                        i = col;
                        j = col + 1;

                    // Iterate left and right until you hit the beginning or end of the line, or
                    // until you hit a non-numeric character, then parse it into an integer and
                    // multiply it by num to get the result
                        while input[row - 1][i].is_numeric() {
                            num_str.insert_str(0, &input[row - 1][i].to_string());
                            if i == 0 { break; }
                            i -= 1;
                        }
                        while j < COLS && input[row - 1][j].is_numeric() {
                            num_str.push(input[row - 1][j]);
                            j += 1;
                        }
                        num *= num_str.parse::<u32>().unwrap();

                    // If there isn't a digit directly above the *, check the spaces diagonally above
                    // and parse any numbers found in either space 
                    } else {
                        if surrounding[0] {
                            num_str = "".to_string();
                            i = col - 1;
                            while input[row - 1][i].is_numeric() {
                                num_str.insert_str(0, &input[row - 1][i].to_string());
                                if i == 0 { break; }
                                i -= 1;
                            }
                            num *= num_str.parse::<u32>().unwrap();
                        }
                        if surrounding[2] {
                            num_str = "".to_string();
                            j = col + 1;
                            while j < COLS && input[row - 1][j].is_numeric() {
                                num_str.push(input[row - 1][j]);
                                j += 1;
                            }
                            num *= num_str.parse::<u32>().unwrap();
                        }
                    }

                    // A digit directly below the * is handled the same as a digit above
                    if surrounding[6] {
                        num_str = "".to_string();
                        i = col;
                        j = col + 1;

                        while input[row + 1][i].is_numeric() {
                            num_str.insert_str(0, &input[row + 1][i].to_string());
                            if i == 0 { break; }
                            i -= 1;
                        }
                        while j < COLS && input[row + 1][j].is_numeric() {
                            num_str.push(input[row + 1][j]);
                            j += 1;
                        }
                        num *= num_str.parse::<u32>().unwrap();

                    } else {
                        if surrounding[5] {
                            num_str = "".to_string();
                            i = col - 1;
                            while input[row + 1][i].is_numeric() {
                                num_str.insert_str(0, &input[row + 1][i].to_string());
                                if i == 0 { break; }
                                i -= 1;
                            }
                            num *= num_str.parse::<u32>().unwrap();
                        }
                        if surrounding[7] {
                            num_str = "".to_string();
                            j = col + 1;
                            while j < COLS && input[row + 1][j].is_numeric() {
                                num_str.push(input[row + 1][j]);
                                j += 1;
                            }
                            num *= num_str.parse::<u32>().unwrap();
                        }
                    }
                    
                    // Digits to the left and right are handled essentially the same
                    if surrounding[3] {
                        num_str = "".to_string();
                        i = col - 1;
                        while line[i].is_numeric() {
                            num_str.insert_str(0, &line[i].to_string());
                            if i == 0 { break; }
                            i -= 1;
                        }
                        num *= num_str.parse::<u32>().unwrap();                        
                    }

                    if surrounding[4] {
                        num_str = "".to_string();
                        j = col + 1;
                        while j < COLS && line[j].is_numeric() {
                            num_str.push(line[j]);
                            j += 1;
                        }
                        num *= num_str.parse::<u32>().unwrap();
                    }

                    total += num;
                } 
            }
        }
    }

    println!("{}", total);

    Ok(())
}

// get the characters surrounding the current index
fn get_surrounding(input: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<bool> {

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

            if input[r][c].is_numeric() {
                surrounding.push(true);
            } else {
                surrounding.push(false);
            }
        }
    }

    surrounding
}
