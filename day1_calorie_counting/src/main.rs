use std::fs;
fn main() {
    let input = fs::read_to_string("./src/input.txt");

    match input {
        Ok(input) => {
            let input: Vec<&str> = input.split('\n').collect();

            let mut cal_sum_per_elf: Vec<u32> = vec![];

            let mut cal_sum: u32 = 0;
            for cal in input {
                if !cal.is_empty() {
                    let cal = cal.trim().parse::<u32>().unwrap_or(0);
                    cal_sum += cal
                } else {
                    cal_sum_per_elf.push(cal_sum);
                    cal_sum = 0
                }
            }

            cal_sum_per_elf.sort();

            if let Some(last) = cal_sum_per_elf.last() {
                println!("most cal: {}", last);
            }

            println!(
                "sum of top three: {}",
                cal_sum_per_elf.iter().rev().take(3).sum::<u32>()
            );
        }
        Err(err) => {
            println!("err reading input: {}", err);
        }
    }
}
