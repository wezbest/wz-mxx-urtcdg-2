/*
Bank utility functions
here
*/

use std::io;

pub fn get_input_amount() -> i32 {
    loop {
        let mut input = String::new();
        println!("Enter amount to deposit:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(amount) => return amount, // return the valid amount
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
