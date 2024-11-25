use std::fs::File;
use std::io::{ self, prelude::*, BufReader };

fn main() -> io::Result<()> {

    // open the file containing input and read the lines
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut first: Option<char>;
    let mut last: Option<char>;
    let mut total: u32 = 0;

    for line in reader.lines() {
        first = None;
        last = None;

        // iterate through the characters in the current line and look for numbers
        for c in line?.chars() {
            if c.is_numeric() {

                // if 'first' is uninitialized, this is the first numeric character in the line, so set 'first'
                if first == None { first = Some(c); }

                // every time you come across a number, set 'last' to that number
                // this handles the case where there's only one digit in the string, so first == last
                last = Some(c);

            }
        }

        // concatenate the first and last characters and parse them into an integer
        let s = format!("{}{}", first.unwrap(), last.unwrap());
        let num = s.parse::<u32>().unwrap();

        // add the current 2-digit number to the running total
        total += num;
    }

    println!("{}", total);
    Ok(())
}