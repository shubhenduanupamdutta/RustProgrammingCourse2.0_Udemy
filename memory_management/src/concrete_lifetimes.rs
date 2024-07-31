// -------------------------------------------------------------
//            Concrete Lifetimes
// -------------------------------------------------------------


pub fn main() {
    let i = 5;
    let j = i;
    println!("i: {i}");
    println!("j: {j}");
    println!("\n Heap Allocated Data\n");
    heap_allocated_data();

    println!("\nReferences \n");
    references();

    println!("\nMutable and Immutable References\n");
    mutable_and_immutable_references();

}

fn heap_allocated_data() {
    let str_1 = String::from("abcd");
    let str_2 = str_1;
    // println!("str_1: {str_1}"); // This line will not compile
    println!("str_2: {str_2}");
}

fn references() {
    let i;
    {
        let j = 5;
        i = &j;
        println!("i: {i}");
    }
    // println!("i: {i}");  // This line will not compile
}

fn mutable_and_immutable_references() {
    let mut vec_1 = vec![6, 5, 8, 9];
    let ref_1 = &vec_1;
    println!("ref_1: {:?}", ref_1);
    let ref_2 = &mut vec_1;
    ref_2.push(3);
    println!("ref_2: {:?}", ref_2);
}