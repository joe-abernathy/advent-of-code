use std::fs::File;
use std::io::{ self, prelude::*, BufReader };

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut total = 0;

    for line in reader.lines() {
        index += 1;
        let l = line?.clone();
        let games: Vec<&str> = l.split(": ").skip(1).collect();

        'game: for game in games {
            let draws: Vec<&str> = game.split(";").collect();

            for draw in draws {
                let colors: Vec<&str> = draw.split(", ").collect();

                for color in colors {
                    let parts: Vec<&str> = color.split_whitespace().collect();
                    let number = parts[0];
                    let color = parts[1]; 

                    if color == "red" && number.parse::<u16>().unwrap() > 12
                        || color == "green" && number.parse::<u16>().unwrap() > 13 
                        || color == "blue" && number.parse::<u16>().unwrap() > 14 
                    { 
                        break 'game; 
                    }
                }
            }
            total += index;
        }
    }
    println!("Total: {}", total);
    Ok(())
}