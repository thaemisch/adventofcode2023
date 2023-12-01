use std::io::{self, Write};

use crate::days::day1;
mod days {
    pub mod day1;
}



fn main() {
    println!("Advent of Code 2023");
    print!("Enter day: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u32 = input.trim().parse().expect("Invalid input");

    print!("Enter part: ");
    std::io::stdout().flush().unwrap();

    let mut input_part = String::new();
    io::stdin()
        .read_line(&mut input_part)
        .expect("Failed to read input");

    let part: u32 = input_part.trim().parse().expect("Invalid input");

    match number {
        1 => {
            if let Err(err) = day1::main(part) {
                eprintln!("Error: {}", err);
            }
        }
        _ => {
            println!("Invalid day");
        }
    }
}
