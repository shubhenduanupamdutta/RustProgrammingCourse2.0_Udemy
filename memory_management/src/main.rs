use memory_management::{concrete_lifetimes, generic_lifetimes, lifetime_elision, lifetimes_structs};

fn main() {
    println!("############ Lifetimes in Rust ############\n");
    println!("1. Concrete Lifetimes\n");
    concrete_lifetimes::main();

    println!("2. Generic Lifetimes\n");
    generic_lifetimes::main();

    println!("3. Lifetime Elision\n");
    lifetime_elision::main();

    println!("4. Lifetime Annotations in Structs\n");
    lifetimes_structs::main();

}
