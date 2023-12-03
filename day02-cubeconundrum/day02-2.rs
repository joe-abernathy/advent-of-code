use std::fs::File;
use std::io::{ self, prelude::*, BufReader };

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut index = 0;

    for line in reader.lines() {
        index += 1;
        let l = line?.clone();
        let games: Vec<&str> = l.split(": ").skip(1).collect();

        for game in games {
            let draws: Vec<&str> = game.split("; ").collect();

            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;

            for draw in draws {
                let colors: Vec<&str> = draw.split(", ").collect();

                for color in colors {
                    let parts: Vec<&str> = color.split_whitespace().collect();
                    
                    let number = parts[0].parse::<u16>().unwrap();
                    let color = parts[1]; 

                    match color {
                        "red" => {
                            if number > red_max { 
                             red_max = number; 
                            }
                        },

                        "green" => {
                            if number > green_max { 
                             green_max = number; 
                            }
                        },

                        "blue" => {
                            if number > blue_max { 
                             blue_max = number; 
                            }
                        },

                        _ => continue,
                    }
                }
            }
            let current_total = red_max * green_max * blue_max;
            println!("{}: current total: red {} * green {} * blue {} = {}", index, red_max, green_max, blue_max, current_total);
            total += current_total;
        }
    }
    println!("Total: {}", total);
    Ok(())
}