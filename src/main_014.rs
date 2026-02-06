//! Using Atomic Data Type

use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;

fn main() {
    println!("\n");

    let mut inc: Incrementor = Incrementor {
        count: AtomicI64::new(0),
    };

    println!(" {} ", inc.next());
    println!(" {} ", inc.next());
    println!(" {} ", inc.next());
    println!(" {} ", inc.next());
    println!(" {} ", inc.next());
    println!(" {} ", inc.next());

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
