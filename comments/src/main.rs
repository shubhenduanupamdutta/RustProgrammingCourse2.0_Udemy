// -----------------------------------------------------
//              - Comments
//              - More on Printing
//              - Inputs
//              - Variable Convention
//              - Statics
// -----------------------------------------------------

use std::io;

fn main() {
    // This is a single line comment
    /* This is a
    multi-line
    comment */

    println!("Hello, World!");
    print!("Hello, ");
    print!("World!");

    // Escape Characters
    println!("Hello, \nWorld!");
    println!("Hello, \tWorld!");
    println!("Hello, \\World!");
    println!("Hello, \"World!\"");
    println!("Hello, \'World!\'");
    println!("Hello \rWorld!");

    // Placeholder
    println!("{} is a {}", "Rust", "systems programming language");
    println!("{0} is a {1} programming language", "Rust", "systems");
    println!("{1} is a {0} programming language", "systems", "Rust");

    // Named Arguments
    println!(
        "{name} is a {desc} programming language",
        name = "Rust",
        desc = "systems"
    );

    // Input
    let mut n = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let n: f64 = n.trim().parse().expect("Invalid input.");
    println!("You entered: {}", n);

    static _WELCOME: &str = "Welcome to Rust!";
}
