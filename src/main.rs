use std::{io, cmp::Ordering};
use rand::Rng;

// Simple program to take input from a user
fn main() {
    println!("hello YOU!");

    println!("Guess the secret number!");
    let secret = rand::thread_rng().gen_range(1..=50);
    println!("please enter your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read your guess.");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret) {
        Ordering::Equal => println!("You guessed it right!! its was {secret}"),
        Ordering::Greater => println!("Ah a little lower and you would've guesses it right! its was {secret}"),
        Ordering::Less => println!("A little higher! nvm, try again later its was {secret}"),
    }
    

    // println!("please enter 2nd your guess.");

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("failed to read your guess.");
    // let guessed = guess.trim();

    print!("your guess: {guess}");
}