# Borrowing in Rust
### - Borrowing is a fundamental concept in Rust Ownership System, that allows multiple part of the program to interact with data, in safe and efficient manner.


## What is borrowing?
- Borrowing is a way to establish a reference to a value/data.
- Reference is similar to a pointer, but comes with some rules and limitations.
- References doesn't take ownership of the value, that's why it's called borrowing.


## Motivation of borrowing
- Prevent unnecessary copying of data and memory usage.
- Ownership is not required in all cases, borrowing allows to interact with data without taking ownership.

## Rules of borrowing in Rust
- **At any time, you can have either one mutable reference or any number of immutable references, but not both at the same time.**
- **References must always be valid.**
- These rules solve out two problems of,
    - Data Races
    - Dangling References

## Example
```rust
fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let ref1 = &mut vec_1;
    let ref2 = &mut vec_1;

    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);
}
```
- In the above code, we are trying to create two mutable references to the same vector, which is not allowed by the Rust compiler.
- If there was no print statement, compiler will not show an error, because compiler will also track the lifetime of the references.
    - If there is no print statement, lifetime of `ref1` ends before `ref2`, so it's valid.
    - Thus following only one mutable reference rule.

## Example
```rust
fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let ref1 = vec_1;
    let ref2 = vec_1;

    println!("ref1: {:?}", ref1);
    println!("ref2: {:?}", ref2);
}
```
- Above code will compile and work, because we can have multiple immutable references at the same time.
- By this first rule, rust avoids problems of data races.
    - Data races happen when two or more pointers access the same data at the same time, and try to change it.
    - Rust avoids this by allowing only one mutable reference at a time.

## Example
```rust
fn main() {
    let vec_2 = {
        let vec_3 = vec![1, 2, 3];
        &vec_3
    };
}
```
- Above code will not compile, because the reference `vec_3` is dropped before it's returned.

## Borrowing in function
```rust
fn main() {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1.clone());
    println!("vec 1 is: {:?}", vec_1);
}

fn takes_ownership(vec: Vec<i32>) {
    println!("vec is: {:?}", vec);
}
```
- Above code will compile and work, because we are passing a clone of `vec_1` to the function `takes_ownership`, not the original `vec_1`.
- But `clone` is not always a good solution, because it will create a copy of the data on the heap, which is not always efficient.
- So, we can use borrowing to pass the reference of the data to the function, without taking ownership.
```rust
fn main() {
    let vec_1 = vec![1, 2, 3];
    let ref1 = &vec_1;
    takes_ownership(ref1);
    println!("vec 1 is: {:?}", vec_1);
}

fn takes_ownership(vec: &Vec<i32>) {
    println!("vec is: {:?}", vec);
}
```

## `ref` Keyword
- Sometime you may want to borrow some portion of the tuple as reference, while other as owned value.
- In such cases, you can use `ref` keyword.
```rust
fn main() {
    let tuple = (String::from("Hello"), 5);
    let (ref s, num) = tuple;
}
```
- In the above code, `s` is a reference to the first element of the tuple, while `num` is the owned value of the second element.