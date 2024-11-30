use aoc_helpers::{ * };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = get_puzzle_input_as_lines(2022, 1)?;
    let nums = str_vec_to_ints(lines);

    println!("{}", get_max_calories(nums));
    
    Ok(())
}

fn str_vec_to_ints(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();

    for line in input {
        if !line.is_empty() {
            let int = line.parse::<i32>().unwrap();
            tmp.push(int);
        
        } else {
            numbers.push(tmp.clone());
            tmp = Vec::new();
        }
    }

    if !numbers.is_empty() {
        numbers.push(tmp.clone());
    }

    numbers
}

fn get_max_calories(list: Vec<Vec<i32>>) -> i32 {
    list.into_iter()
        .map(|entry| entry.into_iter().sum())
        .max()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = read_from_file_as_lines("example1_1.txt");
        let nums = str_vec_to_ints(lines);
        let result = get_max_calories(nums);
        assert_eq!(result, 24000)
    }
}