//! Using Enumeration

fn main() {
    println!("\n");

    let my_options: Options = Options::Five;

    match my_options {
        Options::One => println!("You select One ..."),
        Options::Two => println!("You select Two ..."),
        Options::Three => println!("You select Thee ..."),
        Options::Four => println!("You select Four ..."),
        Options::Five => println!("You select Five ..."),
    }

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}

enum Options {
    One,
    Two,
    Three,
    Four,
    Five,
}
