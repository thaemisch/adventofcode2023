use std::io;

mod day1;

fn main() {
    println!("Enter day:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u32 = input.trim().parse().expect("Invalid input");

    println!("Enter part:");

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
