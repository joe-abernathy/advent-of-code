use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::fmt;
use array2d::Array2D;
use std::collections::HashSet;

/*
The cool thing about overengineering part 1 is that part 2 was fairly simple to implement. Just had to add vecs holding the
edge positions and iterate through them. Since we ignore any beam splitters that we've already visited, this is fairly
efficient and runs in less than a second, even though it's technically a bruteforce solution.
*/


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    N,
    S,
    E,
    W,
}


#[derive(Debug, Clone)]
enum Tile {
    MirrorR,
    MirrorL,
    SplitterV,
    SplitterH,
    Empty,
}


#[derive(Debug)]
struct Map {
    grid: Array2D<Tile>,
    energized: HashSet<(usize, usize)>,
    beam_stack: Vec<(Dir, (usize, usize))>,
    splits: HashSet<(usize, usize)>,
    mirrors: HashSet<((usize, usize), Dir)>,
    rows: usize,
    cols: usize,
}


impl Map {
    fn new(input: Vec<String>) -> Self {
        let grid_vec: Vec<Vec<Tile>> = input
            .iter()
            .map(|row| {
                row.chars().map(|ch| match ch {
                    '\\' => Tile::MirrorL,
                    '/' => Tile::MirrorR,
                    '|' => Tile::SplitterV,
                    '-' => Tile::SplitterH,
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                }).collect()
            }).collect();
        
        let grid = Array2D::from_rows(&grid_vec).expect("Failed to create array");
        let energized: HashSet<(usize, usize)> = HashSet::new();
        let beam_stack: Vec<(Dir, (usize, usize))> = Vec::new();
        let splits: HashSet<(usize, usize)> = HashSet::new();
        let mirrors: HashSet<((usize, usize), Dir)> = HashSet::new();
        let rows = grid.num_rows();
        let cols = grid.num_columns();

        Map { grid, energized, beam_stack, splits, mirrors, rows, cols }
    }

    fn get(&self, pos: (usize, usize)) -> Option<Tile> {
        self.grid.get(pos.0, pos.1).cloned()
    }

    fn step(&mut self, dir: Dir, start_pos: (usize, usize)) -> Option<(Dir, (usize, usize), bool)> {
        let mut next_dir = Dir::N;
        let next_pos: (usize, usize);
        let mut split = false;

        match dir {
            Dir::N => {
                if start_pos.0 == 0 {
                    return None;
                } else {
                    next_pos = (start_pos.0 - 1, start_pos.1);
                    if let Some(tile) = self.get(next_pos) {
                        if self.mirrors.contains(&(next_pos, dir)) {
                            return None;
                        } else {
                            self.mirrors.insert((next_pos, dir));
                            (next_dir, split) = self.get_next_dir(dir, tile);
                        }
                    }
                }
            },

            Dir::S => {
                next_pos = (start_pos.0 + 1, start_pos.1);
                if let Some(tile) = self.get(next_pos) {
                    if self.mirrors.contains(&(next_pos, dir)) {
                        return None;
                    } else {
                        self.mirrors.insert((next_pos, dir));
                        (next_dir, split) = self.get_next_dir(dir, tile);
                    }
                } else {
                    return None;
                }
            },

            Dir::E => {
                next_pos = (start_pos.0, start_pos.1 + 1);
                if let Some(tile) = self.get(next_pos) {
                    if self.mirrors.contains(&(next_pos, dir)) {
                        return None;
                    } else {
                        self.mirrors.insert((next_pos, dir));
                        (next_dir, split) = self.get_next_dir(dir, tile);
                    }
                } else {
                    return None;
                }
            },

            Dir::W => {
                if start_pos.1 == 0 {
                    return None;
                } else {
                    next_pos = (start_pos.0, start_pos.1 - 1);
                    if let Some(tile) = self.get(next_pos) {
                        if self.mirrors.contains(&(next_pos, dir)) {
                            return None;
                        } else {
                            self.mirrors.insert((next_pos, dir));
                            (next_dir, split) = self.get_next_dir(dir.clone(), tile);
                        }
                    } else {
                        return None;
                    }
                }
            },
        }
        Some((next_dir, next_pos, split))
    }

