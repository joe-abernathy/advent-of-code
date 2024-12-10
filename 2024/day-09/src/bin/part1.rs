use aoc_helpers::{ * };
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_string(2024, 9)?;
    let mut diskmap = parse_input_to_diskmap(input);

    defrag(&mut diskmap);

    let result = get_checksum(&diskmap);
    println!("{}", result);
    Ok(())
}

// Iterate across all Some values in the diskmap, summing the product of the index and the file ID
fn get_checksum(diskmap: &Vec<Option<u32>>) -> u64 {
    diskmap
        .iter()
        .enumerate()
        .take_while(|(_, num)| { num.is_some() })
        .map(|(i, num)| {
            i as u64 * num.unwrap() as u64
        })
        .sum()        
}

// Swap values in the diskmap to remove fragmentation as needed
fn defrag(diskmap: &mut Vec<Option<u32>>) {
    let mut left = 0;
    let mut right = diskmap.len() - 1;

    while left < right {
        while diskmap[right].is_none() && left < right {
            right -= 1;
        }

        while diskmap[left].is_some() && left < right {
            left += 1;
        }

        if left < right {
            diskmap.swap(right, left);
        }
    }
}

fn parse_input_to_diskmap(input: String) -> Vec<Option<u32>> {
    let mut diskmap: Vec<Option<u32>> = Vec::new();
    let mut file_id = 0;

    input.trim()
        .chars()
        .into_iter()
        .enumerate()
        .for_each(|(i, ch)| {
            let n = ch.to_digit(10).expect("Unable to parse char to digit");

            let val = match i % 2 {
                0 => {
                    file_id += 1;
                    Some(file_id - 1)
                },
                _ => None,
            };

            for _ in 0..n {
                diskmap.push(val);
            }
        });

    diskmap
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn Error>> {
        let input = read_from_file_as_string("example.txt");
        let mut diskmap = parse_input_to_diskmap(input);
        defrag(&mut diskmap);

        let result = get_checksum(&diskmap);
        Ok(assert_eq!(result, 1928))
    }
}