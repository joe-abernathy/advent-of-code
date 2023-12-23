use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

fn main() {
    let input = get_input("./input.txt");

    // Initialize the vector of boxes
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];

    // Iterate through the steps in the input and get the values
    for step in input {
        let values = get_values(&step);

        let label = String::from_utf8(values[0].clone()).unwrap();
        let hash = hash(&values[0]);

        // Checks if the second vector in the values vector exists (i.e., if the delimiter
        // is '=' and there's an associated lens)
        if let Some(v) = values[1].last() {

            // If the lens number exists, convert it from its u8 ASCII value to char,
            // then from char to an integer
            let num = (*v as char) as u8 - '0' as u8;

            // If the current label already exists in the box, get its index and update
            // its lens number
            if let Some(index) = boxes[hash].iter().position(|(s, _)| s == &label) {
                boxes[hash][index].1 = num;

            // If the current label doesn't exist, add it
            } else {
                boxes[hash].push((label, num));
            }

        // If the second value doesn't exist (i.e., the delimiter is '-' and there's no 
        // associated lens), check if the current label exists in the appropriate box
        // and remove it if it does
        } else {
            if let Some(index) = boxes[hash].iter().position(|(s, _)| s == &label) {
                boxes[hash].remove(index);
            }
        }
    }

    // Get the focusing power of the completed set of boxes
    println!("{:#?}", focusing_power(boxes));
}


// Calculates the focusing power of the lenses currently in the boxes
fn focusing_power(boxes: Vec<Vec<(String, u8)>>) -> usize {
    let mut total = 0;

    for (i, bx) in boxes.iter().enumerate() {
        for (j, lens) in bx.iter().enumerate() {
            total += (i + 1) * (j + 1) * lens.1 as usize;
        }
    }
    total
}


// Gets the label (as u8 bytes) and the lens number, if it exists.
// We'll end up with a vector of vectors of u8's. The label (as bytes)
// is the first vector, and the lens value is the second. If the delimiter
// is '-', the second vector will be empty. We'll use that later.
fn get_values(step: &[u8]) -> Vec<Vec<u8>> {
    let mut delimiter: u8;
    match step.into_iter().find(|&&x| x == b'=') {
        Some(_) => delimiter = b'=',
        None => delimiter = b'-',
    }

    let label: Vec<Vec<u8>> = step
        .split(|&ch| ch == delimiter)
        .map(|splits| splits.to_vec())
        .collect();

    label
}


// Get the hash value of a given vector of u8's
fn hash(step: &[u8]) -> usize {
    step.iter().fold(0, |acc, &ch| (acc + ch as usize) * 17 % 256)
}


// Get the input steps from the input file
fn get_input(filename: impl AsRef<Path>) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.split(b',').map(|s| s.expect("Failed to parse string")).collect()
}