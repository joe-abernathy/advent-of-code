use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

/* 
Well, this one almost beat me. After hours of research and bashing my head against the wall, I was so close -- I had (mostly) figured out the ray casting algorithm, 
I got the correct answer on two of the four test cases, but I couldn't figure out how the hell to handle the "squeeze-through" problem. Not ashamed to admit I got
some help on this one to correct my algorithm. Instead of toggling the ray casting boolean every time I hit a pipe segment perpendicular to the direction of traversal
like I had been doing, I had to toggle only on pipe segments with a north-facing side (for traversal to the east) -- '|', 'L', or 'J'. I still don't quite understand
why that works, but it does, and my noob self is just proud to have gotten this far with this amount of help. 
*/


#[derive(Debug)]
struct Map {
    data: Vec<String>,
    start_pos: (usize, usize),
    loop_pipes: Vec<(usize, usize)>,
    s_shape: char,
    rows: usize,
    cols: usize,
}

impl<'a> Map {
    fn new(data: Vec<String>) -> Self {
        let loop_pipes: Vec<(usize, usize)> = vec![];
        let s_shape: char = '_';

        let rows = data.len();
        let cols = data[0].len();

        let mut x: usize = 0;
        let mut y: usize = 0;

        for (i, line) in data.iter().enumerate() {
            if line.find('S').is_none() { continue; }
            x = line.find('S').unwrap();
            y = i;
            break;
        }

        let start_pos = (x, y);

        Map { data, start_pos, loop_pipes, s_shape, rows, cols }
    }

    // Returns the pipe at a given (x, y) position within the map
    fn get_pipe(&self, pos: (usize, usize)) -> Option<char> {
        let x = pos.0;
        let y = pos.1;
        
        let row = self.data.get(y);
        row?.chars().nth(x)
    }

    // Attempts to traverse the loop in the given direction. Returns
    // true if that direction is valid and the loop can be traversed
    fn traverse(&mut self, start_dir: Dir) -> bool {
        let mut dir = Some(start_dir);
        let mut pos = self.start_pos;
        let mut pipe_list: Vec<(usize, usize)> = vec![];
        let mut loop_complete = false;

        // Loop until we return to the start node or we reach
        // a dead end
        loop {
            let npos = dir.unwrap().step(pos);
            if npos == pos { break; }
            pos = npos;

            let pipe = self.get_pipe(pos).unwrap();
            dir = dir.unwrap().next_dir(pipe);

            pipe_list.push(pos);

            if pipe == 'S' {
                loop_complete = true;
                break;
            }

            if dir.is_none() {
                break;
            }
        }

        if loop_complete {
            self.loop_pipes = pipe_list;
        }

        loop_complete
    }

    // Returns the number of points enclosed within the loop
    fn find_enclosed(&mut self) -> u32 {
        let mut start_dirs: Vec<Dir> = vec![];

        // Attempts to traverse the loop in all four directions, recording which
        // directions are valid for later use
        for direction in Dir::iter() {
            if self.traverse(direction) {
                start_dirs.push(direction);
            }
        }

        // Determine the shape of the starting pipe based on the two valid
        // directions of travel leaving that pipe.
        // We need this for the ray casting algorithm, because whether a node is
        // enclosed within the loop depends on the shapes of the pipes around it.
        if start_dirs.contains(&Dir::N) && start_dirs.contains(&Dir::S) {
            self.s_shape = '|';
        } else if start_dirs.contains(&Dir::N) && start_dirs.contains(&Dir::E) {
            self.s_shape = 'L';
        } else if start_dirs.contains(&Dir::N) && start_dirs.contains(&Dir::W) {
            self.s_shape = 'J';
        } else if start_dirs.contains(&Dir::S) && start_dirs.contains(&Dir::E) {
            self.s_shape = 'F';
        } else if start_dirs.contains(&Dir::S) && start_dirs.contains(&Dir::W) {
            self.s_shape = '7';
        } else if start_dirs.contains(&Dir::E) && start_dirs.contains(&Dir::W) {
            self.s_shape = '-';
        }

        // Performs the ray casting algorithm to get the number of enclosed tiles
        self.cast_ray()
    }

    // Ray casting algorithm. Iterates across the map, toggling a bool any time it
    // reaches a pipe segment that's both part of the loop and has a valid connection
    // facing north ('|', 'L', or 'J'). Crossing any of those means we've either
    // entered or exited an enclosed section of the loop. Any position we reach
    // while is_enclosed == true is enclosed.

    // Full disclosure: I had to look up that last part of the solution. I was so close
    // with my ray casting algorithm, but I was toggling every time I reached any
    // pipe segment that was part of the loop and perpendicular to the direction of
    // traversal. Couldn't figure out why my answers were (usually) wrong until I 
    // got some help. 
    fn cast_ray(&self) -> u32 {
        let mut is_enclosed = false;

        let mut enclosed = 0;

        for y in 0..self.rows {
            for x in 0..self.cols {
                let pos = (x, y);
                let valid_pipes = ['|', 'L', 'J'];

                // If the segment we're checking is the start position ('S'), change
                // it the correct shape based on the valid traversal directions as
                // we calculated earlier
                let mut pipe = self.get_pipe(pos).unwrap();
                if pipe == 'S' {
                    pipe = self.s_shape;
                }

                // If the current segment is both contained within the loop and has
                // the correct shape, toggle is_enclosed and move on
                if self.loop_pipes.contains(&pos) {
                    if valid_pipes.contains(&pipe) {
                        is_enclosed = !is_enclosed;
                    }
                    continue;
                }

                if is_enclosed {
                    enclosed += 1;
                }
            }
        }
        enclosed
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W
}

impl Dir {
    fn iter() -> impl Iterator<Item = Dir> {
        [Dir::N, Dir::S, Dir::E, Dir::W].iter().copied()
    }

    // Gets the next position in the given direction
    fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::N => {
                if pos.1 == 0 {
                    pos
                } else {
                    (pos.0, pos.1 - 1)
                }
            },
            Dir::S => (pos.0, pos.1 + 1),
            Dir::E => (pos.0 + 1, pos.1),
            Dir::W => {
                if pos.0 == 0 {
                    pos
                } else {
                    (pos.0 - 1, pos.1)
                }
            }
        }
    }

    // Returns the next direction given the entry direction 
    // and the current pipe shape
    fn next_dir(&self, pipe: char) -> Option<Dir> {
        match pipe {
            '|' => {
                match self {
                    Dir::N | Dir::S => return Some(*self),
                    _ => return None,
                }
            },
            '-' => {
                match self {
                    Dir::E | Dir::W => return Some(*self),
                    _ => return None,
                }
            },
            'L' => {
                match self {
                    Dir::S => return Some(Dir::E),
                    Dir::W => return Some(Dir::N),
                    _ => return None,
                }
            },
            'J' => {
                match self {
                    Dir::S => return Some(Dir::W),
                    Dir::E => return Some(Dir::N),
                    _ => return None,
                }
            },
            '7' => {
                match self {
                    Dir::N => return Some(Dir::W),
                    Dir::E => return Some(Dir::S),
                    _ => return None,
                }
            },
            'F' => {
                match self {
                    Dir::N => return Some(Dir::E),
                    Dir::W => return Some(Dir::S),
                    _ => return None,
                }
            },
            _ => return None,
        }
    }

}


fn main() {
    let input = get_input("./input.txt");

    // Initialize the map
    let mut map = Map::new(input.clone());

    // Print the number of enclosed segments
    println!("{}", map.find_enclosed());
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}