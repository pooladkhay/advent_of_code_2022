use std::fs;

fn main() {
    let input = get_input();

    let mut full_overlap_count: usize = 0;
    let mut no_overlap_count: usize = 0;
    let total_count = input.len();

    for pair in input {
        let elf1 = pair.first().expect("invalid input data");
        let elf2 = pair.get(1).expect("invalid input data");

        if elf1.0 - elf2.0 >= 0 && elf1.1 - elf2.1 <= 0 {
            full_overlap_count += 1;
        } else if elf1.0 - elf2.0 <= 0 && elf1.1 - elf2.1 >= 0 {
            full_overlap_count += 1
        }

        if elf1.1 - elf2.0 < 0 && elf1.1 - elf2.1 < 0 {
            no_overlap_count += 1;
        } else if elf1.0 - elf2.1 > 0 && elf1.1 - elf2.1 > 0 {
            no_overlap_count += 1
        }
    }

    println!("number of fully overlaping pairs: {}", full_overlap_count);
    println!(
        "number of no overlaping pairs: {}",
        total_count - no_overlap_count
    );
}

fn get_input() -> Vec<Vec<(i32, i32)>> {
    let input = fs::read_to_string("./src/inp.txt");

    match input {
        Ok(input) => input
            .lines()
            .map(|pair| pair.split(',').collect::<Vec<&str>>())
            .map(|pair| {
                pair.iter()
                    .map(|section| section.to_owned().split('-').collect::<Vec<&str>>())
                    .map(|section| {
                        let start = section
                            .first()
                            .expect("invalid input file")
                            .parse::<i32>()
                            .expect("invalid input file");
                        let end = section
                            .get(1)
                            .expect("invalid input file")
                            .parse::<i32>()
                            .expect("invalid input file");
                        (start, end)
                    })
                    .collect::<Vec<(i32, i32)>>()
            })
            .collect::<Vec<Vec<(i32, i32)>>>(),
        Err(err) => {
            panic!("err reading input: {}", err);
        }
    }
}
