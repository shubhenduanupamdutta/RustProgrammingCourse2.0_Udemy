// ----------------------------------------------
//      Understanding Sizes in Rust
// ----------------------------------------------

use sizes_in_rust::{reference_to_unsized, size_unsize_types, sized_optionally_sized_trait};

fn main() {
    println!("####### Size and Unsized Types in Rust #######");
    size_unsize_types::main();

    println!("\n####### Pointers to Sized and Unsized Types #######");
    reference_to_unsized::main();

    println!("\n####### Sized and Optionally Sized Traits #######");
    sized_optionally_sized_trait::main();
}
