//! Using Thread

use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n");

    let mut inc_one: Incrementor = Incrementor {
        count: AtomicI64::new(0),
    };

    let mut inc_two: Incrementor = Incrementor {
        count: AtomicI64::new(0),
    };

    let task_one = thread::spawn(move || {
        for item in 1..15 {
            println!("-->> {} - {} ", item, inc_two.next());
            thread::sleep(Duration::from_millis(500));
        }
    });

    for item in 1..15 {
        println!(">>> {} - {} ", item, inc_one.next());
        thread::sleep(Duration::from_millis(500));
    }

    task_one.join().unwrap();

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}

struct Incrementor {
    count: AtomicI64,
}

trait IncTrait {
    fn next(&mut self) -> i64;
}

impl IncTrait for Incrementor {
    fn next(&mut self) -> i64 {
        self.count.fetch_add(1, Ordering::Acquire)
    }
}
