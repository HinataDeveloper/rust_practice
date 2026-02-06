//! Using file

use std::{fs::File, io::Write};

fn main() {
    println!("\n");

    let hello_file_result = File::open("hello.txt");

    let mut hello_file = match hello_file_result {
        Ok(file) => file,
        Err(error) => panic!("error in open file: {}", error),
    };

    match hello_file.write_all(b"I am a Rust Developer") {
        Ok(_) => println!("Message was write down into the file ..."),
        Err(error) => eprintln!("writing message to the file operation was failed: {error}"),
    }

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
