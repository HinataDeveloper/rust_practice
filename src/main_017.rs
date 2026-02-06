//! Using Thread

use std::thread;

fn main() {
    println!("\n");

    let mut handles = vec![];

    for item in 1..15 {
        let task = thread::spawn(move || {
            println!("thread number ({}) is running ...", item);
            item * 20
        });
        handles.push(task);
    }

    let mut sum: i32 = 0;
    for handle in handles {
        sum += handle.join().unwrap();
    }

    println!("Summary is: {}", sum);

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
