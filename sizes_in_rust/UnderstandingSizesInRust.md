# Understanding Sizes in Rust
----------------------------------------------
- Understanding size related issues in Rust is integral to writing safe, performant and efficient code.
- It allows you to make informed decisions, design better data structures and ensures memory safety throughout your code.
- From size perspective there are two types,
    - `Sized Types` - A type whose size is known at compile time.
    - `Unsized Types` - A type whose size is not known at compile time.
- `Dynamically Sized Types` - A different name for unsized types.

========================================================
## Sized Types
========================================================
- Some sized data types are,
    - `primitives` - All primitives are sized.
    - `tuples of primitives` - All tuples of primitives are sized.
    - `arrays of primitives` - All arrays of primitives are sized.
```rust
use std::mem::size_of;

pub fn main() {
    // Sized Types
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
}
```
```shell
Size of i32: 4
Size of f64: 8
Size of char: 4
Size of bool: 1
Size of usize: 8
Size of (i32, f64): 16
Size of [i32; 5]: 20
```
- All types constructed from primitive types, including structs, enums, HashMaps and Arrays are also sized.
- Consider a struct `Point` which is made up of two `i32` values.
```rust
use std::mem::size_of;

struct Point {
    _x: i32,
    _y: i32,
}

fn main() {
    println!("Size of Point: {}", size_of::<Point>());
}
```
```shell
Size of Point: 8
```
- The size of `Point` is `8` bytes, because it is made up of two `i32` values. Size of struct is determined by the sizes of its fields and padding added by the compiler for alignment.

- `Pointers` and `References` are also of fixes sizes, regardless of they being mutable or immutable.
```rust
println!("Size of &i32: {}", size_of::<&i32>());
println!("Size of &mut i32: {}", size_of::<Box<&mut i32>>());
```
```shell
Size of &i32: 8
Size of &mut i32: 8
```
- `Pointers` are one machine word in size, `A Machine Word` refers to the unit of data that a particular computer architecture can handle in one operation. The size of
machine word is architecture dependent, for 64-bit systems it is 8 bytes.

- `Smart Pointers` like `Box`, `Rc`, `RefCell` and `Arc` are also sized, because they are essentially wrappers around pointers.
```rust
// Machine Word Size
println!();
println!("Size of Machine Word: {}", size_of::<&()>());

// Smart pointers
println!();
println!("Size of Box<i32>: {}", size_of::<Box<i32>>());
println!("Size of Rc<i32>: {}", size_of::<std::rc::Rc<i32>>());
println!("Size of RefCell<i32>: {}", size_of::<std::cell::RefCell<i32>>());
println!("Size of Arc<i32>: {}", size_of::<std::sync::Arc<i32>>());
```
```shell
Size of Machine Word: 8

Size of Box<i32>: 8
Size of Rc<i32>: 8
Size of RefCell<i32>: 16
Size of Arc<i32>: 16
```
- `Function Pointers` are also of fixes sizes, since they are essentially also pointers.
```rust
println!("Size of fn(i32) -> i32: {}", size_of::<fn(i32) -> i32>());
```
```shell
Size of fn(i32) -> i32: 8
```
========================================================
## Unsized Types
========================================================
- Slice of an integer array is an unsized type.
- We can't even use `size_of` because compiler will not compile the code.
- Since a slice is just a reference to a contiguous section of elements in a collection, so its size is not known at compile time.
- In literature or in community `[i32]` is not referred to as an array slice, by an array slice we refer to `&[i32]`.
- Let's from here on refer to `[i32]` as `raw slice`.
- Compiler wil throw an error if we also want to define a variable of type `[i32]`.
```rust
use std::mem::size_of;

fn main() {
    // Unsized Types
    // Raw Slice
    println!("Size of [i32]: {}", size_of::<[i32]>()); // Compiler Error
    let _slice: [i32]; // Compiler Error

    // But if we add a size to it, the compiler error will go away, because it will convert from a raw slice to an array
    let _slice: [i32; 5];
}
```
- To properly use a raw slice, we must always use a reference to it.
- Similar to raw slice, we also have raw string slice `str`, whose size is also not known at compile time, same error will occur here also.
- If we again use a reference to it, the error will go away.
```rust
use std::mem::size_of;

fn main() {
    // Raw String Slice
    println!("Size of str: {}", size_of::<str>()); // Compiler Error
    let _str: str; // Compiler Error

    // But if we add a size to it, the compiler error will go away, because it will convert from a raw string slice to a string
    let _str: &str;
}
```
- `Trait Objects` are also unsized types, because their size is not known at compile time.
```rust
use std::mem::size_of;

trait Shape {
    fn area(&self) -> f64;
}

println!("Size of dyn Shape: {}", size_of::<dyn Shape>());  // Compiler Error
```
- This is because traits can be implemented by any numbers of enums and structs with different sizes, so their size is not known at compile time.
- Here also we need to use a reference to it.
```rust
use std::mem::size_of;

trait Shape {
    fn area(&self) -> f64;
}

println!("Size of &dyn Shape: {}", size_of::<&dyn Shape>());
```
- Any complex types like enums, structs made of unsized types are also unsized.

