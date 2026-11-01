use std::{cmp::Ordering, io::{self, Write}};
use rand::prelude::*;

fn main() {
    let secret_number:u8 = rand::rng().random_range(1..10);

    loop {
        print!("Guess the number: ");
        io::stdout().flush().unwrap();

        let mut guessed_number = String::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to readline");

        let guessed_number: u8 = match guessed_number.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Invalid input: {e}");
                continue;
            },
        };

        match guessed_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Your guess of {guessed_number} is correct!");
                break;
            }
            Ordering::Greater => println!("Your guess is too big!"),
            Ordering::Less => println!("Your guess is too small!")
        }
    }
}
