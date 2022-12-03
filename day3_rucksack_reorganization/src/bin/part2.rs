use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt");

    match input {
        Ok(input) => {
            let lines: Vec<&str> = input.lines().collect();

            let mut badges: Vec<char> = vec![];

            for group in lines.chunks(3) {
                let group: Vec<Vec<char>> = group
                    .iter()
                    .map(|gp| {
                        let mut gp: Vec<char> = gp.chars().collect();
                        let mut uniques = HashSet::new();
                        gp.retain(|e| uniques.insert(*e));
                        gp
                    })
                    .collect();

                badges.append(&mut detect_badge(group))
            }

            let mut sum_priority: u32 = 0;
            for badge in badges {
                sum_priority += item_priority(badge)
            }
            println!("sum of badges priorities: {:?}", sum_priority);
        }
        Err(err) => {
            println!("err reading input: {}", err);
        }
    }
}

fn detect_badge(group: Vec<Vec<char>>) -> Vec<char> {
    if group.len() != 3 {
        panic!("Each group must contain 3 rucksacks")
    }
    let (rucksack1, rucksack2, rucksack3) = (
        group.get(0).unwrap(),
        group.get(1).unwrap(),
        group.get(2).unwrap(),
    );

    let badge: Vec<char> = rucksack1
        .iter()
        .filter(|i| rucksack2.contains(i) && rucksack3.contains(i))
        .map(|i| i.to_owned())
        .collect();

    badge
}

fn item_priority(item: char) -> u32 {
    if !item.is_ascii() {
        panic!("Non-ASCII character found");
    }
    if item.is_ascii_lowercase() {
        let ascii_code = item as u32;
        ascii_code - 96
    } else {
        let ascii_code = item as u32;
        ascii_code - 38
    }
}
