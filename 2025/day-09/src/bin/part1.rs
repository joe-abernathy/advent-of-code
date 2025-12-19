use aoc_helpers::{ * };
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let heap = parse_input()?;

    let now = Instant::now();    
    
    let result = check_pairs(heap);
    println!("Result: {}\nExecution time: {:?}", result, now.elapsed());

    Ok(())
}

fn check_pairs(mut heap: BinaryHeap<Pair>) -> u64 {
    let mut parsed: Vec<(u32, u32)> = Vec::new();

    for _ in 0..heap.len() {
        if let Some(val) = heap.pop() {
            parsed.push((val.width, val.height));
        }
    }

    let max = parsed.iter().max_by_key(|(w, h)| w * h).unwrap();
    max.0 as u64 * max.1 as u64
}

fn parse_input() -> Result<BinaryHeap<Pair>, Box<dyn Error>> {
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 9)?
    };

    let points: Vec<_> = input
        .iter()
        .map(|s| {
            let spl: Vec<_> = s.split(',').collect();
            (spl[0].parse::<u32>().expect("parse, first"), spl[1].parse::<u32>().expect("parse, second"))
        })
        .collect();

    let mut heap: BinaryHeap<Pair> = BinaryHeap::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = (points[i].0, points[i].1);
            let (x2, y2) = (points[j].0, points[j].1);

            let width = ((x2 as i32 - x1 as i32).abs() + 1) as u32;
            let height = ((y2 as i32 - y1 as i32).abs() + 1) as u32;

            heap.push(Pair { width, height });
        }
    }

    Ok(heap)
}


#[derive(Debug)]
struct Pair { 
    width: u32, 
    height: u32, 
}

impl Eq for Pair {}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.width.eq(&other.width)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.width.partial_cmp(&other.width).unwrap()
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}