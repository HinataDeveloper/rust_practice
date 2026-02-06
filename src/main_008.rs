//! Using atomic data type.

use std::sync::atomic::AtomicI32;

fn main() {
    println!("\n");

    let mut one: AtomicI32 = AtomicI32::new(100);
    println!("value of one is: {}", one.get_mut());

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}
