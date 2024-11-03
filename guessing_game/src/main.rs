use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::{IntErrorKind};

const MAX_GUESSING_NUMBER: u8 = 100;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn gen_random_number() -> u8 {
    rand::thread_rng().gen_range(1..=MAX_GUESSING_NUMBER)
}

fn main() -> () {
    println!("Guess a number between 1 and {MAX_GUESSING_NUMBER}");
    let rand_number = gen_random_number();

    loop {
        println!("Please input your guess: ");
        let guess = read_input();
        let trimmed = guess.trim();
        if trimmed == "exit" {
            break;
        }
        let guess_num: u8 = match trimmed.parse() {
            Ok(num) => num,
            Err(err) => {
                match err.kind() {
                    IntErrorKind::Empty => println!("Please enter a number"),
                    IntErrorKind::NegOverflow => println!("Please enter a positive number"),
                    IntErrorKind::PosOverflow => println!("Please enter a number < {MAX_GUESSING_NUMBER}"),
                    _ => println!("Error: {err}, {trimmed} is not a number")
                }
                continue;
            }
        };
        if guess_num > MAX_GUESSING_NUMBER {
            println!("Please enter a number < {MAX_GUESSING_NUMBER}");
            continue;
        }
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
