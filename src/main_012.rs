//! Using Thread

use std::thread;
use std::time::Duration;

fn main() {
    println!("\n");

    let handle = thread::spawn(|| {
        for item in 1..5 {
            println!(" .{}.", item);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for item in 10..15 {
        println!(" ...{}... ", item);
        thread::sleep(Duration::from_millis(400));
    }

    handle.join().unwrap();

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
