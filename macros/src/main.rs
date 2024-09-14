//----------------------------------------------
//        Macros in Rust
//----------------------------------------------

use macros::{capturing_types, macros_basics};

fn main() {
    println!("############# Macros in Rust #############");

    println!();
    println!("########## 1. Macros Basics ##########");
    macros_basics::main();

    println!();
    println!("########## 2. Capturing Types ##########");
    capturing_types::main();
}