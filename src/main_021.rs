//! Using file

use std::{fs::OpenOptions, io::Read};

fn main() {
    println!("\n");

    let mut hello_file_opn = OpenOptions::new();
    let hello_file_result = hello_file_opn
        .create(true)
        .write(true)
        .append(true)
        .read(true)
        .open("message.txt");

    let mut hello_file = match hello_file_result {
        Ok(file) => file,
        Err(error) => panic!("error in open file: {error}"),
    };

    let mut buffer_file: String = String::new();
    match hello_file.read_to_string(&mut buffer_file) {
        Ok(_) => println!("Content of file: {}", buffer_file),
        Err(error) => panic!("Error reading from file: {error}"),
    }

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
