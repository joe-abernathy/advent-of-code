    use aoc_helpers::{ * };
    use std::error::Error;
    use std::collections::HashSet;
    use std::time::Instant;

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
        splits: HashSet<(usize, usize)>,
        split_count: u32,
        beams: Vec<(usize, usize)>,
        visited: HashSet<(usize, usize)>,
    }

    impl Manifold {
        fn new(input: Vec<String>) -> Self {
            let mut grid: Vec<Vec<Node>> = Vec::new();
            let mut start: (usize, usize) = (0, 0);
            let splits: HashSet<(usize, usize)> = HashSet::new();
            let split_count: u32 = 0;

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

            let beams: Vec<(usize, usize)> = Vec::from([start]);
            let visited: HashSet::<(usize, usize)> = HashSet::from([start]);

            Self { grid, splits, split_count, beams, visited }   
        }

        // Start the machine and track the number of splits
        fn run(&mut self) -> u32 {

            // Loop until there are no more active beams
            while !self.beams.is_empty() {

                // This will hold the new beams that are created this iteration of the outer loop
                let mut new_beams: Vec<(usize, usize)> = Vec::new();

                // Loop through all currently active beams. This loop will fully process each beam until it hits a splitter
                // and creates 2 new beams or until it reaches the end of the grid. The new_beams vector will be populated 
                // with the beams created during this loop, which will be copied into self.beams at the end of the outer loop
                for beam in &self.beams {

                    // This will hold the running coordinates for the current beam as it traverses the grid
                    let (x, mut y) = *beam;

                    // Process the current beam until it splits or reaches the end of the grid
                    loop {

                        // Check the beam's current position to see if it's empty space or a splitter, and proceed accordingly
                        if let Some(node) = self.check_node((x, y)) {
                            match node {
                                Node::Empty | Node::Start => {
                                    y += 1;
                                    self.visited.insert((x, y));
                                },

                                Node::Splitter => {
                                    
                                    // If we haven't counted this splitter yet, increment the split_count variable to track the 
                                    // total number of splits 
                                    if self.splits.insert((x, y)) {
                                        self.split_count += 1;
                                    }

                                    // Create a new beam to the left, assuming there isn't already a beam there and the new
                                    // position isn't outside the bounds of the grid
                                    if let Some(x_new) = x.checked_sub(1) && self.check_node((x_new, y)).is_some() {
                                        if !self.visited.contains(&(x_new, y)) && !new_beams.contains(&(x_new, y)) {
                                            new_beams.push((x_new, y));
                                        } 
                                    }

                                    // Create a new beam to the right with the same constraints
                                    if self.check_node((x + 1, y)).is_some() {
                                        if !self.visited.contains(&(x + 1, y)) && !new_beams.contains(&(x + 1, y)) {
                                            new_beams.push((x + 1, y));
                                        }
                                    }
                                    break;
                                }
                            }

                        } else {
                            break;
                        }
                    }
                }

                // Since we've fully processed all beams that were in self.beams, the newly created beams get placed into that
                // vector to be processed next iteration of the loop. If we didn't create any new beams this iteration, self.beams 
                // will be empty and the loop will end.
                self.beams = new_beams;
            }

            // Return the total number of times the beam split
            self.split_count
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