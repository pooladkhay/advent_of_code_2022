use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt");

    match input {
        Ok(input) => {
            let mut chars: Vec<u32> = vec![];

            for line in input.lines() {
                let (lhs, rhs) = line.split_at(line.len() / 2);

                let mut current_line_chars: Vec<u32> = vec![];

                for l_ch in lhs.chars() {
                    for r_ch in rhs.chars() {
                        if l_ch == r_ch {
                            current_line_chars.push(item_priority(r_ch));
                        }
                    }
                }
                current_line_chars.dedup();
                chars.append(&mut current_line_chars);
            }
            let sum: u32 = chars.iter().sum();
            println!("sum of items the priorities: {}", sum);
        }
        Err(err) => {
            println!("err reading input: {}", err);
        }
    }
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
