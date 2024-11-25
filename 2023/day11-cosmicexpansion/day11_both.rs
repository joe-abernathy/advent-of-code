use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

/*
I spent an embarrasing amount of time on this puzzle, trying to implement a BFS algorithm to measure distance between the two
points. I finally realized that these are just Cartesian points and we can just take the Manhattan distance (|x2 - x1| + |y2 - y1|)
to get the distance. It works a lot faster now. Imagine that.

This is my solution to both parts. For part 1, set EXPANSION_RATE to 2, and for part 2, set it to 1000000.
*/


// Each empty row and column is multiplied by the expansion rate
const EXPANSION_RATE: usize = 1000000;

#[derive(Debug)]
struct Map {
    data: Vec<String>,
    rows: usize,
    cols: usize,
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl<'a> Map {
    fn new(data: Vec<String>) -> Self {

        let rows = data.len();
        let cols = data[0].len();

        let empty_rows: Vec<usize> = vec![];
        let empty_cols: Vec<usize> = vec![];

        let galaxies: Vec<(usize, usize)> = vec![];

        Map { data, rows, cols, galaxies, empty_rows, empty_cols }
    }


    // Returns the total distance across all pairs
    fn get_total_distance(&mut self) -> usize {
        self.expand_space();
        self.find_galaxies();

        let mut dist: usize = 0;


        // Iterate through each possible pair
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                let mut p1 = self.galaxies[i];
                let mut p2 = self.galaxies[j];

                // For each single empty column that separates the two points, the distance between them in the
                // x direction is increased by the expansion rate
                for col in &self.empty_cols {
                    if self.galaxies[i].0 < *col && self.galaxies[j].0 > *col {
                        p2.0 += EXPANSION_RATE - 1;

                    } else if self.galaxies[i].0 > *col && self.galaxies[j].0 < *col {
                        p1.0 += EXPANSION_RATE - 1;
                    }
                }

                // Same with rows, but in the y direction
                for row in &self.empty_rows {
                    if self.galaxies[i].1 < *row && self.galaxies[j].1 > *row {
                        p2.1 += EXPANSION_RATE - 1;
                        
                     } else if self.galaxies[i].1 > *row && self.galaxies[j].1 < *row {
                        p1.1 += EXPANSION_RATE - 1;
                    }
                }

                dist += self.measure_distance(p1, p2);
            }
        }
        dist
    }


    // Gets the Manhattan distance between any two points.
    // This function was WAY more complicated when I was trying to implement a BFS
    // algorithm to measure the distance between these two points... whoops
    fn measure_distance(&self, p1: (usize, usize), p2: (usize, usize)) -> usize {
        let x_diff = if p1.0 > p2.0 {p1.0 - p2.0} else {p2.0 - p1.0};
        let y_diff = if p1.1 > p2.1 {p1.1 - p2.1} else {p2.1 - p1.1};

        x_diff + y_diff
    }


    // Checks if the given point is occupied by a galaxy
    fn has_galaxy(&self, pos: (usize, usize)) -> bool {
        let x = pos.0;
        let y = pos.1;

        assert!(x < self.cols, "x out of bounds");
        assert!(y < self.rows, "y out of bounds");

        let row = self.data.get(y);
        row.expect("No such position").chars().nth(x) == Some('#')
    }


    // Checks if a given row is empty
    fn is_row_empty(&self, y: usize) -> bool {
        assert!(y < self.rows, "is_row_empty(): row out of bounds");

        let mut is_empty = true;

        for x in 0..self.cols {
            if self.has_galaxy((x, y)) {
                is_empty = false;
                break;
            }
        }
        is_empty
    }


    // Checks if a given col is empty
    fn is_col_empty(&self, x: usize) -> bool {
        assert!(x < self.cols, "is_col_empty(): col out of bounds");

        let mut is_empty = true;

        for y in 0..self.rows {
            if self.has_galaxy((x, y)) {
                is_empty = false;
                break;
            }
        }
        is_empty
    }

    
    // Collects lists of all empty rows and columns in the map and stores them in
    // the appropriate variables
    fn expand_space(&mut self) {

        for row in 0..self.rows {
            if self.is_row_empty(row) {
                self.empty_rows.push(row);
            }
        }

        for col in 0..self.cols {
            if self.is_col_empty(col) {
                self.empty_cols.push(col);
            }
        }
    }


    // Collects a list of all galaxies in the map
    fn find_galaxies(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                let pos = (x, y);
                if self.has_galaxy(pos) {
                    self.galaxies.push(pos);
                }
            }
        }
    }


}

fn main() {
    let input = get_input("./input.txt");
    let mut map = Map::new(input.clone());

    println!("{}", map.get_total_distance());
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}
