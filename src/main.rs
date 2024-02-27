use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");
    game_loop(rand::thread_rng().gen_range(1..=100));
}

fn game_loop(secret_number: u32) {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
            let guess = match guess.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Your guess must be a number...");
                    continue;
                }
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too Small"),
                Ordering::Greater => println!("Too Big"),
                Ordering::Equal => {
                    println!("You win");
                    break;
                }
            }
    }
}