use shared_lib::{get_initial_stacks, get_moves, Move};
use std::collections::BTreeMap;

fn main() {
    let mut stacks = get_initial_stacks("src/initial_stacks.txt");
    let moves = get_moves("src/moves.txt");

    for mv in moves {
        move_stacks(&mut stacks, mv);
    }

    let mut result: Vec<String> = vec![];
    for (_, v) in stacks {
        result.push(v.last().unwrap().to_owned());
    }

    println!("CrateMover 9000: {:?}", result.concat());
}

fn move_stacks(stacks: &mut BTreeMap<usize, Vec<String>>, mv: Move) {
    let mut temp: Vec<String> = vec![];
    {
        let from = stacks.get_mut(&mv.from).unwrap();
        for _ in 1..=mv.amount {
            temp.push(from.pop().unwrap());
        }
    }

    let to = stacks.get_mut(&mv.to).unwrap();
    to.append(&mut temp);
}
