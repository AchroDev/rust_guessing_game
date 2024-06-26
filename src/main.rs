// Use the standard Rust library with the IO (input/output) module.
// ADDED: Rand library to generate random numbers
// ADDED: Ordering enum to compare the guessed and actual value
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Application entry point
fn main() {
    // Game title and initial request to the user for input on a guess
    println!("Guess the number!");

    // Defines variable "secret_number" and assigns a random value between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // Moved main section of game to the inside of a loop so that it doesn't end on one turn
    loop {
        println!("Please input your guess.");

        // Defining a mutable variable named "guess" and declaring the data type as "String"
        // String::new() is a function that returns a new instance of a String.
        let mut guess = String::new();

        /* Using the input/output module to take input from the user, store the value/n
        * the mutable variable "guess"
        */
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

        // Converting the "guess" string into an integer and setting failure handling
        // trim is used to eliminate any whitespaces and parse converts the string
        // Switched from an expect call to "match" expression to handle the error instead of crashing
        let guess: u32 = match guess.trim().parse() {
            // if there isn't an error, output the number given
            Ok(num) => num,
            // if there is an error, ignore it and continue running the loop
            Err(_) => continue,
        };
            
        // Print to the terminal the input from the user
        println!("You guessed: {guess}");

        // Using the "match" expression with ".cmp" method to compare "guess" with "secret_number"
        // and decide what to do with the return value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations, you guessed correctly!");
                // Added a break to end the program after correctly guessing the number
                break;
            }
        }
    }
}

