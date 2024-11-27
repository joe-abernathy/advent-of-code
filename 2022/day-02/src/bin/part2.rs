use aoc_helpers::{ * };
use std::collections::HashMap;
use lazy_static::lazy_static;

// Maps a tuple containing the elf's shape (rock/paper/scissors) and the desired result
// for the player (win/lose/draw) to the shape the player needs to play (R/P/S)
lazy_static! {
    static ref SELECTOR: HashMap<(char, char), char> = {
        let mut s = HashMap::new();

        for x in ['R', 'P', 'S'] {
            s.insert((x, 'D'), x);
        }

        s.insert(('R', 'W'), 'P');
        s.insert(('R', 'L'), 'S');
        
        s.insert(('P', 'W'), 'S');
        s.insert(('P', 'L'), 'R');

        s.insert(('S', 'W'), 'R');
        s.insert(('S', 'L'), 'P');

        s
    };
}

// Handles a round of the game, calculating the shape the player needs to play from the desired result,
// and calculating the player's score for this round
struct Game {
    player_score: u32,
}

impl<'a> Game {
    fn new(game: Vec<char>) -> Self {
        let elf = Hand::new(game[0]);

        let desired_result: char = match game[1] {
            'Z' => 'W',
            'Y' => 'D',
            'X' => 'L',
            _ => '_',
        };

        let desired_shape = *SELECTOR.get(&(elf.shape, desired_result)).unwrap_or(&'F');
        let player = Hand::new(desired_shape);

        let eval_score = match desired_result {
            'W' => 6,
            'D' => 3,
            'L' => 0,
            _ => 0
        };

        let player_score = eval_score + player.points;
        
        Game { player_score }
    }

}

// Handles a single player's hand
struct Hand {
    shape: char,
    points: u32,
}

impl<'a> Hand {
    fn new(encoded_shape: char) -> Self {
        let shape: char = match encoded_shape {
            'A' | 'R' => 'R',
            'B' | 'P' => 'P',
            'C' | 'S' => 'S',
            _ => '_',
        };

        let points: u32 = match shape {
            'R' => 1,
            'P' => 2,
            'S' => 3,
            _ => 0,
        };

        Hand { shape, points }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = get_puzzle_input_as_lines(2022, 2)?;

    let games = get_games_from_lines(lines);
    let result = evaluate_totals(games);

    println!("{}", result);

    Ok(())
}

fn get_games_from_lines(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.split_whitespace().map(|s| s.chars().next().unwrap()).collect())
        .collect()
}

fn evaluate_totals(games: Vec<Vec<char>>) -> u32 {
    let mut running_total = 0;

    for line in games {
        let game = Game::new(line);
        running_total += game.player_score;
    }

    running_total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = read_from_file("example.txt");
        let games = get_games_from_lines(lines);
        let result = evaluate_totals(games);

        assert_eq!(result, 12)
    }
}