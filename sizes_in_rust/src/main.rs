// ----------------------------------------------
//      Understanding Sizes in Rust
// ----------------------------------------------

use sizes_in_rust::{reference_to_unsized, size_unsize_types};

fn main() {
    println!("####### Size and Unsized Types in Rust #######");
    size_unsize_types::main();

    println!("\n####### Pointers to Sized and Unsized Types #######");
    reference_to_unsized::main();
}
