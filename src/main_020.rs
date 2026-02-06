//! Using file

use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
};

fn main() {
    println!("\n");

    let hello_file_result = File::open("hello.txt");

    let mut hello_file = match hello_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("error in create file: {error}"),
            },
            _ => panic!("error in open file ..."),
        },
    };

    match hello_file.write_all(b"I am a Rust Developer") {
        Ok(_) => println!("Message was write down into the file ..."),
        Err(error) => eprintln!("writing message to the file operation was failed: {error}"),
    }

    let mut file_buffer = String::new();

    match hello_file.read_to_string(&mut file_buffer) {
        Ok(_) => println!("content of hello.txt is: {} ", file_buffer),
        Err(error) => eprintln!("Error in reading content of file: {error}"),
    }

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
