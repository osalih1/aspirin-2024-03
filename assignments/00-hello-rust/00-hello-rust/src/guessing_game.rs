//! A simple number guessing game.
//! This program generates a random number between 1 and 100 and allows the user to guess it.
//! The program gives feedback if the guess is too high, too low, or correct, and continues until the correct guess is made.

#![warn(missing_docs)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Reads input from the user and returns the guessed number as an integer.
///
/// # Panics
/// This function will panic if the user inputs a non-integer value.
fn get_input() -> i32 {
    println!("Please input your guess");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // Expects input to be successfully read from the console.

    // Attempts to parse the input as an integer. Panics if parsing fails.
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// Main function that drives the guessing game.
/// Generates a random number, accepts user guesses, and gives feedback until the correct number is guessed.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Random number between 1 and 100.

    loop {
        let guess = get_input(); // Get user's guess.
        print!("You guessed: {}. ", guess);

        // Compares guess to the secret number and gives feedback accordingly.
        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!"); // End the game if the guess is correct.
                break;
            }
            Ordering::Greater => println!("Your guess is too low."), // Inform the user if the guess is too low.
            Ordering::Less => println!("Your guess is too high."), // Inform the user if the guess is too high.
        }
    }
}
