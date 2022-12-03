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

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const LOSE_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

enum ElfHand {
    Rock,
    Paper,
    Scissors,
}

enum MyHand {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let strategies = get_strategies("./src/input.txt");

    let mut total_score = 0;

    for strategy in strategies {
        let (elf_hand, my_hand) = strategy;
        total_score += calc_score(elf_hand, my_hand);
    }

    println!("Part 1 total score: {}", total_score)
}

fn calc_score(elf_hand: ElfHand, my_hand: MyHand) -> u32 {
    match elf_hand {
        ElfHand::Rock => match my_hand {
            MyHand::Rock => DRAW_SCORE + ROCK_SCORE,
            MyHand::Paper => WIN_SCORE + PAPER_SCORE,
            MyHand::Scissors => LOSE_SCORE + SCISSORS_SCORE,
        },
        ElfHand::Paper => match my_hand {
            MyHand::Rock => LOSE_SCORE + ROCK_SCORE,
            MyHand::Paper => DRAW_SCORE + PAPER_SCORE,
            MyHand::Scissors => WIN_SCORE + SCISSORS_SCORE,
        },
        ElfHand::Scissors => match my_hand {
            MyHand::Rock => WIN_SCORE + ROCK_SCORE,
            MyHand::Paper => LOSE_SCORE + PAPER_SCORE,
            MyHand::Scissors => DRAW_SCORE + SCISSORS_SCORE,
        },
    }
}

// Panics if input file contains invalid strategies.
fn get_strategies(input_file: &str) -> Vec<(ElfHand, MyHand)> {
    let input = fs::read_to_string(input_file);

    let mut strategies: Vec<(ElfHand, MyHand)> = vec![];

    match input {
        Ok(data) => {
            for (index, line) in data.lines().enumerate() {
                let line = line.trim();
                if !line.is_empty() {
                    let strategy: Vec<&str> = line.split(' ').collect();

                    let elf_hand = if let Some(elf_hand) = strategy.first() {
                        match elf_hand.to_owned() {
                            "A" => ElfHand::Rock,
                            "B" => ElfHand::Paper,
                            "C" => ElfHand::Scissors,
                            _ => panic!("Invalid value found on strategy number: {}", index + 1),
                        }
                    } else {
                        panic!("Invalid value found on strategy number: {}", index + 1)
                    };

                    let my_hand = if let Some(my_hand) = strategy.last() {
                        match my_hand.to_owned() {
                            "X" => MyHand::Rock,
                            "Y" => MyHand::Paper,
                            "Z" => MyHand::Scissors,
                            _ => panic!("Invalid value found on strategy number: {}", index + 1),
                        }
                    } else {
                        panic!("Invalid value found on strategy number: {}", index + 1)
                    };

                    strategies.push((elf_hand, my_hand));
                }
            }
        }
        Err(err) => {
            println!("err reading input file: {:?}", err)
        }
    }

    strategies
}
