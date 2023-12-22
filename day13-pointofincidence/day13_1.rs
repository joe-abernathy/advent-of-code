use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use array2d::{Array2D};

fn main() {
    let input: Vec<String> = get_input("./input.txt");
    let maps = map_input(input);
    
    let mut total = 0;

    for map in maps {
        total += find_mirror(map);
    }

    println!("{}", total);
}


fn find_mirror(array: Array2D<char>) -> usize {
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
                    //println!("whoops, break (col)");
                    continue 'end;
                } else {
                   // println!("col {} == col {}", i, j);
                }
            }
            //println!("Found mirror on col {}", col + 1);
            return col + 1;
        }
    }
    0
}


fn map_input(input: Vec<String>) -> Vec<Array2D<char>> {
    let mut maps: Vec<Array2D<char>> = vec![];
    let mut map_vec: Vec<Vec<char>> = vec![];

    for line in input {
        if line.is_empty() {
            if let Ok(map) = Array2D::from_rows(&map_vec) {
                maps.push(map);
            }
            map_vec = vec![];
        } else {
            map_vec.push(line.chars().collect());
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
