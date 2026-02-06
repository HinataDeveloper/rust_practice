//! Reading from terminal and parsing 
//! input data to a number.

use std::io::{self, Write};

fn main() {
    println!("\n");

    let mut calc: Calculate = Calculate {
        number_one: 0,
        number_two: 0,
    };

    let message = "Please enter first number: ";
    let number_one: i32 = StringUtil::read_number_from_terminal(message);
    calc.set_number_one(number_one);

    let message = "Please enter second number: ";
    let number_two: i32 = StringUtil::read_number_from_terminal(message);
    calc.set_number_two(number_two);

    let add_result = calc.add();
    let subtract_result = calc.subtract();
    let multiply_result = calc.multiply();
    let divide_result = calc.divide();

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    println!(" Number One is: {}", calc.get_number_one());
    println!(" Number Two is: {}", calc.get_number_two());

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    println!(" {} + {} = {}", number_one, number_two, add_result);
    println!(" {} - {} = {}", number_one, number_two, subtract_result);
    println!(" {} * {} = {}", number_one, number_two, multiply_result);
    println!(" {} / {} = {}", number_one, number_two, divide_result);

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}

struct Calculate {
    number_one: i32,
    number_two: i32,
}

impl Calculate {
    fn set_number_one(&mut self, n1: i32) {
        self.number_one = n1;
    }

    fn set_number_two(&mut self, n2: i32) {
        self.number_two = n2;
    }

    fn get_number_one(&self) -> i32 {
        self.number_one
    }

    fn get_number_two(&self) -> i32 {
        self.number_two
    }

    fn add(&self) -> i32 {
        self.number_one + self.number_two
    }

    fn subtract(&self) -> i32 {
        self.number_one - self.number_two
    }

    fn multiply(&self) -> i32 {
        self.number_one * self.number_two
    }

    fn divide(&self) -> i32 {
        self.number_one / self.number_two
    }
}

struct StringUtil {}

impl StringUtil {
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
}
