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
    let mut total: i32 = 0;

    let file = File::open("/home/tim/prog/adventofcode2023/src/inputs/day1.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut numbers = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                numbers.push(c);
            }
        }
        let mut first_plus_last = String::new();
        if numbers.chars().count() == 1 {
            first_plus_last.push(numbers.chars().nth(0).unwrap());
            first_plus_last.push(numbers.chars().nth(0).unwrap());
        } else {
            first_plus_last.push(numbers.chars().nth(0).unwrap());
            first_plus_last.push(numbers.chars().nth(numbers.len() - 1).unwrap());
        }

        let subtotal: i32 = first_plus_last.parse::<i32>().unwrap();
        total += subtotal;
    }

    println!("{}", total);
    Ok(())
}

fn part_two() -> io::Result<()> {
    let mut total: i32 = 0;

    let file = File::open("/home/tim/prog/adventofcode2023/src/inputs/day1.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: String = transform_string(&line);
        let mut first_plus_last = String::new();
        if numbers.chars().count() == 1 {
            first_plus_last.push(numbers.chars().nth(0).unwrap());
            first_plus_last.push(numbers.chars().nth(0).unwrap());
        } else {
            first_plus_last.push(numbers.chars().nth(0).unwrap());
            first_plus_last.push(numbers.chars().nth(numbers.len() - 1).unwrap());
        }

        let subtotal: i32 = first_plus_last.parse::<i32>().unwrap();
        total += subtotal;
    }

    println!("{}", total);
    Ok(())
}

fn transform_string(input: &str) -> String {
    let mut result = String::new();
    let numbers = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut temp = String::new();
    for c in input.chars() {
        if c.is_digit(10) {
            result.push(c);
        } else if c.is_alphabetic() {
            temp.push(c);
            for &(num_str, num_char) in &numbers {
                if temp.ends_with(num_str) {
                    result.push(num_char);
                    temp.clear();
                    break;
                }
            }
        }
    }
    result
}
