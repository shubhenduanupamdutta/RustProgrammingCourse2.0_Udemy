use memory_management::{box_smart_pointer, concrete_lifetimes, generic_lifetimes, lifetime_elision, lifetimes_structs, rc_smart_pointer, refcell_pointer_example, refcell_smart_pointer};

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


    println!("\n############ Smart Pointers in Rust ############\n");
    println!("1. Box Smart Pointer\n");
    box_smart_pointer::main();

    println!("\n2. Reference Counting (RC) Smart Pointer\n");
    rc_smart_pointer::main();

    // println!("\n3. RefCell Smart Pointer\n");
    refcell_smart_pointer::main();

    println!("\n4. RC Pointer Example\n");
    refcell_pointer_example::main();

}
