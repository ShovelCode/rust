// Import the standard input/output library for user interaction.
use std::io;

/// This function calculates the factorial of a non-negative integer.
/// 
/// # Arguments
/// 
/// * `n` - A non-negative integer for which the factorial is calculated.
/// 
/// # Returns
/// 
/// * `u64` - The factorial of the input number.
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// This function prompts the user for a non-negative integer,
/// reads the input, and returns it if valid.
/// 
/// # Returns
/// 
/// * `Result<u64, &'static str>` - Ok(u64) if valid input, Err(&'static str) if invalid.
fn get_user_input() -> Result<u64, &'static str> {
    println!("Enter a non-negative integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Attempt to parse the input into an unsigned 64-bit integer.
    match input.trim().parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Please enter a valid non-negative integer."),
    }
}

/// This is the main function that drives the program.
/// It gets user input, calculates the factorial, and displays the result.
fn main() {
    match get_user_input() {
        Ok(n) => {
            let result = factorial(n);
            println!("The factorial of {} is {}", n, result);
        }
        Err(e) => println!("{}", e),
    }
}
