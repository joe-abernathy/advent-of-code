use aoc_helpers::{ * };
use std::error::Error;
use std::collections::HashMap;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_puzzle_input_as_lines(2022, 7)?;
    let mut filesys: HashMap<String, (u32, HashMap<String, u32>)> = HashMap::new();

    parse_inputs(input, &mut filesys);
    let result = get_total_size(filesys.clone());

    println!("{}", result);

    Ok(())
}

// Processes each line in the input 
fn parse_inputs(input: Vec<String>, filesys: &mut HashMap<String, (u32, HashMap<String, u32>)>) {

    // Points to the current directory we're working with, since we need to keep track of hierarchy and whatnot
    let mut current_dir = String::new();
    
    for line in input {

        // If the first character in the line is '$', process it as a command
        if line.clone().chars().nth(0) == Some('$') {

            // Get the command by trimming off the first 2 characters ('$ '), then split it by whitespace
            let cmd: String = line.chars().skip(2).collect();
            let split: Vec<&str> = cmd.split(' ').collect();

            // split[0] is the first word of the command ("cd" or "ls")
            match split[0] {
                "cd" => {

                    // split[1] is the second word of the command. This only applies for "cd" commands, since this
                    // puzzle doesn't implement ls with any secondary commands
                    match split[1] {

                        // If the command is "cd ..", traverse up one level
                        ".." => {
                            current_dir = up_one_level(current_dir);
                        },

                        // If the command is "cd <anything else>", traverse down one level using the subdirectory provided
                        _ => {
                            current_dir = down_one_level(current_dir, split[1]);

                            // Once we've updated the current_dir pointer, if the new directory doesn't exist in the hashmap
                            // yet, add it now.
                            if filesys.get(&current_dir) == None {
                                let new_dir_map: HashMap<String, u32> = HashMap::new();
                                filesys.insert(current_dir.clone(), (0, new_dir_map));
                            }
                        }
                    }
                }

                // If the command is "ls", we don't need to do any more processing -- just skip to the next line where we will
                // process the response(s).
                "ls" => continue,

                // If the first word is anything other than "cd" or "ls", something has gone horribly wrong
                _ => panic!("Got an unexpected command: {}", split[0]),
            }

        // If the line doesn't start with a $ and is therefore a response from the 'ls' command:
        } else {

            // Split the response by words
            let split_response: Vec<String> = line.split(' ')
                .map(|s| s.to_string())
                .collect();

            match split_response[0].as_str() {

                // If the first word of the response is "dir", the response is describing a subdirectory. The way I've implemented
                // this, we don't need to do any more processing here, since the directory will be added to the hashmap elsewhere,
                // so we just move on to the next line. Is this the right way to do it? Probably not. Will this come back to bite 
                // me in part 2? Almost certainly. But we're here now.
                "dir" => continue,

                // If the first word isn't "dir", the response is describing a file, and will include two words: the size of the
                // file, followed by the filename.
                _ => {

                    // Parse the first word (size) into an integer
                    let size: u32 = split_response[0].parse::<u32>().expect("Expecting a string that can be parsed to an integer, didn't get it");

                    // Get the second word (filename)
                    let filename = split_response[1].clone();

                    // Insert a new entry into the inner hashmap corresponding to the current directory. The new entry contains
                    // the filename (key) and file size (value).
                    if let Some((_, current_dir_map)) = filesys.get_mut(&current_dir) {
                        current_dir_map.insert(filename.clone(), size);

                        recursive_dir_size_update(filesys, current_dir.clone(), size);
                    }
                }
            }
        }
    }
}

// Take a string pointing to the current directory and return a string pointing to its parent directory
fn up_one_level(current_dir: String) -> String {

    // If we're already at the root directory, just return without processing.
    // This shouldn't ever happen, but just in case.
    if current_dir == "/" {
        return current_dir;
    }

    // Split the directory string by '/' characters and collect it into a vector for processing
    let dir_vec: Vec<String> = current_dir.split('/')
        .map(|s| s.to_string())
        .collect();

    // Chop off the last entry in the split vector (the current directory), then slap what's left
    // back together, joined with "/"s
    let parent_dir = dir_vec
        .iter()
        .take(dir_vec.len() - 2)
        .map(|s| s.as_str())
        .join("/");

    // Add a trailing "/" and return
    format!("{}{}", parent_dir, "/")
}

// Take a string pointing to the current directory and a string slice pointing to the subdirectory,
// and return a string pointing to the subdirectory
fn down_one_level(current_dir: String, subdir: &str) -> String {
    if current_dir.is_empty() {
        return "/".to_string();
    }
    let new_dir = format!("{}{}", subdir, "/");
    format!("{}{}", current_dir, &new_dir)
}

// Whenever we add a new file, we need to update not only the directory size for the current directory, but also the directory
// size for every parent directory going up to the root. This function traverses the directory upwards toward the root, adding
// the new file's size to each directory's dir_size.
fn recursive_dir_size_update(filesys: &mut HashMap<String, (u32, HashMap<String, u32>)>, current_dir: String, filesize: u32) {
    
    // Points to the current directory in the traversal
    let mut dir = current_dir.clone();
    
    // Loop until we explictly break after reaching the root directory
    loop {

        // If the current directory already exists in the hashmap, add the current filesize to its dirsize
        if let Some((dirsize, _)) = filesys.get_mut(&dir) {
            *dirsize += filesize;
        
        // If the current directory isn't in the hashmap yet, insert it into the outer hashmap with an initial
        // dirsize equal to the filesize
        } else {
            let parent_dir_map: HashMap<String, u32> = HashMap::new();
            filesys.insert(dir.clone(), (filesize, parent_dir_map));
        }
        
        // Break out of the loop once we've processed the root directory
        if dir == "/" {
            break;
        }

        // Traverse up one level in the file system and continue the loop
        dir = up_one_level(dir);
    }
}

// Count up the sizes of all directories in the file system with a dirsize <= 100,000 and return the total
fn get_total_size(filesys: HashMap<String, (u32, HashMap<String, u32>)>) -> u32 {
    let mut result = 0;

    for (dirsize, _) in filesys.values() {
        if *dirsize <= 100000 {
            result += dirsize;
        }
    }
    
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<(), Box<dyn Error>> {
        let input = read_from_file_as_lines("example.txt");
        let mut filesys: HashMap<String, (u32, HashMap<String, u32>)> = HashMap::new();

        parse_inputs(input, &mut filesys);
        let result = get_total_size(filesys);

        Ok(assert_eq!(result, 95437))
    }
}