//! Using atomic data type.

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

fn main() {
    println!("\n");

    let mut atomic_one: AtomicI32 = AtomicI32::new(100);
    println!(" Value of atomic_one is: {}", atomic_one.get_mut());

    atomic_one.store(200, Ordering::Relaxed);
    println!(" Value of atomic_one is: {}", atomic_one.get_mut());

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
