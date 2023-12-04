use std::io::{self, Write};

use crate::days::day1;
use crate::days::day2;
use crate::days::day3;
use crate::days::day4;
mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
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
        2 => {
            if let Err(err) = day2::main(part) {
                eprintln!("Error: {}", err);
            }
        }
        3 => {
            if let Err(err) = day3::main(part) {
                eprintln!("Error: {}", err);
            }
        }
        4 => {
            if let Err(err) = day4::main(part) {
                eprintln!("Error: {}", err);
            }
        }
        _ => {
            println!("Invalid day");
        }
    }
}
