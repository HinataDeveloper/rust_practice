//! Using atomic data type.

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

fn main() {
    println!("\n");

    let atomic_one: AtomicI32 = AtomicI32::new(100);

    // atomic_one variable is moved in below line.
    println!(" Value of atomic_one is: {}", atomic_one.into_inner());

    // Program encounter with compile time error
    // Because there is not atomic_one variable at this point.
    atomic_one.store(200, Ordering::Acquire);
    println!(" Value of atomic_one is: {}", atomic_one.into_inner());

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
