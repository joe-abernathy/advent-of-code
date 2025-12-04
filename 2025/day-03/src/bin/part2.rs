use aoc_helpers::{ * };
use std::error::Error;
use std::time::Instant;

// This one hurt my brain. My approach was similar to part 1, but instead of just going through the input string once, I used multiple iterations
// to build the 12-digit output, finding the highest digit in the valid slice of the input string. For each iteration, the valid slice of the
// input string starts at the index following the previous max digit (assuming that digit wasn't at the end of the input slice) and ends when
// the distance from the end of the input equals the number of digits remaining to be filled in the output buffer. A separate check reads the 
// last unused digit in the input string to see if it's greater than the max we just calculated, and if so, adds that max to the last available
// entry in the output buffer. 

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_puzzle_input_as_lines(2025, 3)?;
    // let lines = read_from_file_as_lines("example.txt");
    let result = parse_banks(lines);

    // Monitoring execution time
    let now = Instant::now();
    println!("Result: {}, execution time: {:?}", result, now.elapsed());

    Ok(())
}

// Iterate across the input vector and sum the best joltages from each line
fn parse_banks(input: Vec<String>) -> u64 {
    let mut count: u64 = 0;
    
    input
        .iter()
        .for_each(|bank| count += get_best_joltage(bank.to_string()));

    count
}

// Get the best joltage from a single bank
fn get_best_joltage(input: String) -> u64 {

    // Output buffer to build the final joltage value
    let mut output: Vec<u8> = vec![0_u8; 12];

    // Start and end pointers for the input
    let mut in_start = 0_usize;
    let mut in_end = input.len() - 1;

    // Start and end pointers for the output
    let mut out_start = 0_usize;
    let mut out_end = output.len() - 1;

    loop {

        // Tracks the number of digits remaining to be filled in the output
        let remaining_output_digits = out_end - out_start + 1;

        // Tracks the number of digits remaining to be evaluated in the input
        let remaining_input_digits = in_end - in_start + 1;

        // Tracks the last valid index in the input -- essentially, the last index where the number of digits that
        // follow exceeds the number of digits remaining to be filled in the output. If we go past this threshold
        // we'll run out of digits and won't be able to fill the output.
        let max_valid_leading_index = in_end - remaining_output_digits;

        // If there's only one digit remaining in the input, add it to the last spot in the output and break out of
        // the loop. The other checks should ensure that we don't reach this point with more than one digit remaining 
        // to fill in the output.
        if remaining_input_digits == 1 {
            output[out_start] = input.as_bytes()[in_start];
            break; 
        }

        // If there's only one digit left to fill in the output, fill it with the max digit in the current valid input 
        // slice and break out of the loop.
        if remaining_output_digits == 1 {
            output[out_start] = input[in_start ..= in_end].bytes().max().unwrap();
            break;
        }

        // If the number of remaining input digits equals the number of output digits left to fill, we fill the remainng
        // output digits with the remaining input digits in order and break out of the loop.
        if remaining_input_digits == remaining_output_digits {

            let remaining_in = &input.as_bytes()[in_start ..= in_end];
            let remaining_out = &mut output[out_start ..= out_end];

            remaining_out.copy_from_slice(remaining_in);
            break;
        }

        // If none of those ending cases are true, find the max digit in the valid input slice and get its index.
        let max = input[in_start ..= max_valid_leading_index].bytes().max().unwrap();
        let max_i = input[in_start ..= max_valid_leading_index].bytes().position(|x| x == max).unwrap();
        
        // If the last unused digit in the input is greater than the max we just calculated, add it to the last empty
        // spot in the output buffer and decrement the two end pointers
        let last = input.as_bytes()[in_end];
        if last > max {
            output[out_end] = last;
            out_end -= 1;
            in_end -= 1;

        // Otherwise, add the max we just calculated to the first empty spot in the output buffer, increment the output
        // buffer start pointer, and add max_i + 1 to the input start pointer (to make the next cycle's valid slice start
        // with the index following the max we just found).
        } else {
            output[out_start] = max;
            out_start += 1;
            in_start += max_i + 1;
        }
    }
    
    // Convert the output buffer to a string and then to a u64 and return it
    String::from_utf8(output).unwrap().parse::<u64>().expect("Failed to parse final output string to int")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file_as_lines("example.txt");
        let result = parse_banks(input);

        Ok(assert_eq!(result, 3121910778619))
    }
}