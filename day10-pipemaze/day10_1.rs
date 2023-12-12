use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug)]
struct Map {
    data: Vec<String>,
}

impl<'a> Map {
    fn new(data: Vec<String>) -> Self {
        Map { data }
    }

    fn get_pipe(&self, pos: (usize, usize)) -> char {
        let x = pos.0;
        let y = pos.1;
        
        let row = self.data.get(y);
        row.expect("Invalid index").chars().nth(x).unwrap()
    }

    fn traverse(&self, start_pos: (usize, usize), start_dir: Dir) -> (bool, HashMap<(usize, usize), u32>) {
        let mut steps = 0;
        let mut dir = Some(start_dir);
        let mut hmap = HashMap::<(usize, usize), u32>::new();
        let mut pos = start_pos;
        let mut pipe = self.get_pipe(pos);
        let mut loop_complete = false;

        loop {
            let mut npos = dir.unwrap().step(pos);
            if npos == pos { break; }
            pos = npos;

            pipe = self.get_pipe(pos);
            dir = dir.unwrap().next_dir(pipe);

            if pipe == 'S' {
                loop_complete = true;
                break;
            }

            if dir.is_none() {
                break;
            }
            steps += 1;
            hmap.insert(pos, steps);
        }

        (loop_complete, hmap)
    }

}

#[derive(Debug, Copy, Clone)]
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
            },
            _ => pos,
        }
    }

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

    let rows = input.len();
    let cols = input[0].len();

    let mut x: usize = 0;
    let mut y: usize = 0;

    for (i, line) in input.iter().enumerate() {
        if line.find('S').is_none() { continue; }
        x = line.find('S').unwrap();
        y = i;
        break;
    }

    let mut loop_dirs: Vec<Dir> = vec![];

    let map = Map::new(input.clone());

    let mut hmaps: Vec<HashMap<(usize, usize), u32>> = vec![];

    let start_pos = (x, y);
    let mut pos = start_pos;
    let mut pipe = map.get_pipe(pos);

    for start_dir in Dir::iter() {
        let (loop_found, dist_map) = map.traverse(start_pos, start_dir);
        if loop_found {
            hmaps.push(dist_map);
        }
    }

    let mut min_values: Vec<u32> = vec![];
    
    for (key, &val1) in hmaps[0].iter() {
        if let Some(&val2) = hmaps[1].get(key) {
            min_values.push(val1.min(val2));
        }
    }
    println!("{:?}", min_values.iter().max());
}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}