========================================================
## Pointers to Sized and Unsized Types
========================================================
```rust
use std::mem::size_of;

pub fn main() {
    println!("Size of a reference or sized type: {}", size_of::<&[i32, 3]>());
    println!("Size of a reference to an unsized type: {}", size_of::<&[i32]>());
}
```
- Let's consider above code, `&[i32, 3]` is a reference to an array of size `3`, which is a sized type, so its size is known at compile time.
- `&[i32]` is a reference to an array slice. Array slice is an unsized type, so its size is not known at compile time.
- Let's check the output of above code.
```shell
Size of a reference or sized type: 8
Size of a reference to an unsized type: 16
```
- Why is reference to an unsized type is `16` bytes?
- Simple reference to sized types are one Machine Word (i.e. 8 bytes in 64-bit systems), but reference to unsized types are two Machine Words (i.e. 16 bytes in 64-bit systems).
- Why is this so?
- Let's define an array containing 3 elements,
```rust
let num_1: &[i32; 3] = &[1, 2, 3];
let num_2: &[i32] = &[1, 2, 3];
```
- In the first case, data length information is embedded in the type information itself. There for size information is not required to be stored again.
- But in second case, data length information is not attached to the type itself, therefore pointer needs to have some additional information (which may be mutable). That added information about size of data type is stored in next Machine Word size.
- Reference which are two machine word are referred to as `Fat Pointers`.
- References which are only one machine word are referred to as `Thin Pointers`.
- To confirm, that the size information is stored in the pointer itself in second case, we can iterate through the two data.
```rust
let num_1: &[i32; 3] = &[1, 2, 3];
let num_2: &[i32] = &[1, 2, 3];

let mut sum_1 = 0;
for num in num_1 {
    sum_1 += num;
}

println!("Sum of the array num_1: {}", sum_1);

let mut sum_2 = 0;
for num in num_2 {
    sum_2 += num;
}
println!("Sum of the slice num_2: {}", sum_2);
```
- We expect the second loop to throw an error, because size data is not explicitly stated anywhere, but it will compile and run successfully.
- Because size data is stored in the pointer itself, making the pointer a `Fat Pointer`.
- References to trait objects are also two machine words in size, because size information is stored in the pointer itself.
```rust

use std::mem::size_of;

trait Shape {
    fn print(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    println!();
    println!("Size of reference to Circle: {}", size_of::<&Circle>());
    println!("Size of reference to Rectangle: {}", size_of::<&Rectangle>());
    println!("Size of a reference to a trait object: {}", size_of::<&dyn Shape>());
}
```
- Let's check the output of above code.
```shell
Size of reference to Circle: 8
Size of reference to Rectangle: 8
Size of a reference to a trait object: 16
```
- As expected, reference to trait object is `16` bytes, why is this, there is no length data needs to associated with trait objects.
- In this case the extra 8 bytes will be used to point to the virtual table or `vtable`. `vtable` is a table of function pointers associated with the trait.
- When a trait object is created rust generates a `vtable` that contains pointers to the actual implementation of the trait methods for the specific types.