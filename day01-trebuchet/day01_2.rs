use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut first = 0;
    let mut last = 0;
    let mut first_i;
    let mut last_i;
    let mut total: u32 = 0;

    // I know zeros aren't included in this puzzle, but leaving them in makes indexing a lot simpler
    let numbers = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9"
    ];

    for line in reader.lines() {
        let mut positions: Vec<Vec<usize>> = Vec::new();

        first_i = std::usize::MAX;
        last_i = 0;

        // Find any numbers (text or numeric) in the string and record the indices where they appear.
        // For each entry in the numbers list, find any instances of that entry in the given line and
        // store the index in the string where it appears. There can be multiple, so positions is a 
        // vector of vectors of usizes.
        let l = line?.clone();

        for num in numbers {
            positions.push(l.match_indices(num).map(|(i, _)|i).collect());
        }
        
        // Find the first and last entry from the positions vector
        for (i, pos) in positions.iter().enumerate() {
            if pos.len() == 0 { continue; }

        // first_i is initialized to usize::MAX and last_i is initialized to 0 so we can find the min
        // and max values in the position vector to get the first and last numbers in the string. 
        // There's probably a much better way to do this, but here we are. 

        // This gives us the first and last numbers in the string, but as their index in the numbers
        // array. To get the actual number, we take that index mod 10. If we had taken zero out of the
        // array, this would have been a lot messier.
            for x in pos {
                if x < &first_i {
                    first_i = *x;
                    first = i % 10;
                }

                if x >= &last_i {
                    last_i = *x;
                    last = i % 10;
                }
            }
        }

        // Concatenate the first and last numbers and parse them as a single integer
        let s = format!("{}{}", first.to_string(), last.to_string());
        let num = s.parse::<u32>().unwrap();
        total += num;
    }

    println!("{}", total);
    Ok(())
}