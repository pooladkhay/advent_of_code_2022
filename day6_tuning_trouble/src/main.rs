use std::{collections::HashSet, fs};
fn main() {
    let input = get_input("src/input.txt");

    println!("start of packet: {}", detect_start_of_packet(&input));
    println!("start of message: {}", detect_start_of_message(&input));
}

fn detect_start_of_packet(input: &String) -> usize {
    let mut window_start = 0_usize;
    let mut window_end = 3_usize;

    while window_end < input.len() {
        let window = input.get(window_start..=window_end).unwrap();
        let set = window.chars().collect::<HashSet<char>>();
        if set.len() == 4 {
            break;
        }
        window_start += 1;
        window_end += 1;
    }

    window_end + 1
}

fn detect_start_of_message(input: &String) -> usize {
    let mut window_start = 0_usize;
    let mut window_end = 13_usize;

    while window_end < input.len() {
        let window = input.get(window_start..=window_end).unwrap();
        let set = window.chars().collect::<HashSet<char>>();
        if set.len() == 14 {
            break;
        }
        window_start += 1;
        window_end += 1;
    }

    window_end + 1
}

fn get_input(path: &str) -> String {
    let input = fs::read_to_string(path);
    match input {
        Ok(input) => input,
        Err(err) => panic!("err reading input: {}", err),
    }
}
