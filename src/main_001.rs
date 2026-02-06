//! Reading from terminal and parsing 
//! input data to a number.

use std::io::{self, Write};

fn main() {
    println!("\n");

    let mut input_data = String::new();

    print!("Please enter a number: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input_data)
        .expect("error in read data from console");

    let number:i32 = input_data
        .trim()
        .parse().unwrap_or(0);
        
    println!("The number you entered is {}", number);
    
    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
