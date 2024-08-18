//-------------------------------------------------
//           Typical Data Structures
//-------------------------------------------------

use typical_data_structures::{doubly_linked_list, linked_list, reference_cycles};

fn main() {
    println!("############## Typical Data Structures in Rust. ##############\n");

    println!("\n============ 1. Singly Linked List ============\n");
    linked_list::main();

    println!("\n============ 2. Doubly Linked List ============\n");
    doubly_linked_list::main();

    println!("\n############## Reference Cycles ##############\n");
    reference_cycles::main();

}