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

    let file = File::open("/home/tim/prog/adventofcode2023/src/inputs/day2.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut line = line?;
        line = line.strip_prefix("Game ").unwrap().to_string();
        let mut game_id: String = String::new();
        for char in line.chars() {
            if char.is_digit(10) {
                game_id.push(char);
            } else {
                break;
            }
        }
        let mut max_red: u8 = 0;
        let mut max_green: u8 = 0;
        let mut max_blue: u8 = 0;
        let mut temp_string: String = String::new();
        let mut temp_number: String = String::new();
        for char in line.chars() {
            if char.is_digit(10) {
                temp_number.push(char);
            } else if char == ' ' {
                continue;
            } else if char == ';' || char == ',' || char == ':' {
                temp_number = String::new();
                temp_string = String::new();
                continue;
            } else {
                temp_string.push(char);
                if temp_string.ends_with("red") {
                    if temp_number.parse::<u8>().unwrap() > max_red {
                        max_red = temp_number.parse::<u8>().unwrap();
                    }
                    temp_string = String::new();
                } else if temp_string.ends_with("green") {
                    if temp_number.parse::<u8>().unwrap() > max_green {
                        max_green = temp_number.parse::<u8>().unwrap();
                    }
                    temp_string = String::new();
                } else if temp_string.ends_with("blue") {
                    if temp_number.parse::<u8>().unwrap() > max_blue {
                        max_blue = temp_number.parse::<u8>().unwrap();
                    }
                    temp_string = String::new();
                }
            }
        }
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            total += game_id.parse::<u32>().unwrap();
        }
    }
    println!("{}", total);
    Ok(())
}

fn part_two() -> io::Result<()> {
    let mut total: u32 = 0;

    let file = File::open("/home/tim/prog/adventofcode2023/src/inputs/day2.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut line = line?;
        line = line.strip_prefix("Game ").unwrap().to_string();
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;
        let mut temp_string: String = String::new();
        let mut temp_number: String = String::new();
        for char in line.chars() {
            if char.is_digit(10) {
                temp_number.push(char);
            } else if char == ' ' {
                continue;
            } else if char == ';' || char == ',' || char == ':' {
                temp_number = String::new();
                temp_string = String::new();
                continue;
            } else {
                temp_string.push(char);
                if temp_string.ends_with("red") {
                    if temp_number.parse::<u32>().unwrap() > max_red {
                        max_red = temp_number.parse::<u32>().unwrap();
                    }
                    temp_string = String::new();
                } else if temp_string.ends_with("green") {
                    if temp_number.parse::<u32>().unwrap() > max_green {
                        max_green = temp_number.parse::<u32>().unwrap();
                    }
                    temp_string = String::new();
                } else if temp_string.ends_with("blue") {
                    if temp_number.parse::<u32>().unwrap() > max_blue {
                        max_blue = temp_number.parse::<u32>().unwrap();
                    }
                    temp_string = String::new();
                }
            }
        }
        total += max_red * max_green * max_blue;
    }
    println!("{}", total);
    Ok(())
}
