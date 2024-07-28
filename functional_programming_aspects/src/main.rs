use functional_programming_aspects::{closures, iterating_through_options};
use functional_programming_aspects::{combinators, functional_pointers, into_iterator, iterating_through_collections, iterators};

fn main() {
    println!("################ Closures ################");
    closures::main();

    println!("################ Functional Pointers ################");
    functional_pointers::main();

    println!("################ Iterators ################");
    iterators::main();

    println!("\n\n################ IntoIterator ################\n");
    into_iterator::main();

    println!("\n\n################ Iterating through collections ################\n");
    iterating_through_collections::main();

    println!("\n\n################ Combinators ################\n");
    combinators::main();

    println!("\n\n################ Iterating through Options ################\n");
    iterating_through_options::main();

}
