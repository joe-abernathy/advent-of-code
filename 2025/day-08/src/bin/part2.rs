use aoc_helpers::{ * };
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::Instant;
use union_find::{ UnionFind, UnionBySize, QuickUnionUf };

// Part 2 only required a little bit of re-working thanks to the union-find data structure I used,
// which I definitely knew of off the top of my head and didn't have to do any research at all to
// find. 

// Okay, that's false, I had never heard of a union-find before this, but that's the cool thing 
// about AoC. I'm learning all sorts of cool shit.

fn main() -> Result<(), Box<dyn Error>> {

    let (coords, heap) = parse_input()?;

    let now = Instant::now();
    println!("Result: {}, execution time: {:?}", find_circuits(coords, heap), now.elapsed());

    Ok(())
}

// Uses a union-find structure to keep track of the number of circuits and the elements in each
fn find_circuits(coords: Vec<(u32, u32, u32)>, mut heap: BinaryHeap<Pair>) -> u64 {
    let mut uf = QuickUnionUf::<UnionBySize>::new(coords.len());

    // Loop until the number of non-empty circuits is 1 -- in other words, until all nodes are
    // in the same circuit. When that happens, return the product of the X coordinates for the
    // two nodes that caused all nodes to be connected together.
    loop {
        if let Some(val) = heap.pop() {
            uf.union(val.i, val.j);

            if count_circuits(&mut uf) == 1 {
                let (i, j) = (coords[val.i], coords[val.j]);
                return i.0 as u64 * j.0 as u64;
            }
        }
    }
}

// Returns the total number of circuits with at least 1 node
fn count_circuits(uf: &mut QuickUnionUf<UnionBySize>) -> u32 {
    
    // The number of iterations depends on whether we're looking at the example input or the real puzzle input
    let len: usize = if cfg!(feature = "test") {
        20
    } else {
        1000
    };

    let mut counts = vec![0u32; len];

    // Totals the number of times each root node is seen in the UF structure
    for i in 0..len {
        let root = uf.find(i);
        counts[root] += 1;
    }

    // Return the total number of circuits with at least 1 entry
    counts.iter().filter(|&x| *x > 0).count() as u32
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
            let spl: Vec<_> = s.split(',').collect();
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

// Calculates the distance between two points in 3D space
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

