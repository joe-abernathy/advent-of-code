use aoc_helpers::{ * };
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_string(2024, 9)?;
    let (mut diskmap, mut free_space) = parse_input_to_diskmap(input);

    defrag(&mut diskmap, &mut free_space);

    let result = get_checksum(&diskmap);
    println!("{}", result);

    Ok(())
}

// Iterate across all Some values in the diskmap, summing the product of the index and the file ID
fn get_checksum(diskmap: &Vec<Option<u32>>) -> u64 {
    diskmap
        .iter()
        .enumerate()
        .filter(|(_, num)| { num.is_some() })
        .map(|(i, num)| {
            i as u64 * num.unwrap() as u64
        })
        .sum()        
}

// Swap values in the diskmap to remove fragmentation as needed
fn defrag(diskmap: &mut Vec<Option<u32>>, free_space: &mut Vec<(usize, usize)>) {
    let mut current_val: Option<u32> = None;
    let mut count = 0;
    let mut start_index: usize = 0;

    // Iterate across all entries in the diskmap from right to left
    for (i, val) in diskmap.clone().iter().enumerate().rev() {

        match val {

            // If we've reached a Some() value (meaning we're at a file and not free space),
            // and this is the second or subsequent file with this ID that we've read, add 
            // to the count of file blocks and set the start_index to the current index. 
            // Since this iterates backwards, once we get to the first block in the file,
            // start_index will point to the beginning of the file, and count will hold its
            // size.
            Some(v) if Some(*v) == current_val => {
                count += 1;
                start_index = i;
            },

            // If we're reading a file and it's the first block from this file, check for a
            // previously completed file and process it if it exists.
            Some(v) => {
                if current_val.is_some() && count > 0 {
                    process_file(count, start_index, diskmap, free_space);
                }

                // Then start a new file sequence
                current_val = Some(*v);
                count = 1;
                start_index = i;
            },

            // If we get a None value, we're reading free space. Check for a previously
            // completed file and process it if it exists, then set the current_val and 
            // count values to their defaults.
            None => {
                if current_val.is_some() && count > 0 {
                    process_file(count, start_index, diskmap, free_space);
                    current_val = None;
                    count = 0;
                }
            }
        }
    }
}

// Takes a file that we've read in, find the first free space that fits it, and if a valid space exists, move the file
// into that free space.
fn process_file(count: usize, start_index: usize, diskmap: &mut Vec<Option<u32>>, free_space: &mut Vec<(usize, usize)>) {

    // Find the first free block in free_space that's large enough to fit the file. If no such block exists, the
    // file doesn't move.
    if let Some(index) = free_space
        .iter()
        .enumerate()
        .find(|(_, (_, space))| *space >= count)
        .map(|(i, _)| i) 
    {
            // Get the start index of the free space we're moving into
            let (free_index, _) = &mut free_space[index];
            
            // Swap the file and the free blocks in the diskmap vector
            for i in 0..count {
                let left = *free_index + i;
                let right = start_index + i;

                if left < right {
                    diskmap.swap(left, right);
                }
            }

            // Update the free_space vector to reflect the moves
            *free_space = get_free_space_vec(&diskmap);
    }
}

// Takes the long input string and parses it into a diskmap and a vector of free blocks
fn parse_input_to_diskmap(input: String) -> (Vec<Option<u32>>, Vec<(usize, usize)>) {
    let mut diskmap: Vec<Option<u32>> = Vec::new();
    let mut free_space: Vec<(usize, usize)> = Vec::new();
    let mut file_id = 0;

    input.trim()
        .chars()
        .into_iter()
        .enumerate()
        .for_each(|(i, ch)| {
            let n = ch.to_digit(10).expect("Unable to parse char to digit") as usize;

            let val = match i % 2 {
                0 => {
                    file_id += 1;
                    Some(file_id - 1)
                },

                _ => {
                    free_space.push((diskmap.len(), n));
                    None
                },
            };

            for _ in 0..n {
                diskmap.push(val);
            }
        });
    
    let free_space = get_free_space_vec(&diskmap);
    (diskmap, free_space)
}

// Iterates across the diskmap vec and generates a vector representing the free blocks
fn get_free_space_vec(diskmap: &Vec<Option<u32>>) -> Vec<(usize, usize)> {
    let mut free_space: Vec<(usize, usize)> = Vec::new();

    let mut count = 0;
    let mut start_index = 0;

    for (i, num) in diskmap.iter().enumerate() {
        match num {
            Some(_) => {
                if count > 0 {
                    free_space.push((start_index, count));
                }

                count = 0;
            },

            None => {
                if count == 0 {
                    start_index = i;
                }
                count += 1;
            }
        }
    }

    if count > 0 {
        free_space.push((start_index, count));
    }

    free_space
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn Error>> {
        let input = read_from_file_as_string("example.txt");
        let (mut diskmap, mut free_space) = parse_input_to_diskmap(input);
        defrag(&mut diskmap, &mut free_space);

        let result = get_checksum(&diskmap);
        Ok(assert_eq!(result, 2858))
    }
}