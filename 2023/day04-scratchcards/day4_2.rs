use std::fs::File;
use std::io::{ self, prelude::*, BufReader };

const NUM_GAMES: usize = 220;

fn main() -> io::Result<()> {
    let filename = "./input.txt";

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut cards: Vec<Vec<u32>> = vec![];
    let mut winners: Vec<Vec<u32>> = vec![];

    let mut num_cards: Vec<u32> = vec![1; NUM_GAMES];

    // Parse the input into a vector of cards and a vector of winning numbers for each game
    for line in reader.lines() {
        let l = line?.clone();

        let values: Vec<_> = l.split("|").collect();
        let card_line: Vec<_> = values[0].split(":").skip(1).collect();

        let card_str: Vec<_> = card_line.iter().flat_map(|&s| s.split_whitespace()).collect();
        let card: Vec<u32> = card_str.iter().map(|s| s.parse::<u32>().unwrap()).collect();
        cards.push(card);

        let win_str: Vec<_> = values[1].split_whitespace().collect();
        let win: Vec<u32> = win_str.iter().map(|s| s.parse::<u32>().unwrap()).collect();
        winners.push(win);
    }

    // Count the number of wins in each game
    for (c, game) in cards.iter().enumerate() {
        let mut wins = 0;
        for num in game {
            if winners[c].contains(num) {
                wins += 1;
            }
        }

        // For n wins, increment the number of cards for the next n games by the
        // number of cards held for the current game
        for i in 1..wins + 1 {
            num_cards[c + i] += num_cards[c];
        }
    }

    let total: u32 = num_cards.iter().sum();
    println!("{}", total);

    Ok(())
}