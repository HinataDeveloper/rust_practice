//! Reading from terminal and parsing 
//! input data to a number.


use std::io::{self, Write};

fn main() {
    println!("\n");

    let message = "Please enter a number: ";

    let number: i32 = read_number_from_terminal(message);
    println!("The number that you entered is: {}", number);

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}

fn read_number_from_terminal(msg: &'static str) -> i32 {
    print!("{}", msg);
    io::stdout().flush().unwrap();

    let mut input_data: String = String::new();
    io::stdin()
        .read_line(&mut input_data)
        .expect("Error in read data from terminal ...");

    let result: i32 = input_data.trim().parse().unwrap_or(0);

    result
}
