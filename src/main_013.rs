//! Using Thread

use std::thread;
use std::time::Duration;

fn main() {
    println!("\n");

    let handle_one = thread::spawn(|| {
        for item in 1..=50 {
            println!(" .{}.", item);
            thread::sleep(Duration::from_millis(500));
        }
    });

    let handle_two = thread::spawn(|| {
        for item in 300..=350 {
            println!(" ...{}...", item);
            thread::sleep(Duration::from_millis(300));
        }
    });

    handle_one.join().unwrap();
    handle_two.join().unwrap();

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
