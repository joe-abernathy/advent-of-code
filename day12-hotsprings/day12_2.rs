use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use itertools::Itertools;


/* 
This is very much a work in progress. I solved part 1 on a computer that's not connected to my GitHub, so it's not posted yet, but it will be.
My part 1 solution is a bruteforce, which works fine with the small dataset in that part, but completely fails on the massive dataset for part 2.
*/


/* TODO: node doesn't get passed the substring, it calculates it on its own? Maybe? Idk. */

#[derive(Debug)]
struct Node {
    substring: String,
    index: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(substring: String, i: usize) -> Self {
        Node { 
            substring, 
            index, 
            left: None, 
            right: None 
        }
    }
}


#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<Node>>,
    arrangement: Vec<usize>,
    unknowns: Vec<usize>,
    orig_string: String,
}

impl BinaryTree {
    fn new(orig_string: String, arrangement: Vec<usize>, unknowns: Vec<usize>) -> Self {
        let init_substring = if unknowns.len() > 1 && unknowns[1] > 0 && unknowns[1] <= orig_string.len() {
            orig_string[..unknowns[1] - 1].to_string()
        } else {
            orig_string.to_string()
        };

        let root = Some(Box::new(Node::new(init_substring, unknowns[0])));

        BinaryTree {
            root,
            arrangement,
            unknowns,
            orig_string,
        }
    }

    fn insert(&mut self, index: usize) {
        self.root = self.insert_recursive(self.root.take(), index);
    }

    fn insert_recursive(&mut self, node: Option<Box<Node>>, value: String, index: usize) -> Option<Box<Node>> {
        match node {
            Some(mut n) => {
                if index < n.1 {
                    n.left = self.insert_recursive(n.left.take(), index)
                } else {
                    n.right = self.insert_recursive(n.right.take(), index)
                }
                Some(n)
            }
            None => Some(Box::new(Node::new(index)));
        }
    }

    fn inorder_traversal(&self, node: &Option<Box<Node>>) {
        if let Some(n) == node {
            self.inorder_traversal(&n.left);


        }
    }

    fn prune(&mut self, condition: &Fn(&Node) -> bool) {
        if let Some(ref mut left) == self.left {
            if condition(&**left) {
                self.left = None;
            } else {
                left.prune(condition);
            }
        }
        if let Some(ref mut right) = self.right {
            if condition(&**right) {
                self.right = None;
            } else {
                right.prune(condition);
            }
        }
    }

    fn traverse(&self) {
        self.inorder_traversal(&self.root);
    }
}



fn main() {
    let input: Vec<String> = get_input("./example.txt");

    let mut total = 0;

    for line in input {
        let (springs, arrangement) = unfold(line);

        let unknowns: Vec<usize> = springs.match_indices('?').map(|(i, _)|i).collect();
        let broken: Vec<usize> = springs.match_indices('#').map(|(i, _)|i).collect();
        let operational: Vec<usize> = springs.match_indices('.').map(|(i, _)|i).collect();

        let start_substring = springs[..unknowns[0]].to_string();
        let mut tree = BinaryTree::new(start_substring, arrangement, unknowns);
        println!("{:#?}", tree);
    }
}


fn parse_input(line: String) -> (String, Vec<usize>) {
    let parts: Vec<_> = line.split_whitespace().collect();

    let springs = parts[0].to_string();

    let arr_str: Vec<_> = parts[1].split(',').collect();
    let arrangement: Vec<usize> = arr_str.iter().map(|s| s.parse::<usize>().unwrap()).collect();

    (springs, arrangement)
}


fn unfold(line: String) -> (String, Vec<usize>) {
    let parts: Vec<_> = line.split_whitespace().collect();

    let mut springs = parts[0].to_string() + "?";
    springs = springs.repeat(5);
    springs = springs[..springs.len() - 1].to_string();
    
    let arr_str: Vec<_> = parts[1].split(',').collect();
    let mut arrangement_folded: Vec<usize> = arr_str.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    let mut arrangement: Vec<usize> = vec![];

    for _ in 0..5 {
        for arr in arrangement_folded.clone() {
            arrangement.push(arr);
        }
    }
    (springs, arrangement)
}


fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Failed to parse line")).collect()
}