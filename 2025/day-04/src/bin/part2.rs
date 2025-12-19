use aoc_helpers::{ * };
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2025, 4)?;
    // let input = read_from_file_as_lines("example.txt");
    
    let mut grid = parse_input(input);

    let now = Instant::now();
    let result = check_grid(grid);

    println!("Result: {}, execution time: {:?}", result, now.elapsed());

    Ok(())
}

fn check_grid(mut grid: Vec<Vec<Cell>>) -> u32 {
    let mut count = 0_u32;

    loop {
        let accessible = get_accessible_rolls(&grid);
        count += accessible.len() as u32;
        
        if accessible.is_empty() { break; }

        update_grid(&mut grid, accessible);
    }

    count
}

fn update_grid(grid: &mut Vec<Vec<Cell>>, removed: Vec<(usize, usize)>) {
    removed.iter().for_each(|(x, y)| grid[*y][*x] = Cell::Empty);    
}

fn get_accessible_rolls(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let mut accessible: Vec<(usize, usize)> = Vec::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if check_pos(&grid, (x, y)) && check_all_dirs(&grid, (x, y)) < 4 {
                accessible.push((x, y));
            }
        }
    }

    accessible
}

fn check_all_dirs(grid: &Vec<Vec<Cell>>, (x, y): (usize, usize)) -> u32 {
    let dirs = [Dir::N, Dir::S, Dir::E, Dir::W, Dir::NE, Dir::NW, Dir::SE, Dir::SW];
    
    dirs.iter()
        .map(|dir| check_dir(grid, (x, y), dir))
        .filter(|b| *b == true)
        .count().try_into().unwrap()
}

fn check_dir(grid: &Vec<Vec<Cell>>, (x, y): (usize, usize), dir: &Dir) -> bool {
    let (x_n, y_n) = match dir {
        Dir::N => {
            if let Some(y_minus_1) = y.checked_sub(1) {
                (x, y_minus_1)
            
            } else { return false; }
        },

        Dir::S => (x, y + 1),

        Dir::E => (x + 1, y),

        Dir::W => {
            if let Some(x_minus_1) = x.checked_sub(1) {
                (x_minus_1, y)
            
            } else { return false; }
        },

        Dir::NE => {
            if let Some(y_minus_1) = y.checked_sub(1) {
                (x + 1, y_minus_1)

            } else { return false; }
        },

        Dir::NW => {
            if let Some(x_minus_1) = x.checked_sub(1) && let Some(y_minus_1) = y.checked_sub(1) {
                (x_minus_1, y_minus_1)

            } else { return false; }
        },

        Dir::SE => (x + 1, y + 1),

        Dir::SW => {
            if let Some(x_minus_1) = x.checked_sub(1) {
                (x_minus_1, y + 1)

            } else { return false; }
        },
    };

    check_pos(&grid, (x_n, y_n))
}

fn check_pos(grid: &Vec<Vec<Cell>>, (x, y): (usize, usize)) -> bool {

    if let Some(cell) = grid.get(y).and_then(|row| row.get(x)) {
        return *cell == Cell::Roll;
    }

    false
}


fn parse_input(input: Vec<String>) -> Vec<Vec<Cell>> {
    input
        .iter().map(|s| s.chars().map(|ch| Cell::try_from(ch).unwrap()).collect())
        .collect::<Vec<Vec<Cell>>>()
}

#[derive(Debug, Eq, PartialEq)]
enum Cell { Empty, Roll }

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Roll),
            _ => Err(())
        }
    }
}

enum Dir { N, S, E, W, NE, NW, SE, SW }