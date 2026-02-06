//! Reading from terminal and parsing 
//! input data to a number.

use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    println!("\n");

    let mut calc: Calculate<i32> = Calculate {
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

struct Calculate<T> {
    number_one: T,
    number_two: T,
}

trait CalculateTrait<T> {
    fn set_number_one(&mut self, n1: T);

    fn set_number_two(&mut self, n2: T);

    fn get_number_one(&self) -> T;

    fn get_number_two(&self) -> T;

    fn add(&self) -> T;

    fn subtract(&self) -> T;

    fn multiply(&self) -> T;

    fn divide(&self) -> T;
}

impl<T> CalculateTrait<T> for Calculate<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Copy,
{
    fn set_number_one(&mut self, n1: T) {
        self.number_one = n1;
    }

    fn set_number_two(&mut self, n2: T) {
        self.number_two = n2;
    }

    fn get_number_one(&self) -> T {
        self.number_one
    }

    fn get_number_two(&self) -> T {
        self.number_two
    }

    fn add(&self) -> T {
        self.number_one + self.number_two
    }

    fn subtract(&self) -> T {
        self.number_one - self.number_two
    }

    fn multiply(&self) -> T {
        self.number_one * self.number_two
    }

    fn divide(&self) -> T {
        self.number_one / self.number_two
    }
}

struct StringUtil {}

trait StringUtilTrait<T> {
    fn read_number_from_terminal(msg: &'static str) -> T;
}

impl<T> StringUtilTrait<T> for StringUtil
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn read_number_from_terminal(msg: &'static str) -> T {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        let mut input_data: String = String::new();
        io::stdin()
            .read_line(&mut input_data)
            .expect("Error in read data from terminal ...");

        let result: T = input_data
            .trim()
            .parse()
            .expect("Error in convert input data ...");

        result
    }
}
