use std::fs;

struct Section {
    start: i32,
    end: i32,
}

fn main() {
    let input = get_input();

    let mut full_overlap_count: usize = 0;
    let mut no_overlap_count: usize = 0;
    let total_count = input.len();

    for pair in input {
        let elf_1_section = pair.first().expect("invalid input data");
        let elf_2_section = pair.get(1).expect("invalid input data");

        if elf_1_section.start - elf_2_section.start >= 0
            && elf_1_section.end - elf_2_section.end <= 0
        {
            full_overlap_count += 1;
        } else if elf_1_section.start - elf_2_section.start <= 0
            && elf_1_section.end - elf_2_section.end >= 0
        {
            full_overlap_count += 1
        }

        if elf_1_section.end - elf_2_section.start < 0 && elf_1_section.end - elf_2_section.end < 0
        {
            no_overlap_count += 1;
        } else if elf_1_section.start - elf_2_section.end > 0
            && elf_1_section.end - elf_2_section.end > 0
        {
            no_overlap_count += 1
        }
    }

    println!("number of fully overlaping pairs: {}", full_overlap_count);
    println!(
        "number of no overlaping pairs: {}",
        total_count - no_overlap_count
    );
}

fn get_input() -> Vec<Vec<Section>> {
    let input = fs::read_to_string("./src/input.txt");

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

                        Section { start, end }
                    })
                    .collect::<Vec<Section>>()
            })
            .collect::<Vec<Vec<Section>>>(),
        Err(err) => {
            panic!("err reading input: {}", err);
        }
    }
}
