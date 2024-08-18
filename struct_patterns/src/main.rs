//----------------------------------------------
//       Useful Struct Patterns
//----------------------------------------------

use struct_patterns::{builder_pattern, initializing_struct, simplifying_structs};

fn main() {
    println!("########### Useful Struct Patterns ###########");
    println!();
    println!("1. Initializing Struct Instance");
    initializing_struct::main();

    println!();
    println!("2. Builder Pattern");
    builder_pattern::main();

    println!();
    println!("3. Simplifying Structs");
    simplifying_structs::main();
}