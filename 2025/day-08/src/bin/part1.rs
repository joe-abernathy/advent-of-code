use aoc_helpers::{ * };
use std::error::Error;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::time::Instant;
use union_find::{ UnionFind, UnionBySize, QuickUnionUf };

// My approach here was to create a Pair struct that holds the indices of two nodes and their distance
// apart. Then use a binary heap to collect those and sort by shortest distance so I could pop the nodes
// in order by closest together. Then I used a union-find structure to handle adding them to circuits. 

fn main() -> Result<(), Box<dyn Error>> {

    let (coords, heap) = parse_input()?;

    let now = Instant::now();
    println!("Result: {}, execution time: {:?}", find_circuits(coords.len(), heap), now.elapsed());

    Ok(())
}

// 
fn find_circuits(node_count: usize, mut heap: BinaryHeap<Pair>) -> u32 {
    let mut uf = QuickUnionUf::<UnionBySize>::new(node_count);

    // The number of iterations here depends on whether we're looking at the real puzzle input or
    // the example input. The example specified 10 iterations, and the real input asks for 1000.
    let len = if cfg!(feature = "test") {
        10
    } else {
        1000
    };

    // Pop each pair from the binary heap, sorted by distance ascending, and if the two nodes in the given
    // pair aren't already in the same circuit, add them to the same circuit.
    for _ in 0..len {
        if let Some(val) = heap.pop() {
            if uf.find(val.i) != uf.find(val.j) {
                uf.union(val.i, val.j);
            }
        }
    }

    get_largest_circuits(uf)
}

// Counts the nodes in each circuit and returns the product of the three largest nodes
fn get_largest_circuits(mut uf: QuickUnionUf<UnionBySize>) -> u32 {
    
    let len: usize = if cfg!(feature = "test") {
        20
    } else {
        1000
    };

    // Initialize a vector of zeros with the same length as the number of nodes in the 
    // input (20 if we're looking at the example input, 1000 for the real input)
    let mut counts = vec![0usize; len];

    // A union-find structure stores parent nodes for each index. For example, if nodes
    // 3, 4, and 5 are in the same circuit, they would all have the same parent node 
    // (3 in this case, because this UF implementation sorts by size... I think). So
    // we iterate across all entries and increment the index in the counts vector that
    // corresponds to the current entry's root node.
    for i in 0..len {
        let root = uf.find(i);
        counts[root] += 1;
    }

    // Sort the counts vector from largest to smallest, then multiply the number of nodes
    // in the largest 3 and return the result
    counts.sort_by(|a, b| b.cmp(&a));

    counts[0..3].iter().product::<usize>() as u32
}

fn parse_input() -> Result<(Vec<(u32, u32, u32)>, BinaryHeap<Pair>), Box<dyn Error>> {
    let input: Vec<String> = if cfg!(feature = "test") {
        read_from_file_as_lines("example.txt")
    } else {
        get_puzzle_input_as_lines(2025, 8)?
    };

    let coords: Vec<(u32, u32, u32)> = input
        .iter()
        .map(|s| {
            let mut spl: Vec<_> = s.split(',').collect();
            ( spl[0].parse::<u32>().unwrap(), spl[1].parse::<u32>().unwrap(), spl[2].parse::<u32>().unwrap() )
        })
        .collect();

    let mut heap: BinaryHeap<Pair> = BinaryHeap::new();

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let dist = get_distance(coords[i], coords[j]);
            heap.push(Pair { dist, i, j });
        }
    }

    Ok((coords, heap))
    }


fn get_distance((x1, y1, z1): (u32, u32, u32), (x2, y2, z2): (u32, u32, u32)) -> f32 {
    let dx = x2 as f32 - x1 as f32;
    let dy = y2 as f32 - y1 as f32;
    let dz = z2 as f32 - z1 as f32;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[derive(Debug)]
struct Pair {
    dist: f32,
    i: usize,
    j: usize,
}

impl Pair {
    fn new(dist: f32, i: usize, j: usize) -> Self {
        Self { dist, i, j }
    }
}

impl Eq for Pair {}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.partial_cmp(&self.dist).unwrap()
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

