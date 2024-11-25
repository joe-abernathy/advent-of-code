use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use array2d::{Array2D, Error};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

/*
I used a weird custom implementation of Dijkstra to make this work. The constraint of no more than 3 steps in the same direction was a pain,
but I was able to make it work (sort of) by implementing the grid as a kind of 4D array. The (x, y) dimensions are 1 and 2, the direction used
to get to the current node is 3, and the number of sequential straight-line movements into the current node is dimension 4. Using this, I was
able to build a custom Dijkstra implementation to find the shortest path from the start to the end.

I was honestly shocked that this worked. It takes like 10 minutes to run, which tells me I've done something horribly wrong, but at this point
in the competition, I'm taking the fact that this gets the correct answer and takes less than an hour to run as a huge win. 
*/

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}


#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Ord, PartialOrd)]
struct Node {
    pos: (usize, usize),
    direction: Option<Dir>,
    consecutive_dir: usize,
    cost: u32
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct State {
    node: Node,
    total_cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.node.cost.cmp(&self.total_cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Map {
    grid: Array2D<u32>,
    dist: HashMap<Node, u32>,
    heap: BinaryHeap<State>,
    neighbors: HashMap<Node, Vec<Node>>,
    start_state: State,
    end_pos: (usize, usize),
}

impl Map {
    fn new(grid: Array2D<u32>) -> Self {
        let mut dist: HashMap<Node, u32> = HashMap::new();
        let heap: BinaryHeap<State> = BinaryHeap::new();
        let neighbors: HashMap<Node, Vec<Node>> = HashMap::new();

        for (i, row) in grid.as_rows().iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                for d in vec![Dir::N, Dir::S, Dir::E, Dir::W] {
                    for n in 0..3 {
                        let c = grid.get(i, j).unwrap();
                        let node = Node { 
                            pos: (i, j), 
                            direction: Some(d), 
                            consecutive_dir: n, 
                            cost: *c,
                        };

                        dist.insert(node, u32::MAX);
                    }
                }
            }
        }

        let start_pos = (0, 0);
        let end_pos = (grid.as_rows().len() - 1, grid.as_columns().len() - 1);

        let start_cost = 0;

        let start_node = Node {
            pos: start_pos,
            direction: None,
            consecutive_dir: 0,
            cost: *grid.get(start_pos.0, start_pos.1).unwrap(),
        };

        let start_state = State { node: start_node, total_cost: start_cost };

        dist.insert(start_node, 0);
        
        Map { grid, dist, heap, start_state, end_pos, neighbors }
    }

    fn populate_neighbors(&mut self) {
        for node in self.dist.keys() {
            self.neighbors.insert(*node, self.get_neighbors(*node));
        }
    } 

    fn next_node(&self, current: Node, dir: Dir) -> Option<Node> {
        if (current.direction.is_some() && current.direction.unwrap() == dir && current.consecutive_dir > 1)
            || (current.direction.is_some() && dir == current.direction.unwrap().opposite()) {
                None

        } else {
            match dir {
                Dir::N => {
                    if current.pos.0 == 0 {
                        None
                    } else {
                        let (next_row, next_col) = (current.pos.0 - 1, current.pos.1);
                        let consecutive = if Some(dir) == current.direction {
                            current.consecutive_dir + 1
                        } else {
                            0
                        };

                        let next_cost = self.grid.get(next_row, next_col).unwrap();
                        let next_node = Node { pos: (next_row, next_col), direction: Some(dir), consecutive_dir: consecutive, cost: *next_cost };

                        Some(next_node)
                    }
                },

                Dir::S => {
                    let (next_row, next_col) = (current.pos.0 + 1, current.pos.1);
                    if let Some(next_cost) = self.grid.get(next_row, next_col) {
                        let consecutive = if Some(dir) == current.direction {
                            current.consecutive_dir + 1
                        } else {
                            0
                        };

                        let next_node = Node { pos: (next_row, next_col), direction: Some(dir), consecutive_dir: consecutive, cost: *next_cost };

                        Some(next_node)
                    } else { None }
                },

                Dir::E => {
                    let (next_row, next_col) = (current.pos.0, current.pos.1 + 1);
                    if let Some(next_cost) = self.grid.get(next_row, next_col) {
                        let consecutive = if Some(dir) == current.direction {
                            current.consecutive_dir + 1
                        } else {
                            0
                        };

                        let next_node = Node { pos: (next_row, next_col), direction: Some(dir), consecutive_dir: consecutive, cost: *next_cost };

                        Some(next_node)
                    } else { None }
                },

                Dir::W => {
                    if current.pos.1 == 0 {
                        None
                    } else {
                        let (next_row, next_col) = (current.pos.0, current.pos.1 - 1);
                        let consecutive = if Some(dir) == current.direction {
                            current.consecutive_dir + 1
                        } else {
                            0
                        };

                        let next_cost = self.grid.get(next_row, next_col).unwrap();
                        let next_node = Node { pos: (next_row, next_col), direction: Some(dir), consecutive_dir: consecutive, cost: *next_cost };

                        Some(next_node)
                    }
                }
            }
        }
    }

    fn get_neighbors(&self, current: Node) -> Vec<Node> {
        let mut adj: Vec<Node> = Vec::new();

        let directions: Vec<Dir> = vec![Dir::N, Dir::S, Dir::E, Dir::W];
        for dir in directions {
            if let Some(next) = self.next_node(current, dir) {
                adj.push(next);
            }
        }
        adj
    }

    fn shortest_path(&mut self) -> u32 {
        self.heap.push(self.start_state);

        while let Some(state) = self.heap.pop() {
            let node = state.node;

            if Some(state.total_cost) > self.dist.get(&node).copied() { continue; }

            for neighbor in self.neighbors.get(&node).unwrap() {
                if let Some(cost) = Some(self.dist.get(&node).unwrap() + self.grid.get(neighbor.pos.0, neighbor.pos.1).unwrap()) {
                    if cost < self.dist.get(&neighbor).copied().unwrap() {
                        self.heap.push(State { node: *neighbor, total_cost: cost });
                        self.dist.insert(*neighbor, cost);
                    }
                }
            }
        }

        // After all nodes have been visited, check all nodes with position == end_pos and return the minimum
        let nodes_at_end: Vec<&Node> = self.dist.keys().filter(|&node| node.pos == self.end_pos).collect();
        let end_distances: Vec<&u32> = nodes_at_end.iter().map(|&node| self.dist.get(node).unwrap()).collect();

        end_distances.into_iter().copied().min().unwrap()
    }
}

fn main() {
    let grid = map_input("./input.txt");

    let mut map = Map::new(grid.expect("Failed"));
    map.populate_neighbors();

    println!("{}", map.shortest_path());
}


fn map_input(filename: impl AsRef<Path>) -> Result<Array2D<u32>, Error> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    let input: Vec<_> = buf.lines().map(|l| l.expect("Failed to parse line")).collect();

    let grid_vec: Vec<Vec<u32>> = input
        .iter()
        .map(|row| {
            row.chars().filter_map(|ch| ch.to_digit(10).map(|digit| digit as u32)).collect()
        }).collect();

    Ok(Array2D::from_rows(&grid_vec).expect("Failed to create array"))
}