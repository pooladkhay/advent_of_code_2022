use std::fs;

pub fn item_priority(item: char) -> u32 {
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

pub fn get_input(path: &str) -> String {
    let input = fs::read_to_string(path);
    match input {
        Ok(input) => input,
        Err(err) => panic!("err reading input: {}", err),
    }
}
