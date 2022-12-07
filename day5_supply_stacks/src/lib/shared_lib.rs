use std::{collections::BTreeMap, fs};

pub struct Move {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}

pub fn get_initial_stacks(path: &str) -> BTreeMap<usize, Vec<String>> {
    let initial_stacks = get_input(path);
    let initial_stacks = initial_stacks.trim().to_owned();

    let mut stacks: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for (index, _) in initial_stacks.lines().enumerate() {
        stacks.insert(index + 1, vec![]);
    }

    for (index, line) in initial_stacks.lines().rev().enumerate() {
        let line = line
            .chars()
            .map(|ch| ch.to_string())
            .collect::<Vec<String>>();

        if index != 0 {
            let mut col = 1;
            let mut row = 1;

            while row < line.len() {
                let crt = line.get(row).unwrap().to_owned();
                if !crt.trim().is_empty() {
                    let stack = stacks.get_mut(&col).unwrap();
                    stack.push(crt);
                }

                row += 4;
                col += 1;
            }
        }
    }

    stacks
}

pub fn get_moves(path: &str) -> Vec<Move> {
    let moves = get_input(path);

    moves
        .lines()
        .map(|m| m.trim().split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|m| Move {
            amount: m.get(1).unwrap().parse::<usize>().unwrap(),
            from: m.get(3).unwrap().parse::<usize>().unwrap(),
            to: m.get(5).unwrap().parse::<usize>().unwrap(),
        })
        .collect::<Vec<Move>>()
}

pub fn get_input(path: &str) -> String {
    let input = fs::read_to_string(path);
    match input {
        Ok(input) => input,
        Err(err) => panic!("err reading input: {}", err),
    }
}
