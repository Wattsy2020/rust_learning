use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::{IntErrorKind, ParseIntError};

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn gen_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn main() -> () {
    println!("Guess the number!");
    let rand_number = gen_random_number();

    loop {
        println!("Please input your guess: ");
        let guess = read_input();
        let trimmed = guess.trim();
        if trimmed == "exit" {
            break;
        }
        let guess_num: u32 = match trimmed.parse() {
            Ok(num) => num,
            Err(err) => {
                match err.kind() {
                    IntErrorKind::Empty => println!("Please enter a number"),
                    IntErrorKind::NegOverflow => println!("Please enter a positive number"),
                    IntErrorKind::PosOverflow => println!("Please enter a number < 100"),
                    _ => println!("Error: {err}, {trimmed} is not a number")
                }
                continue;
            }
        };
        println!("You guessed: {guess_num}");

        match guess_num.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
