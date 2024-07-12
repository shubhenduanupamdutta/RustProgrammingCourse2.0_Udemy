use std::vec;

fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let ref1 = &mut vec_1;
    let ref2 = &mut vec_1;

    // println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);

    println!("###### Borrowing in Functions ######");
    borrowing_in_functions();

    println!("###### Dereferencing a Reference ######");
    dereferencing_a_reference();
}

// -----------------------------------------------------
//             Borrowing in Functions
// -----------------------------------------------------

/*
    - At any time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.
*/

fn borrowing_in_functions() {
    let mut vec_1 = vec![1, 2, 3];
    let ref1 = &vec_1;
    borrows_vec(ref1);

    let ref2 = &mut vec_1;
    mutably_borrows_vec(ref2);

    let _vec_2 = gives_ownership();

    println!("vec_1: {:?}", vec_1);
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("vec: {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(4);
    println!("vec: {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    let vec = vec![1, 2, 3];
    vec // Error: `vec` does not live long enough
}

// -----------------------------------------------------
//            Dereferencing a Reference
// -----------------------------------------------------
fn dereferencing_a_reference() {
    let mut some_data: i32 = 42;
    println!("some_data: {}", some_data);
    let ref1 = &mut some_data;
    println!("ref1: {}", ref1);
    let deref_copy = *ref1;
    println!("deref_copy: {}", deref_copy);
    *ref1 = 13;
    println!("some_data: {}, deref_copy: {}", some_data, deref_copy);


    // Heap allocated data behaves differently
    let mut heap_data = vec![5, 6, 7];
    let ref1 = &mut heap_data;
    // let deref_copy = *ref_1; // Error: cannot move out of borrowed content (reference can't be used to pass ownership)
    
    let deref_copy = ref1.clone(); // compiler automatically dereferences the reference

    // References typically reside in stack, stack allocated data are typically copied and not moved
    let move_out = ref1;
    println!("move_out: {:?}", move_out);

    let move_out_again = ref1;
}
