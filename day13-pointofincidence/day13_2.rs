use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::cmp;
use array2d::{Array2D};

/*
We store the cells in the input as 1s and 0s, so we can do some binary algebra on them to figure out the number of
differences. For part 1, we needed to find rows/cols that were identical (i.e., with 0 differences), but for part 2, to
find the smudge, we have to find a set of rows/cols with exactly 1 difference. From there, we do some validation to check
whether those two rows/cols are part of a mirrored set, and if so, we find the two center rows/cols in that set and return
the index of the lower of the two. I used the Array2D crate to store the input so I could easily access it by rows or columns.
*/

fn main() {
    let input: Vec<String> = get_input("./input.txt");
    let maps = map_input(input);
    
    let mut total = 0;
    let mut count = 0;

    for map in maps {
        total += fix_smudge(map);
    }
    println!("Total: {}", total);
}


// Since we're storing each cell as a 1 or 0, we can convert rows or columns to binary numbers, xor them,
// and count the number of ones in the result to find the number of differences in the two.
fn diff(line1: u32, line2: u32) -> usize { (line1 ^ line2).count_ones().try_into().unwrap() }


// Converts the provided vector of 1s and 0s into a single integer. This works on a row or a column
fn vec_to_int(arr: Vec<u32>) -> u32 { arr.into_iter().fold(0, |acc, digit| (acc << 1) + digit) }


// Finds the smudge by counting the number of differences between each row. If we find a set of rows
// with exactly 1 difference, we check to see if it's part of a mirror, and if so, return the appropriate
// value. This checks rows and then columns, returning as soon as it finds a valid mirror.
// We also find the original mirror value from part 1 and discard any result that's equal to it,
// because we know the result in this part will be different.
fn fix_smudge(map: Array2D<u32>) -> usize {
    
    let original = find_mirror(map.clone());

    // Try rows first
    let rows = map.as_rows();

    for i in 0..rows.len() - 1 {
        for j in i + 1..rows.len() {
            let line1: u32 = vec_to_int(rows[i].clone());
            let line2: u32 = vec_to_int(rows[j].clone());

            // If there's 1 difference, and this is part of a mirror...
            if diff(line1, line2) == 1 && check_mirror(rows.clone(), i, j) {
                let result = 100 * (1 + i + (j - i) / 2);

                // ...and if the result is different from the part 1 result from this input, we have a winner
                if result != original {
                    return result;
                }
            }
        }
    }

    // Try columns if we didn't find a valid row mirror
    let cols = map.as_columns();

    for i in 0..cols.len() - 1 {
        for j in i + 1..cols.len() {
            let line1: u32 = cols[i].clone().into_iter().fold(0, |acc, digit| (acc << 1) + digit);
            let line2: u32 = cols[j].clone().into_iter().fold(0, |acc, digit| (acc << 1) + digit);

            if diff(line1, line2) == 1 && check_mirror(cols.clone(), i, j) {
                let result = 1 + i + (j - i) / 2;
                if result != original {
                    return result;
                }
            }
        }
    }
    0
}


// This is essentially the algorithm from part 1
fn find_mirror(array: Array2D<u32>) -> usize {
    let rows = array.as_rows();

    'end: for row in 0..rows.len() - 1 {
        if rows[row] == rows[row + 1] {
            let mut i = row;
            let mut j = row + 1;
            while i > 0 && j < rows.len() - 1 {
                i -= 1;
                j += 1;

                if rows[i] != rows[j] {
                    continue 'end;
                } else {
                }
            }
            return 100 * (row + 1);
        }
    }

    let cols = array.as_columns();

    'end: for col in 0..cols.len() - 1 {
        if cols[col] == cols[col + 1] {
            let mut i = col;
            let mut j = col + 1;
            while i > 0 && j < cols.len() - 1 {
                i -= 1;
                j += 1;
                
                if cols[i] != cols[j] {
                    continue 'end;
                }
            }
            return col + 1;
        }
    }
    0
}


// Takes a map (converted to rows or columns) and two indices and checks if these
// indices are part of a valid mirror
fn check_mirror(map: Vec<Vec<u32>>, in1: usize, in2: usize) -> bool {

    // If the distance between the two indices is even, it can't be a valid mirror.
    // Just trust me, the math checks out.
    let dist = in2 - in1;
    
    if dist % 2 == 0{
        return false;
    }

    let mut i: usize;
    let mut j: usize;

    // If the indices differ by 1, these indices are across the axis of reflection
    if dist == 1 {
        i = in1;
        j = in2;

    // Otherwise, the lower index of reflection is at in1 + floor((in2 - in1) / 2)
    // Again, trust me, the math checks out.
    } else {
        i = in1 + (in2 - in1) / 2;
        j = i + 1;
    }


    // Once we have the two rows/cols across the axis of reflection, iterate in both
    // directions verifying that each pair is mirrored. 
    while i >= 0 && j < map.len() { 
        let current_diff = diff(vec_to_int(map[i].clone()), vec_to_int(map[j].clone()));

        // Check the number of differences between the two rows/cols we're checking. If it's 0,
        // they're a match, we're good to go. If it's 1, as long as we're checking the original
        // indices passed to the function, we're good -- that accounts for the smudge. If it's
        // any other value, we don't have a mirror, so return false.
        match current_diff {
            0 => {},
            1 => { 
                if i != in1 && j != in2 {
                    return false;
                }
            },
            _ => return false,
        }

        // Rust doesn't have a concept of a do-while loop, so we have to do a little extra logic
        // to make sure we hit all the right rows/cols and we break at the right time. There's 
        // probably a better way to do this, but here we are. I feel like I say that a lot.
        if i > 0 {
            i -= 1;
        } else {
            break;
        }
        j += 1;
    }
    true
}


// Convert the input maps to a vector of Array2D's
fn map_input(input: Vec<String>) -> Vec<Array2D<u32>> {
    let mut maps: Vec<Array2D<u32>> = Vec::new();
    let mut bin_line: Vec<u32> = Vec::new();
    let mut bin_vec: Vec<Vec<u32>> = Vec::new();

    for line in input {
        if line.is_empty() {
            if let Ok(map) = Array2D::from_rows(&bin_vec) {
                maps.push(map);
                bin_vec = vec![];
            }
        } else {
            for ch in line.chars() {
                match ch {
                    '.' => bin_line.push(0),
                    '#' => bin_line.push(1),
                    _ => continue,
                }
            }
            bin_vec.push(bin_line);
            bin_line = vec![];
        }
    }
    maps
}


// Get the input lines from the input file
fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}
