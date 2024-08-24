//-----------------------------------------------
//        Size in Rust
//            - Sized Types
//            - Unsized Types
//-----------------------------------------------

use std::mem::size_of;

struct Point {
    _x: i32,
    _y: i32,
}

struct AlignmentCheck {
    _x: u8,
    _y: u32,
    _z: u16,
}

trait _SomeTrait {}

pub fn main() {
    // Sized Types
    println!("\n========= Sized Types =========");
    // Primitives
    println!("Size of i32: {}", size_of::<i32>());
    println!("Size of f64: {}", size_of::<f64>());
    println!("Size of char: {}", size_of::<char>());
    println!("Size of bool: {}", size_of::<bool>());
    println!("Size of usize: {}", size_of::<usize>());

    // Tuples consisting of primitives are also sized
    println!("Size of (i32, f64): {}", size_of::<(i32, f64)>());

    // Array of primitives are also sized
    println!("Size of [i32; 5]: {}", size_of::<[i32; 5]>());

    // Structs consisting of primitives are also sized
    println!("Size of Point: {}", size_of::<Point>());

    // Structs with different alignment and padding
    println!("\nSize of AlignmentCheck: {}", size_of::<AlignmentCheck>());
    println!("Individual Field Sizes of AlignmentCheck:");
    println!("Size of u8: {}", size_of::<u8>());
    println!("Size of u32: {}", size_of::<u32>());
    println!("Size of u16: {}", size_of::<u16>());
    println!("Total padding by compiler: {}", size_of::<AlignmentCheck>() - size_of::<u8>() - size_of::<u32>() - size_of::<u16>());

    // Pointers and References are sized
    println!();
    println!("Size of &i32: {}", size_of::<&i32>());
    println!("Size of &mut i32: {}", size_of::<Box<&mut i32>>());

    // Machine Word Size
    println!();
    println!("Size of Machine Word: {}", size_of::<&()>());

    // Smart pointers
    println!();
    println!("Size of Box<i32>: {}", size_of::<Box<i32>>());
    println!("Size of Rc<i32>: {}", size_of::<std::rc::Rc<i32>>());
    println!("Size of RefCell<i32>: {}", size_of::<std::cell::RefCell<i32>>());

    // Function Pointers
    println!();
    println!("Size of fn(i32) -> i32: {}", size_of::<fn(i32) -> i32>());

    // Unsized Types
    println!("\n========= Unsized Types =========");
    // Slice is unsized
    // println!("Size of [i32]: {}", size_of::<[i32]>()); // Compile Error (Size of [i32] is not known at compile time)
    println!("Size of &[i32]: {}", size_of::<&[i32]>());

    // let _slice: [i32]; // Again compile error (Size of [i32] is not known at compile time)

    let _slice: [i32; 5]; // This is a sized array

    // Trait Objects are unsized
    println!();
    // println!("Size of _SomeTrait: {}", size_of::<_SomeTrait>()); // Compile Error (Size of _SomeTrait is not known at compile time)

    // Reference to trait objects are sized
    println!("Size of &dyn _SomeTrait: {}", size_of::<&dyn _SomeTrait>());
}