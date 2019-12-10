// Refers to https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#programming-a-guessing-game

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    const MIN: u32 = 1;
    const MAX: u32 = 100;

    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(MIN, MAX + 1);

    loop {
        println!("Please input your guess, between {} and {}.", MIN, MAX);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Your guess was no number! ({})", err);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("⬆️  {} is too small!", guess),
            Ordering::Greater => println!("⬇️  {} is too big!", guess),
            Ordering::Equal => {
                println!("✅ It was {}! You win!", guess);
                break;
            }
        }
    }
}
