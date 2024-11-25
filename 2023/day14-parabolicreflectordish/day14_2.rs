use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::result::Result;
use std::fmt::Error;
use array2d::{Array2D, Error as A2DErr};
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rock {
    Square,
    Round,
    Empty,
    Invalid,
}

#[derive(Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}


#[derive(Debug)]
struct Map {
    grid: Array2D<Rock>,
    rows: usize,
    cols: usize,
}


impl Map {
    fn new(grid: Array2D<Rock>) -> Self {
        let rows = grid.num_rows();
        let cols = grid.num_columns();

        Map { grid, rows, cols }
    }


    fn spin_cycle(&mut self) {
        for dir in [Dir::N, Dir::W, Dir::S, Dir::E] {
            match dir {
                Dir::N => {
                    for i in 0..self.rows {
                        let round_rocks = self.find_round_rocks_row(i);
                        for rock in round_rocks {
                            self.tilt(Dir::N, rock);
                        }
                    }
                },

                Dir::W => {
                    for i in 0..self.cols {
                        let round_rocks = self.find_round_rocks_col(i);
                        for rock in round_rocks {
                            self.tilt(Dir::W, rock);
                        }
                    }
                },

                Dir::S => {
                    for i in (0..self.rows).rev() {
                        let round_rocks = self.find_round_rocks_row(i);
                        for rock in round_rocks {
                            self.tilt(Dir::S, rock);
                        }
                    }
                },

                Dir::E => {
                    for i in (0..self.cols).rev() {
                        let round_rocks = self.find_round_rocks_col(i);
                        for rock in round_rocks {
                            self.tilt(Dir::E, rock);
                        }
                    }
                }
            }
        }
    }

    fn get(&self, pos: (usize, usize)) -> Rock {
        if let Some(rock) = self.grid.get(pos.0, pos.1).cloned() {
            rock
        } else {
            Rock::Invalid
        }
    }

    fn set(&mut self, pos: (usize, usize), rock: Rock) -> Result<(), A2DErr> {
        self.grid.set(pos.0, pos.1, rock)
    }

    fn tilt(&mut self, dir: Dir, start_pos: (usize, usize)) {
        let mut rock = start_pos;
        loop {
            if let Some(new) = self.shift_once(dir, rock) {
                rock = new;
            } else {
                break;
            }
        }
    }

    fn shift_once(&mut self, dir: Dir, pos: (usize, usize)) -> Option<(usize, usize)> {
        let new_pos: (usize, usize);

        match dir {
            Dir::N => {
                match self.north(pos) {
                    Some(new) => { new_pos = new; },
                    None => return None,
                }
            },

            Dir::S => {
                match self.south(pos) {
                    Some(new) => { new_pos = new; },
                    None => return None,
                }
            },

            Dir::E => {
                match self.east(pos) {
                    Some(new) => { new_pos = new; },
                    None => return None,
                }
            },

            Dir::W => {
                match self.west(pos) {
                    Some(new) => { new_pos = new; },
                    None => return None,
                }
            },
        }

        let current_rock = self.get(pos);
        let _ = self.set(new_pos, current_rock);
        let _ = self.set(pos, Rock::Empty);
        Some(new_pos)
    }

    fn find_round_rocks_row(&self, row: usize) -> Vec<(usize, usize)> {
        let rows = self.grid.as_rows();
        let mut round = Vec::new();

        for (col, rock) in rows[row].iter().enumerate() {
            if *rock == Rock::Round {
                round.push((row, col));
            }
        }
        round
    }

    fn find_round_rocks_col(&self, col: usize) -> Vec<(usize, usize)> {
        let cols = self.grid.as_columns();
        let mut round = Vec::new();

        for (row, rock) in cols[col].iter().enumerate() {
            if *rock == Rock::Round {
                round.push((row, col));
            }
        }
        round
    }

    fn count_round_rocks(&self, row: usize) -> usize {
        let rocks = self.find_round_rocks_row(row);
        rocks.len()
    }

    fn calculate_load(&self) -> usize {
        (0..self.rows).map(|i| (self.rows - i) * self.count_round_rocks(i)).sum()
    }

    fn north(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self.check_north(pos) {
            Rock::Empty => Some((pos.0 - 1, pos.1)),
            _ => None,
        }
    }

    fn south(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self.check_south(pos) {
            Rock::Empty => Some((pos.0 + 1, pos.1)),
            _ => None,
        }
    }

    fn east(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self.check_east(pos) {
            Rock::Empty => Some((pos.0, pos.1 + 1)),
            _ => None
        }
    }

    fn west(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self.check_west(pos) {
            Rock::Empty => Some((pos.0, pos.1 - 1)),
            _ => None,
        }
    }

    fn check_north(&self, pos: (usize, usize)) -> Rock {
        if pos.0 == 0 {
            Rock::Invalid
        } else {
            self.grid.get(pos.0 - 1, pos.1).unwrap_or(&Rock::Invalid).clone()
        }
    }

    fn check_south(&self, pos: (usize, usize)) -> Rock {
        self.grid.get(pos.0 + 1, pos.1).unwrap_or(&Rock::Invalid).clone()
    }
    
    fn check_east(&self, pos: (usize, usize)) -> Rock {
        self.grid.get(pos.0, pos.1 + 1).unwrap_or(&Rock::Invalid).clone()
    }

    fn check_west(&self, pos: (usize, usize)) -> Rock {
        if pos.1 == 0 {
            Rock::Invalid
        } else {
            self.grid.get(pos.0, pos.1 - 1).unwrap_or(&Rock::Invalid).clone()
        }
    }
}


fn main() {
    let input = map_input("./input.txt").expect("Failed to open file");

    let mut map = Map::new(input.clone());

    let mut rev_hmap: HashMap<usize, usize> = HashMap::new();
    let mut hmap: HashMap<Array2D<Rock>, usize> = HashMap::new();
    let mut cycle_start: Option<usize> = None;
    let mut cycle_first: usize = 0;
    let mut cycle_end: usize = 0;

    let num_iters = 1000000000;
    for i in 0..num_iters {
        map.spin_cycle();
        rev_hmap.insert(i, map.calculate_load());
        if cycle_start.is_none() {
            if hmap.contains_key(&map.grid) {
                cycle_start = hmap.get(&map.grid).copied();
                cycle_first = i;
            } else {
                hmap.insert(map.grid.clone(), i);
            }
        } else {
            if hmap.get(&map.grid) == cycle_start.as_ref() {
                cycle_end = i;
                break;
            }
            hmap.insert(map.grid.clone(), i);
        }
    }

    let cycle_length = cycle_end - cycle_first - 1;
    let index = (num_iters - cycle_start.unwrap()) % cycle_length + cycle_start.unwrap();

    println!("Total: {:?}", rev_hmap.get(&index));
}   


fn map_input(filename: impl AsRef<Path>) -> Result<Array2D<Rock>, Error> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    let input: Vec<_> = buf.lines().map(|l| l.expect("Failed to parse line")).collect();

    let grid_vec: Vec<Vec<Rock>> = input
        .iter()
        .map(|row| {
            row.chars().map(|ch| match ch {
                '#' => Rock::Square,
                'O' => Rock::Round,
                '.' => Rock::Empty,
                _ => unreachable!(),
            }).collect()
        }).collect();

    Ok(Array2D::from_rows(&grid_vec).expect("Failed to create array"))
}