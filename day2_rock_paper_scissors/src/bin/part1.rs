// A = Rock
// B = Paper
// C = Scissors

// X = Rock -> 1
// Y = Paper -> 2
// Z = Scissors -> 3

// Lost -> 0
// Draw -> 3
// Win -> 6

use std::{fs, vec};

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

fn main() {
    let input = get_strategy_guide();

    let mut total_score = 0;

    for game in input {
        total_score += calc_score(game)
    }

    println!("total score: {}", total_score)
}

fn calc_score(game: (String, String)) -> u32 {
    match game.0.as_str() {
        "A" => match game.1.as_str() {
            "X" => DRAW + ROCK,
            "Y" => WIN + PAPER,
            "Z" => LOSE + SCISSORS,
            _ => 0,
        },
        "B" => match game.1.as_str() {
            "X" => LOSE + ROCK,
            "Y" => DRAW + PAPER,
            "Z" => WIN + SCISSORS,
            _ => 0,
        },
        "C" => match game.1.as_str() {
            "X" => WIN + ROCK,
            "Y" => LOSE + PAPER,
            "Z" => DRAW + SCISSORS,
            _ => 0,
        },
        _ => 0,
    }
}

fn get_strategy_guide() -> Vec<(String, String)> {
    let input = fs::read_to_string("./src/input.txt");

    let mut strategies: Vec<(String, String)> = vec![];

    match input {
        Ok(data) => {
            for line in data.lines() {
                let hands: Vec<&str> = line.split(' ').collect();
                let elf = hands.first().unwrap().to_owned().to_owned();
                let me = hands.last().unwrap().to_owned().to_owned();
                strategies.push((elf, me));
            }
        }
        Err(err) => {
            println!("err reading input file: {:?}", err)
        }
    }

    strategies
}
