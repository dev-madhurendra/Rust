// Import the standard library module 'io' for input/output operations
use std::io;

// The entry point of the program
fn main() {
    // Print a message to the console
    println!("Guess the number!");

    // Prompt the user to input a guess
    println!("Please input your guess.");

    // Create a mutable String to store the user's input
    // By default all the variables are immutable in rust
    let mut guess = String::new();

    // Read a line from standard input (the console) and store it in the 'guess' variable
    // 'read_line' returns a Result, and 'expect' is used to handle any errors that may occur
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read !");

    // Print the user's guessed value using string interpolation
    println!("You guessed: {}", guess);
}
