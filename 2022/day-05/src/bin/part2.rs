use aoc_helpers::{ * };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_puzzle_input_as_lines(2022, 5)?;
    
    // The puzzle input contains a chart of the initial crate positions and a list of
    // moves, split by an empty row.
    let split_index = input.iter().position(|s| s.is_empty()).unwrap();
    let (crates, moves) = input.split_at(split_index);
    
    // Convert the crates into a vector of stacks
    let mut stacks = generate_stacks(crates);

    // Convert the list of moves into a vector of tuples: number of crates, src, dst
    let instructions = parse_moves(moves);
    
    // Go through the list of instructions and make the moves as requested.
    // We have to subtract 1 from the src and dst indices, because the puzzle
    // input is 1-indexed.
    for instr in instructions {
        stacks = move_crates(stacks.clone(), instr.0, instr.1 - 1, instr.2 - 1);
    }

    let result: String = stacks
        .iter()
        .filter_map(|stack| stack.last().copied())
        .collect();

    println!("{}", result);

    Ok(())
}

// Take the vector of stacks, the number of crates to move, and the source and destination
// stacks. Moves the requested number of crates from src to dst, maintaining their order.
// The requirement to maintain the order of the moved crates ended up making it a little more 
// complex than part 1, but not really.
fn move_crates(mut stacks: Vec<Vec<char>>, n: usize, src: usize, dst: usize) -> Vec<Vec<char>> {

    // Since we need to "pop" multiple crates in a single go, we can use the split_off() method.
    // Just need to get the index to split at.
    let split_index = stacks[src].len() - n;
    let ch: Vec<_> = stacks[src].split_off(split_index);

    // Add the crates to the destination stack
    stacks[dst].extend(ch);

    stacks
}

// Parse the list of moves into a vector of tuples with usize values
// corresponding to the number of crates to move, source, and destination.
fn parse_moves(moves: &[String]) -> Vec<(usize, usize, usize)> {
    moves
        .into_iter()
        .skip(1)
        .map(|s| {
            let words: Vec<_> = s.split(' ').collect();
            let x = words[1].parse::<usize>().unwrap();
            let y = words[3].parse::<usize>().unwrap();
            let z = words[5].parse::<usize>().unwrap();
            (x, y, z)
        })
        .collect()
}

// Generate a vector of stacks of characters to represent the stacks of crates
fn generate_stacks(crates: &[String]) -> Vec<Vec<char>> {
    
    // Calculate the number of stacks from the length of a horizontal line
    let num_stacks = ((crates[0].len() - 1) / 4) + 1;

    // Initialize the vector of stacks
    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    // Iterate over the lines in crates, from bottom to top, adding the chars
    // in the correct indices to their respective stacks
    for line in crates.iter().rev().skip(1) {

        // i => index in the string of the current char
        // j => index of the stack that char belongs to
        for i in (1..= (num_stacks * 4)).step_by(4) {
            let j = (i - 1) / 4;
            let ch = line.chars().nth(i).expect("Index in the string seems to be out of range");
            if !ch.is_whitespace() {
                stacks[j].push(ch);
            }
        }

    }
    stacks
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_from_file("example.txt");
        let split_index = input.iter().position(|s| s.is_empty()).unwrap();
        let (crates, moves) = input.split_at(split_index);
        
        let mut stacks = generate_stacks(crates);
        let instructions = parse_moves(moves);
        
        for instr in instructions {
            stacks = move_crates(stacks.clone(), instr.0, instr.1 - 1, instr.2 - 1);
        }

        let result: String = stacks
            .iter()
            .filter_map(|stack| stack.last().copied())
            .collect();

        Ok(assert_eq!(result, "MCD"))
    }
}