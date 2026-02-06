//! Using Enumeration

fn main() {
    println!("\n");

    let my_options: Options = Options::Five(String::from("I am Five"));

    match my_options {
        Options::One(msg) => println!("You select One: {}", msg),
        Options::Two(msg) => println!("You select Two: {}", msg),
        Options::Three(msg) => println!("You select Thee: {}", msg),
        Options::Four(msg) => println!("You select Four: {}", msg),
        Options::Five(msg) => println!("You select Five: {}", msg),
    }

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}

enum Options {
    One(String),
    Two(String),
    Three(String),
    Four(String),
    Five(String),
}
