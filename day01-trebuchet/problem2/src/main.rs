use std::fs::File;
use std::io::{ self, prelude::*, BufReader };

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut first;
    let mut last;
    let mut first_i = 0;
    let mut last_i = 0;
    let mut total: u32 = 0;

    // I know zero isn't part of this, but it makes the math a lot simpler later if I leave the zeros in
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
        let l = line?.clone();
        
        first = std::usize::MAX;
        last = 0;

        for num in numbers {
            positions.push(l.match_indices(num).map(|(i, _)|i).collect());
        }

        for (i, pos) in positions.iter().enumerate() {
            if pos.len() == 0 { continue; }

            for x in pos {
                if x < &first {
                    first = *x;
                    first_i = i % 10;
                }

                if x >= &last {
                    last = *x;
                    last_i = i % 10;
                }
            }
        }

        let s = format!("{}{}", first_i.to_string(), last_i.to_string());
        let int = s.parse::<u32>().unwrap();
        total += int;
    }

    println!("{}", total);
    Ok(())
}