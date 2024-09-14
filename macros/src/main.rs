//----------------------------------------------
//        Macros in Rust
//----------------------------------------------

use macros::{capturing_types, macros_basics, repeating_patterns};

fn main() {
    println!("############# Macros in Rust #############");

    println!();
    println!("########## 1. Macros Basics ##########");
    macros_basics::main();

    println!();
    println!("########## 2. Capturing Types ##########");
    capturing_types::main();

    println!();
    println!("########## 3. Repeating Patterns ##########");
    repeating_patterns::main();
}