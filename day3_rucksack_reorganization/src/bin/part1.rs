use shared_lib::{get_input, item_priority};

fn main() {
    let input = get_input("./src/input.txt");

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