    fn get_next_dir(&self, dir: Dir, tile: Tile) -> (Dir, bool) {
        match tile {
            Tile::MirrorR => match dir {
                Dir::N => (Dir::E, false),
                Dir::S => (Dir::W, false),
                Dir::E => (Dir::N, false),
                Dir::W => (Dir::S, false),
            },

            Tile::MirrorL => match dir {
                Dir::N => (Dir::W, false),
                Dir::S => (Dir::E, false),
                Dir::E => (Dir::S, false),
                Dir::W => (Dir::N, false),
            },

            Tile::SplitterH => match dir {
                Dir::N | Dir::S => (Dir::E, true),
                Dir::E | Dir::W => (dir.clone(), false),
            },

            Tile::SplitterV => match dir {
                Dir::N | Dir::S => (dir.clone(), false),
                Dir::E | Dir::W => (Dir::N, true),
            },

            Tile::Empty => (dir.clone(), false),
        }
    }

    fn traverse(&mut self, dir: Dir, pos: (usize, usize)) {
        let mut current_dir = dir;
        let mut current_pos = pos;

        loop {

            self.energize(current_pos);        
            if let Some((next_dir, next_pos, split)) = self.step(current_dir, current_pos) {

                //println!("{:?}/{:?} -> {:?}/{:?}", current_dir, current_pos, next_dir, next_pos);
                current_dir = next_dir;
                current_pos = next_pos;
    
                if split {
                    let spawn_dir: Dir = match current_dir {
                        Dir::N => Dir::S,
                        Dir::E => Dir::W,
                        _ => unreachable!(),
                    };
                    self.spawn_beam(current_pos, spawn_dir);
                }
            } else {
                break;
            }
        }
        if !self.beam_stack.is_empty() {
            self.next_beam();
        }
    }

    fn energize(&mut self, pos: (usize, usize)) { 
        if !self.energized.contains(&pos) {
            self.energized.insert(pos); 
        }
    }

    fn spawn_beam(&mut self, pos: (usize, usize), dir: Dir) {
        if !self.splits.contains(&pos) {
            self.splits.insert(pos);
            self.beam_stack.push((dir, pos));
        }
    }

    fn next_beam(&mut self) {
        let (dir, pos) = self.beam_stack.pop().unwrap();
        self.traverse(dir, pos);
    }
}

fn main() {
    let input = get_input("./input.txt");
    let mut map = Map::new(input.clone());

    let mut top_edge: Vec<(usize, usize)> = Vec::new();
    let mut bottom_edge: Vec<(usize, usize)> = Vec::new();
    let mut left_edge: Vec<(usize, usize)> = Vec::new();
    let mut right_edge: Vec<(usize, usize)> = Vec::new();

    for row in 0..map.rows - 1 {
        left_edge.push((row, 0));
        right_edge.push((row, map.cols - 1));
    }    

    for col in 0..map.cols - 1 {
        top_edge.push((0, col));
        bottom_edge.push((map.rows - 1, col));
    }

    let mut max_tiles = 0;

    let edges = [&top_edge, &bottom_edge, &left_edge, &right_edge];

    for (i, edge) in edges.iter().enumerate() {
        let dir = match i {
            0 => Dir::S,
            1 => Dir::N,
            2 => Dir::E,
            3 => Dir::W,
            _ => unreachable!(),
        };

        for pos in *edge {
            map = Map::new(input.clone());
            
            if let Some(start_tile) = map.get(*pos) {
                let (start_dir, _) = map.get_next_dir(dir, start_tile);
                map.traverse(start_dir, *pos);
                let energized = map.energized.len();
                if energized > max_tiles {
                    max_tiles = energized;
                }
            } else {
                println!("Couldn't start at {:?}", pos);
            }
        }
    }

    println!("Max: {}", max_tiles);
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}