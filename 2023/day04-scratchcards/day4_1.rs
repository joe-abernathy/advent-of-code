use std::fs::File;
use std::io::{ self, prelude::*, BufReader };


fn main() -> io::Result<()> {
    let filename = "./input.txt";

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut cards: Vec<Vec<u32>> = vec![];
    let mut winners: Vec<Vec<u32>> = vec![];

    let mut total: u32 = 0;

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

    for (c, game) in cards.iter().enumerate() {
        let mut wins = 0;
        for num in game {
            if winners[c].contains(num) {
                wins += 1;
            }
        }
        if wins == 0 { continue; }

        let two:u32 = 2;
        total += two.pow(wins - 1);
    }

    println!("{}", total);

    Ok(())
}