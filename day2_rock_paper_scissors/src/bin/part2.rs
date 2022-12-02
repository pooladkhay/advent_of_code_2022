// A = Rock -> 1
// B = Paper -> 2
// C = Scissors -> 3

// X = Lose -> 0
// Y = Draw -> 3
// Z = Win -> 6

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
        let (elf_hand, my_hand) = game;
        total_score += calc_score(elf_hand, my_hand);
    }

    println!("total score: {}", total_score)
}

fn calc_score(elf_hand: String, my_hand: String) -> u32 {
    match elf_hand.as_str() {
        "A" => match my_hand.as_str() {
            "X" => LOSE + SCISSORS,
            "Y" => DRAW + ROCK,
            "Z" => WIN + PAPER,
            _ => 0,
        },
        "B" => match my_hand.as_str() {
            "X" => LOSE + ROCK,
            "Y" => DRAW + PAPER,
            "Z" => WIN + SCISSORS,
            _ => 0,
        },
        "C" => match my_hand.as_str() {
            "X" => LOSE + PAPER,
            "Y" => DRAW + SCISSORS,
            "Z" => WIN + ROCK,
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
