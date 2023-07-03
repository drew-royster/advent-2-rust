use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_points:i32 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let mut game_points = 0;
        let line = line?;
        let split: Vec<&str> = line.split(" ").collect();

        let their_move = split[0];
        let our_strategy = split[1];
        println!("They chose {}, you chose {}", split[0], split[1]);

        // we give ourselves points just for what we choose

        // A is rock
        // B is paper
        // C is scissors
        // X means lose
        // Y means draw
        // Z means win
        if their_move == "A" {
            if our_strategy == "X" {
                game_points += 3;
            } else if our_strategy == "Y" {
                game_points += 4;
            } else {
                game_points += 8;
            }
        } else if their_move == "B" {
            if our_strategy == "X" {
                game_points += 1;
            } else if our_strategy == "Y" {
                game_points += 5;
            } else {
                game_points += 9;
            }
        } else if their_move == "C" {
            if our_strategy == "X" {
                game_points += 2;
            } else if our_strategy == "Y" {
                game_points += 6;
            } else {
                game_points += 7;
            }
        }

        println!("{}", game_points);
        total_points += game_points;
    }

    println!("{}", total_points);
    Ok(())
}