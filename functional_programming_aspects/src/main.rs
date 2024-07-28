use functional_programming_aspects::closures;
use functional_programming_aspects::functional_pointers;
use functional_programming_aspects::into_iterator;
use functional_programming_aspects::iterating_through_collections;
use functional_programming_aspects::iterators;

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

}
