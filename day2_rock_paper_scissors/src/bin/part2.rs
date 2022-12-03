// A = Rock -> 1
// B = Paper -> 2
// C = Scissors -> 3

// X = Lose -> 0
// Y = Draw -> 3
// Z = Win -> 6

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

enum GameResult {
    Lose,
    Draw,
    Win,
}

fn main() {
    let strategies = get_strategies("./src/input.txt");
    let mut total_score = 0;

    for strategy in strategies {
        let (elf_hand, game_result) = strategy;
        total_score += calc_score(elf_hand, game_result);
    }

    println!("Part 2 total score: {}", total_score)
}

fn calc_score(elf_hand: ElfHand, game_result: GameResult) -> u32 {
    match elf_hand {
        ElfHand::Rock => match game_result {
            GameResult::Lose => LOSE_SCORE + SCISSORS_SCORE,
            GameResult::Draw => DRAW_SCORE + ROCK_SCORE,
            GameResult::Win => WIN_SCORE + PAPER_SCORE,
        },
        ElfHand::Paper => match game_result {
            GameResult::Lose => LOSE_SCORE + ROCK_SCORE,
            GameResult::Draw => DRAW_SCORE + PAPER_SCORE,
            GameResult::Win => WIN_SCORE + SCISSORS_SCORE,
        },
        ElfHand::Scissors => match game_result {
            GameResult::Lose => LOSE_SCORE + PAPER_SCORE,
            GameResult::Draw => DRAW_SCORE + SCISSORS_SCORE,
            GameResult::Win => WIN_SCORE + ROCK_SCORE,
        },
    }
}

// Panics if input file contains invalid strategies.
fn get_strategies(input_file: &str) -> Vec<(ElfHand, GameResult)> {
    let input = fs::read_to_string(input_file);

    let mut strategies: Vec<(ElfHand, GameResult)> = vec![];

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

                    let game_result = if let Some(game_result) = strategy.last() {
                        match game_result.to_owned() {
                            "X" => GameResult::Lose,
                            "Y" => GameResult::Draw,
                            "Z" => GameResult::Win,
                            _ => panic!("Invalid value found on strategy number: {}", index + 1),
                        }
                    } else {
                        panic!("Invalid value found on strategy number: {}", index + 1)
                    };

                    strategies.push((elf_hand, game_result));
                }
            }
        }
        Err(err) => {
            println!("err reading input file: {:?}", err)
        }
    }

    strategies
}
