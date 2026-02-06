//! Reading from terminal and parsing 
//! input data to a number.


use std::io::{self, Write};

fn main() {
    println!("\n");

    let message = "Please enter first number: ";
    let number_one: i32 = read_number_from_terminal(message);

    let message = "Please enter second number: ";
    let number_two: i32 = read_number_from_terminal(message);

    let add_result = add(number_one, number_two);
    let subtract_result = subtract(number_one, number_two);
    let multiply_result = multiply(number_one, number_two);
    let divide_result = divide(number_one, number_two);

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!(" {} + {} = {}", number_one, number_two, add_result);
    println!(" {} - {} = {}", number_one, number_two, subtract_result);
    println!(" {} * {} = {}", number_one, number_two, multiply_result);
    println!(" {} / {} = {}", number_one, number_two, divide_result);

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

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn subtract(n1: i32, n2: i32) -> i32 {
    n1 - n2
}

fn multiply(n1: i32, n2: i32) -> i32 {
    n1 * n2
}

fn divide(n1: i32, n2: i32) -> i32 {
    n1 / n2
}
