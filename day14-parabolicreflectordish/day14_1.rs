use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::result::Result;
use std::fmt::Error;
use array2d::{Array2D, Error as A2DErr};


/*
Set up a struct to handle the map, rock types, movement, etc. Part 1 only requires movement to the north, but I'm anticipating
movement in other directions in part 2, so I've implemented all 4 directions.
*/

#[derive(Debug, Clone, PartialEq)]
enum Rock {
    Square,
    Round,
    Empty,
    Invalid,
}


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
}


impl Map {
    fn new(grid: Array2D<Rock>) -> Self {
        let rows = grid.num_rows();

        Map { grid, rows }
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


    fn shift(&mut self, dir: Dir, pos: (usize, usize)) -> Option<(usize, usize)> {
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

    fn find_round_rocks(&self, row: usize) -> Vec<(usize, usize)> {
        let rows = self.grid.as_rows();
        let mut round = Vec::new();

        for (col, rock) in rows[row].iter().enumerate() {
            if *rock == Rock::Round {
                round.push((row, col));
            }
        }
        round
    }

    fn count_round_rocks(&self, row: usize) -> usize {
        let rocks = self.find_round_rocks(row);
        rocks.len()
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
        if pos.0 == 0 {
            Rock::Invalid
        } else {
            self.grid.get(pos.0, pos.1 - 1).unwrap_or(&Rock::Invalid).clone()
        }
    }
}


fn main() {
    let input = map_input("./input.txt").expect("Failed to open file");

    let mut map = Map::new(input);

    let mut total = 0;
    
    // Iterate through the rows in the map and find all the round rocks in each row
    for (i, _) in map.grid.as_rows().iter().enumerate() {
        let round_rocks = map.find_round_rocks(i);

        // For each round rock found, shift it north until it can't go north anymore
        for mut rock in round_rocks {
            loop {
                if let Some(new) = map.shift(Dir::N, rock) {
                    rock = new;
                } else {
                    break;
                }
            }
        }
    }

    for (i, _) in map.grid.as_rows().iter().enumerate() {
        total += (map.rows - i) * map.count_round_rocks(i);
    }

    println!("Total: {}", total);
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