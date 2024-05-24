use std::io::{self, BufRead};

pub fn print_error(message: String) {
    println!("[ \x1b[31;1mError\x1b[0m ] {}", message);
}

pub fn read_n_numbers_range(n: u8, min: u8, max: u8) -> Vec<u8> {
    let stdin = io::stdin();
    let mut line = String::new();

    // Read a line from stdin
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Failed to read line");

    // Split the line into whitespace-separated tokens
    let mut tokens = line.trim().split_whitespace();

    let mut numbers = Vec::with_capacity(n as usize);

    // Iterate over the tokens and convert them to u8
    for _ in 0..n {
        if let Some(token) = tokens.next() {
            if let Ok(num) = token.parse::<u8>() {
                if num >= min && num <= max {
                    if !numbers.contains(&num) {
                        numbers.push(num);
                    } else {
                        print_error("Duplicit numbers".to_string());
                        return Vec::new();
                    }
                } else {
                    print_error("Number out of range [1-100]".to_string());
                    return Vec::new();
                }
            } else {
                print_error("Invalid input".to_string());
                return Vec::new();
            }
        } else {
            print_error("Not enough numbers provided".to_string());
            return Vec::new();
        }
    }

    numbers
}

pub fn read_n_play_numbers(n: u8) -> Vec<u8> {
    read_n_numbers_range(n, 1, 100)
}
