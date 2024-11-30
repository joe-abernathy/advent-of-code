use aoc_helpers::{ * };
use std::collections::HashMap;
use lazy_static::lazy_static;

// Maps every possible combination of shapes played by each player to a win, lose, or draw for the player
// (the first entry in the tuple). Wins are mapped to 6 points, draws to 3 points, losses to 0 points.
lazy_static! {
    static ref RULES: HashMap<(char, char), u32> = {
        let mut m = HashMap::new();

        // Handle all possible draws
        for x in ['R', 'P', 'S'] {
            m.insert((x, x), 3);
        }

        // Handle cases where the player played rock
        m.insert(('R', 'S'), 6);
        m.insert(('R', 'P'), 0);

        // Handle cases where the player played paper
        m.insert(('P', 'R'), 6);
        m.insert(('P', 'S'), 0);

        // Handle cases where the player played scissors
        m.insert(('S', 'P'), 6);
        m.insert(('S', 'R'), 0);

        m
    };
}

struct Game {
    player_score: u32,

}

// Handles a round of the game, getting win/lose/draw for a combination of hands and calculating the
// player's score for this game
impl<'a> Game {
    fn new(game: Vec<char>) -> Self {

        // Create a Hand object for the player and elf
        let player = Hand::new(game[1]);
        let elf = Hand::new(game[0]);

        // Calculate the score from evaluating the game for win/lose/draw
        let eval_score = *RULES.get(&(player.shape, elf.shape)).unwrap();

        // Calculate the player's total score from their win/lose/draw score plus the number of 
        // points awarded for the player's selection
        let player_score = eval_score + player.points;

        Game { player_score }
    }
}

// Handles an individual player's hand
struct Hand {
    shape: char,
    points: u32,
}

impl<'a> Hand {
    fn new(encoded_shape: char) -> Self {

        // Maps the encoded shape chars (ABC, XYZ) to decoded shape chars (RPS)
        let shape: char = match encoded_shape {
            'A' | 'X' => 'R',
            'B' | 'Y' => 'P',
            'C' | 'Z' => 'S',
            _ => '_'
        };

        // Get the number of points this player earned for their shape selection
        // (rock = 1, paper = 2, scissors = 3)
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

// Convert a vector of strings (lines from the input) to the vector of vectors of chars needed for processing
fn get_games_from_lines(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.split_whitespace().map(|s| s.chars().next().unwrap()).collect())
        .collect()
}

// Calculate the player's total score across all games
fn evaluate_totals(games: Vec<Vec<char>>) -> u32 {
    let mut running_total = 0;

    // Iterate across all games, creating a Game object for each to calculate the player's current total
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
        let lines = read_from_file_as_lines("example.txt");
        let games = get_games_from_lines(lines);
        let result = evaluate_totals(games);

        assert_eq!(result, 15)
    }
}