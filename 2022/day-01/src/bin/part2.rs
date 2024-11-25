use aoc_helpers::{ * };

fn main() {
    let raw_input = get_input(2022, 1).expect("Failed to get input");
    let lines = raw_input.lines().map(String::from).collect();

    let nums = str_vec_to_ints(lines);
    let result = sum_top_three_cal_totals(nums);

    println!("{}", result);
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

fn sum_top_three_cal_totals(list: Vec<Vec<i32>>) -> i32 {
    let mut totals: Vec<i32> = list.into_iter()
        .map(|entry| entry.into_iter().sum())
        .collect();
    
    totals.sort_unstable_by(|a, b| b.cmp(a));
    totals.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = read_from_file("example1_1.txt");
        let nums = str_vec_to_ints(lines);
        let result = sum_top_three_cal_totals(nums);

        assert_eq!(result, 45000)
    }
}