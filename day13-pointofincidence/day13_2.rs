use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::cmp;
use array2d::{Array2D};

fn main() {
    let input: Vec<String> = get_input("./input.txt");
    let maps = map_input(input);
    
    let mut total = 0;
    let mut count = 0;

    for map in maps {
        count += 1;
        println!("#{}", count);
        let current = fix_smudge(map);
        if current == 0 {println!("ZERO");}
        total += current;
    }
    println!("Total: {}", total);
}


fn diff(line1: u32, line2: u32) -> usize { (line1 ^ line2).count_ones().try_into().unwrap() }


fn fix_smudge(map: Array2D<u32>) -> usize {
    
    let original = find_mirror(map.clone());
    //println!("original: {}", original);

    let rows = map.as_rows();

    for i in 0..rows.len() - 1 {
        for j in i + 1..rows.len() {
            let line1: u32 = rows[i].clone().into_iter().fold(0, |acc, digit| (acc << 1) + digit);
            let line2: u32 = rows[j].clone().into_iter().fold(0, |acc, digit| (acc << 1) + digit);

            if diff(line1, line2) == 1 && check_mirror(rows.clone(), i, j) {
                let result = 100 * (1 + i + (j - i) / 2);
                if result != original {
                    return result;
                }
            }
        }
    }

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


fn check_mirror(map: Vec<Vec<u32>>, in1: usize, in2: usize) -> bool {
    let dist = in2 - in1;
    
    if dist % 2 == 0{
        return false;
    }

    let mut i: usize;
    let mut j: usize;

    if dist == 1 {
        i = in1;
        j = in2;
    } else {
        i = in1 + (in2 - in1) / 2;
        j = i + 1;
    }


    // println!("orig: ({}, {})", in1, in2);
    // println!("i, j: ({}, {})", i, j);
    while i > 0 && j < map.len() - 1 {
        i -= 1;
        j += 1;

        // println!("{:?} =?= {:?}", map[i], map[j]);
        if i != in1 && j != in2 && map[i] != map[j] {
            // println!("false");
            return false;
        }
    }
    // println!("true");
    true
}


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
