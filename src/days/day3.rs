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

    let file = File::open("/home/tim/prog/adventofcode2023/src/inputs/test.txt")?;
    let reader = io::BufReader::new(file);

    let mut symbols: Vec<Vec<u32>> = Vec::new();
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    let mut temp_number: String = String::new();
    let mut temp_coord: Vec<u32> = Vec::new();
    let mut line_number: u32 = 0;
    for line in reader.lines() {
        let mut char_number: u32 = 0;
        let line = line?;
        for char in line.chars() {
            if char == '.' {
                if temp_number.len() > 0 {
                    numbers.push(vec![
                        temp_number.parse::<u32>().unwrap(),
                        temp_coord[0],
                        temp_coord[1],
                        char_number,
                    ]);
                    temp_number = String::new();
                    temp_coord = Vec::new();
                }
            } else if char.is_digit(10) {
                temp_number.push(char);
                temp_coord.push(line_number);
                temp_coord.push(char_number);
            } else {
                symbols.push(vec![char as u32, line_number, char_number]);
                if temp_number.len() > 0 {
                    numbers.push(vec![
                        temp_number.parse::<u32>().unwrap(),
                        temp_coord[0],
                        temp_coord[1],
                        char_number,
                    ]);
                }
                temp_number = String::new();
                temp_coord = Vec::new();
            }
            char_number += 1;
        }
        line_number += 1;
    }
    for number in numbers {
        let mut first_coord = number[2];
        if first_coord > 0 {
            first_coord -= 1;
        }
        for symbol in &symbols {
            if symbol[1] == number[1] {
                if symbol[2] == first_coord || symbol[2] == number[3] {
                    total += number[0];
                    break;
                }
            } else if symbol[1] == number[1] + 1 {
                if symbol[2] >= first_coord && symbol[2] <= number[3] {
                    total += number[0];
                    break;
                }
            } else if number[1] > 0 {
                if symbol[1] == number[1] - 1 {
                    if symbol[2] >= first_coord && symbol[2] <= number[3] {
                        total += number[0];
                        break;
                    }
                }
            }
        }
    }
    println!("{}", total);
    Ok(())
}

fn part_two() -> io::Result<()> {
    Ok(())
}
