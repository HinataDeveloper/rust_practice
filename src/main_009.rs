//! Using atomic data type.

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

fn main() {
    println!("\n");

    let atomic_one: AtomicI32 = AtomicI32::new(100);
    let atomic_two: AtomicI32 = AtomicI32::new(200);

    let fetch_result = atomic_one.fetch_add(atomic_two.into_inner(), Ordering::Acquire);

    println!(" value of fetch_result is: {}", fetch_result);
    println!(" value of add_result is: {}", atomic_one.into_inner());

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
