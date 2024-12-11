use aoc_helpers::{ * };
use std::error::Error;
use std::collections::{ VecDeque, HashMap };

// Looks like it's time for my semi-annual re-learning of pathfinding algorithms. Went with BFS for this one.

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2024, 10)?;
    let result = find_trails(input);

    println!("{}",  result);
    Ok(())
}

// Iterates over all possible starting points in the graph ('0' nodes) and runs the BFS algorithm on each,
// returning the total number of valid paths from 0 to 9
fn find_trails(input: Vec<String>) -> usize {
    let (grid, starting_points) = input_to_grid(input);
    
    starting_points
        .iter()
        .map(|source| find_paths(&grid, *source) )
        .sum()
}

// Run BFS from a single source node and return the number of valid paths (0 to 9) from that source
fn find_paths(grid: &HashMap<(i32, i32), u8>, source: (i32, i32)) -> usize {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut explored: HashMap<(i32, i32), u32> = HashMap::new();
    
    let mut distance = 0;
    queue.push_back(source);
    explored.insert(source, distance);

    while let Some(node) = queue.pop_front() {
        distance += 1;

        let neighbors = get_neighbors(&grid, node);
        neighbors
            .iter()
            .for_each(|n| {
                if explored.get(n).is_none() {
                    queue.push_back(*n);
                    explored.insert(*n, distance);
                }
        });
    }

    // Count the number of explored nodes whose value is 9 (number of complete paths from
    // this node) and return the result
    explored
        .iter()
        .filter(|(node, _)| {
            grid.get(node) == Some(&9)
        })
        .count()

}   

// Get all valid neighbors from a given node
fn get_neighbors(grid: &HashMap<(i32, i32), u8>, node: (i32, i32)) -> Vec<(i32, i32)> {
    let orig_score: u8 = *grid.get(&node).unwrap();
    
    // Check all 4 cardinal directions and return a vector of results that are within the
    // grid and have a score 1 higher than the previous node
    [Dir::N, Dir::S, Dir::E, Dir::W]
        .iter()
        .map(|dir| {
            dir.step(node)
        })
        .filter(|neighbor| {
            let score = grid.get(&neighbor);
            score.is_some() && *score.unwrap() == orig_score + 1
        })
        .collect()
}

// Parse the input into a grid (hashmap) and a list of trailheads ('0' nodes)
fn input_to_grid(input: Vec<String>) -> (HashMap<(i32, i32), u8>, Vec<(i32, i32)>) {
    let mut grid: HashMap<(i32, i32), u8> = HashMap::new();
    let mut starting_points: Vec<(i32, i32)> = Vec::new();

    input
        .iter()
        .enumerate()
        .for_each(|(y, s)| {
            s.chars()
                .enumerate()
                .for_each(|(x, ch)| {
                    let num= ch.to_digit(10).expect("Failed to parse char to digit") as u8;
                    grid.insert((x as i32, y as i32), num);

                    if num == 0 {
                        starting_points.push((x as i32, y as i32));
                    }
            })
        });

    (grid, starting_points)
}


#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl<'a> Dir {
    fn step(&self, node: (i32, i32)) -> (i32, i32) {
        match self {
            Dir::N => (node.0, node.1 - 1),
            Dir::S => (node.0, node.1 + 1),
            Dir::E => (node.0 + 1, node.1),
            Dir::W => (node.0 - 1, node.1),
        }
    }
}