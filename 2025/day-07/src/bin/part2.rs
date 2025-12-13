    use aoc_helpers::{ * };
    use std::error::Error;
    use std::collections::HashMap;
    use std::time::Instant;

    // My implementation for this part was a lot simpler than my implementation for part 1, which makes me think I overengineered
    // part 1. This was just a DFS binary tree traversal problem

    fn main() -> Result<(), Box<dyn Error>> {
        let input: Vec<String> = if cfg!(feature = "test") {
            read_from_file_as_lines("example.txt")
        } else {
            get_puzzle_input_as_lines(2025, 7)?
        };

        let mut m = Manifold::new(input);

        let now = Instant::now();
        let result = m.run();

        println!("Result: {}\nExecution time: {:?}", result, now.elapsed());
        Ok(())
    }

    // Struct to handle the manifold, beams, etc
    struct Manifold { 
        grid: Vec<Vec<Node>>,
        splits: HashMap<(usize, usize), u64>,
        start: (usize, usize),          
    }

    impl Manifold {
        fn new(input: Vec<String>) -> Self {
            let mut grid: Vec<Vec<Node>> = Vec::new();
            let mut start: (usize, usize) = (0, 0);
            let splits: HashMap<(usize, usize), u64> = HashMap::new();
            let split_count: u64 = 0;

            // Parse the input vector into a 2D grid
            for (row, s) in input.iter().enumerate() {
                let mut column: Vec<Node> = Vec::new();

                for (col, ch) in s.chars().enumerate() {
                    let node  = Node::try_from(ch).unwrap();
                    if node == Node::Start {
                        start = (col, row);
                    }
                    column.push(node);
                }

                grid.push(column);
            }

            Self { grid, splits, start }   
        }

        fn run(&mut self) -> u64 {
            let start_vec: Vec<(usize, usize)> = Vec::from([self.start]);
            self.traverse(&start_vec)
        }

        // Do a recursive DFS traversal of the tree to find the total number of possible paths
        fn traverse(&mut self, paths: &Vec<(usize, usize)>) -> u64 {

            // If the current node is a leaf, return 1
            if paths.is_empty() {
                return 1;
            }

            let mut path_count: u64 = 0;
            
            // For each possible path from this node, either get the saved path count from that node,
            // or recursively calculate the path count and save it in the splits hashmap
            for path in paths {
                if let Some(count) = self.splits.get(path) {
                    path_count += count;
                
                } else {
                    let next_paths = self.step(*path);
                    let count = self.traverse(&next_paths);

                    self.splits.insert(*path, count);
                    path_count += count;
                }
            }

            path_count
        }

        // Step through one path in the graph until reaching a splitter or the end of the graph
        fn step(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
            let (x, mut y) = (pos.0, pos.1 + 1);

            let mut new_paths: Vec<(usize, usize)> = Vec::new();

            while let Some(node) = self.check_node((x, y)) {
                match node {
                    Node::Empty | Node::Start => y += 1,
                    Node::Splitter => {
                        if let Some(x_new) = x.checked_sub(1) && self.check_node((x_new, y)).is_some() {
                            new_paths.push((x_new, y));
                        }

                        if self.check_node((x + 1, y)).is_some() {
                            new_paths.push((x + 1, y));
                        }

                        break;
                    },
                }
            }
            new_paths
        }

        // Checks an (x, y) position and returns either the Node enum at that position, or None if that index is outside the grid
        fn check_node(&self, (x, y): (usize, usize)) -> Option<Node> {
            if let Some(node) = self.grid.get(y).and_then(|row| row.get(x)).copied() {
                return Some(node);
            }
            None
        }

    }

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    enum Node {
        Start,
        Empty,
        Splitter,
    }

    impl TryFrom<char> for Node {
        type Error = InvalidNodeError;

        fn try_from(ch: char) -> Result<Self, Self::Error> {
            match ch {
                'S' => Ok(Node::Start),
                '.' => Ok(Node::Empty),
                '^' => Ok(Node::Splitter),
                _ => Err(InvalidNodeError(ch)),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct InvalidNodeError(char);