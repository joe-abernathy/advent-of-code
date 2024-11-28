use aoc_helpers::{ * };

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get puzzle input
    let lines = get_puzzle_input_as_lines(2022, 4)?;

    // Parse the input into a vector of pairs of strings
    let pairs: Vec<(String, String)> = lines
        .into_iter()
        .map(|s| {
            let mut parts = s.split(',');
            let p0: String = parts.next().unwrap().to_string();
            let p1: String = parts.next().unwrap().to_string();
            (p0, p1)
        })
        .collect();

    let result = count_contained_pairs(pairs.clone());
    
    println!("{}", result);
    Ok(())
}

// Parses the string pairs into pairs of pairs of ints, then counts the number of these
// pairs where one contains the other and returns the total.
fn count_contained_pairs(pairs: Vec<(String, String)>) -> u32 {
    let mut running_total = 0;
    let sections = strings_to_pair_vec(pairs);
    
    'outer: for mut section in sections {

        // Sort each range by size so we know which range is smaller
        section = sort_ranges_by_size(section.0, section.1);

        // If any entry in the smaller range is NOT included in the larger range, continue,
        // or if we get to the end without this ever being the case, increment the running total
        for i in section.0.0..=section.0.1 {
            if !(section.1.0..=section.1.1).contains(&i) {
                continue 'outer;
            }
        }
        running_total += 1;
    }
    running_total
}

fn strings_to_pair_vec(pairs: Vec<(String, String)>) -> Vec<((u32, u32), (u32, u32))> {
    pairs
        .into_iter()
        .map(|(l, r)| {
            let s0 = range_to_int_pair(&l);
            let s1 = range_to_int_pair(&r);
            (s0, s1)
        })
    
        .collect()
}

fn range_to_int_pair(range: &str) -> (u32, u32) {
    let mut parts = range.split('-');
    let start = parts.next().unwrap().parse::<u32>().unwrap();
    let end = parts.next().unwrap().parse::<u32>().unwrap();

    (start, end)
}

fn sort_ranges_by_size(range_0: (u32, u32), range_1: (u32, u32)) -> ((u32, u32), (u32, u32)) {
    if (range_0.1 - range_0.0) < (range_1.1 - range_1.0) {
        return (range_0, range_1);
    }

    (range_1, range_0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let lines = read_from_file("example.txt");
        let pairs: Vec<(String, String)> = lines
            .into_iter()
            .map(|s| {
                let mut parts = s.split(',');
                let p0: String = parts.next().unwrap().to_string();
                let p1: String = parts.next().unwrap().to_string();
                (p0, p1)
            })
            .collect();

        let result = count_contained_pairs(pairs.clone());
        Ok(assert_eq!(result, 2))
    }
}