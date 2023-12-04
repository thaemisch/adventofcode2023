use std::fs::File;
use std::io::{self, BufRead};

pub fn main(part: u32) -> io::Result<()> {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => {
            println!("Invalid part");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid part"));
        }
    }
}

fn part_one() -> io::Result<()> {
    let mut total: u32 = 0;

    let file = File::open("/home/tim/prog/adventofcode2023/src/inputs/day4.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut line = line?;
        line = line.strip_prefix("Card ").unwrap().to_string();
        let mut temp_number: String = String::new();
        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut counter: u32 = 0;
        let mut winning_done: bool = false;
        let mut start: bool = false;
        let mut subtotal: u32;
        let mut count_chars: u32 = 0;
        for char in line.chars() {
            count_chars += 1;
            if char.is_digit(10) {
                temp_number.push(char);
                if line.chars().count() == count_chars.try_into().unwrap() {
                    if temp_number != "" && start {
                        if winning_numbers.contains(&temp_number.parse::<u32>().unwrap()) {
                            counter += 1;
                            temp_number.clear();
                        } else {
                            temp_number.clear();
                        }
                    }
                }
            } else if char == '|' {
                winning_done = true;
                temp_number.clear();
            } else if char == ':' {
                start = true;
                temp_number.clear();
            } else if char == ' ' {
                if temp_number != "" && start {
                    if !winning_done {
                        winning_numbers.push(temp_number.parse::<u32>().unwrap());
                        temp_number.clear();
                    } else {
                        if winning_numbers.contains(&temp_number.parse::<u32>().unwrap()) {
                            counter += 1;
                            temp_number.clear();
                        } else {
                            temp_number.clear();
                        }
                    }
                }
            }
        }
        if counter > 0 {
            subtotal = 1;
            for _ in 1..counter {
                subtotal *= 2;
            }
            total += subtotal;
        }
    }
    println!("{}", total);
    Ok(())
}

fn part_two() -> io::Result<()> {
    let mut total: u32 = 0;

    println!("{}", total);
    Ok(())
}
