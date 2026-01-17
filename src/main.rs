// Ready to make a guessing game in rust
// Import the necessary libraries
// Import the std io

use std::io;

// Import the cmp Ordering

use std::cmp::Ordering;

// Import rust crate rand : library crate

use rand::Rng;

// Define the main function

fn main() {
    // Initialize the random number which is of any integer type
    let random = rand::rng().random_range(1..=100);
    // Define a count variable to define counts : mutable type to update
    // Set count type to usize : this is what rust prefers for idiomatic code for counter
    let mut count: usize = 0;
    // The above statement generates a random number between 1 and 100, using gen_range function

    // Start the loop
    loop {
        // Initialize a new empty string @ the start of the loop ----> Ref = 1
        let mut guess = String::new();
        // This is of string type and is stored in heap memory, growable, dynamic size.
        count = count + 1;
        println!("[Guess : {count}] Enter your guess below : ");

        // Now take input from the user as a number
        io::stdin()
            // Read the line and init a mutable borrow from guess variable ----> Reference (1)
            .read_line(&mut guess)
            // Expect an error
            .expect("Failed to read the line");

        // Shadow the variable guess into an immutable i32 type and do error handling ----> Ref = 2
        let guess: i32 = match guess.trim().parse() {
            // Every result is an enum of :
            // Ok : Success result
            Ok(number) => number,
            // Err : Failure mode which can be treated as information to handle ----> Ref = 3
            Err(_) => continue,
            // Here as this part of code ----> Reference 2 code block is placed in loop
            // We will be able to continue the loop without handling the error by using continue
            // We catch all errors regardless of information using _ ----> Reference 3
        };

        // use cmp to compare the guess and random ----> Ref = 4
        // Here also we use &random : immutable borrow : we can read only for cmp, but not mut
        match guess.cmp(&random) {
            Ordering::Less => println!("Your guess is less than actual..."),
            Ordering::Greater => println!("Your guess is greater than actual..."),
            Ordering::Equal => {
                println!("You guessed it right in {count} guesses!...");
                // Break after finding right number
                break;
            }
        }
        println!("___________________________________________________________________________\n");
    }

}
