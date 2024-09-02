// ----------------------------------------------
//      Understanding Sizes in Rust
// ----------------------------------------------

use sizes_in_rust::{reference_to_unsized, size_unsize_types, sized_optionally_sized_trait, unsized_coercion, zero_sized_types};
use sizes_in_rust::optionally_sized_and_generic_parameters as sized_and_generics;

fn main() {
    println!("####### Size and Unsized Types in Rust #######");
    size_unsize_types::main();

    println!("\n####### Pointers to Sized and Unsized Types #######");
    reference_to_unsized::main();

    println!("\n####### Sized and Optionally Sized Traits #######");
    sized_optionally_sized_trait::main();

    println!("\n####### ?Sized and Generic Parameters #######");
    sized_and_generics::main();

    println!("\n####### Unsized Coercion in Rust #######");
    unsized_coercion::main();

    println!("\n####### Zero Sized Types #######");
    zero_sized_types::main();
}
