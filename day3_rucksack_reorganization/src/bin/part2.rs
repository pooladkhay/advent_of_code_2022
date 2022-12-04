use std::collections::HashSet;

use shared_lib::{get_input, item_priority};

fn main() {
    let input = get_input("./src/input.txt");

    let lines: Vec<HashSet<char>> = input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .collect();

    let mut badges: Vec<char> = vec![];

    for group in lines.chunks(3) {
        badges.push(detect_badge(group));
    }

    let mut sum_priority: u32 = 0;
    for badge in badges {
        sum_priority += item_priority(badge)
    }
    println!("sum of badges priorities: {:?}", sum_priority);
}

fn detect_badge(group: &[HashSet<char>]) -> char {
    if group.len() != 3 {
        panic!("Each group must contain 3 rucksacks")
    };

    let (rucksack1, rucksack2, rucksack3) = (
        group.get(0).unwrap(),
        group.get(1).unwrap(),
        group.get(2).unwrap(),
    );

    let badge = rucksack1
        .intersection(
            &rucksack2
                .intersection(rucksack3)
                .map(|ch| ch.to_owned())
                .collect::<HashSet<char>>(),
        )
        .map(|ch| ch.to_owned())
        .collect::<String>();

    if badge.is_empty() {
        panic!("All groups must contain a badge")
    }

    // convert badge from String to char
    badge
        .chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap()
        .to_owned()
}
