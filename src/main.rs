// Use the standard Rust library with the IO (input/output) module.
use std::io;

// Application entry point
fn main() {
    // Game title and initial request to the user for input on a guess
    println!("Guess the number!");

    println!("Please input your guess.");

    // Defining a mutable variable named "guess" and defining the data type as "String"
    // String::new() is a function that returns a new instance of a String.
    let mut guess = String::new();

    /* Using the input/output module to take input from the user, store the value/n
     * the mutable variable "guess"
    */
    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read the line");

    // Print to the terminal the input from the user
    println!("You guessed: {guess}");
}
