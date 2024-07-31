# Memory Management Features
--------------------------------------------------------
## Lifetimes
--------------------------------------------------------
- One of the most important features of Rust is memory safety.
- Rust guarantees that there will be no dangling references. This is accomplished by the help of `lifetimes`, which are checked by borrow checker at compile time.
- `Lifetimes` may be better understood by dividing them into `concrete lifetimes` and `generic lifetimes`.

========================================================
### Concrete Lifetimes
========================================================
- `Concrete Lifetime` is the time during which a value exists inside the memory.
- The lifetime of a value starts, when it is created, and ends when the value is dropped or moved out, from a particular memory location, due to change in ownership.
```rust
pub fn main() {
    let i = 5;
    let j = i;
    println!("i: {i}");
}
```
- Lifetime of value `i` starts when it is defined and ends when its value is dropped, which happens when the main function ends.
- Similarly for `j`, its lifetime starts when it is defined and ends when its value is dropped, which happens when the main function ends.
```rust
fn main() {
    {
        let i = 5;
    }
    let j = i;  // Will throw an error
    prinln!("j: {j}");
}
```
- In the above code, the lifetime of `i` is limited to the block in which it is defined. So, when the block ends, the value of `i` is dropped and the variable `i` is no longer available in the memory.
- That's why line `let j = i;` will throw an error, because `i` is no longer available in the memory, its lifetime has ended.
- Let's look at another example, with heap allocated memory.
```rust
fn heap_allocated_data() {
    let str_1 = String::from("abcd");
    let str_2 = str_1;
    println!("str_1: {str_1}"); // This line will not compile

}
```
- In the above code, the lifetime of `str_1` starts when it is defined and ends when its value is dropped, which happens when `str_1` data is moved to `str_2`. Due to this change of ownership, lifetime of `str_1` ends and it is no longer available in the memory.
- Let's look at an example with reference.
```rust
fn references() {
    let i;
    {
        let j = 5;
        i = &j;
    }
    println!("i: {i}"); // Will not compile
}
```
- Above code will not compile because of `dangling reference`.
-  `Dangling Reference` is a reference that points to a memory location that has been deallocated.
- To prevent `dangling references`, borrow checker verifies during compile time that a reference lifetime is confined withing the lifetime of the value it is referring to.
- However in this case while `i` is still in scope, the value it is referring to, `j`, is no longer in scope. So, the reference `i` is pointing to a variable whose lifetime has already ended, that is `i` is not confined to the lifetime of `j`.
- Let's see another example with Mutable and Immutable references.
```rust
fn mutable_and_immutable_references() {
    let mut vec_1 = vec![6, 5, 8, 9];
    let ref_1 = &vec_1;
    println!("ref_1: {:?}", ref_1);
    let ref_2 = &mut vec_1;
    ref_2.push(3);
    println!("ref_2: {:?}", ref_2);
}
```
- In this example we have a vector defined, and in the next line we have an immutable reference `ref_1` to the vector `vec_1`, then print the value of `ref_1`.
- In the next line we have a mutable reference `ref_2` to the vector `vec_1`, then push a value to the vector using `ref_2`, and then print the value of `ref_2`.
- **note:** We can have multiple immutable references to a value, but only one mutable reference to a value at a time.
- In the above example, we have an immutable reference `ref_1` and a mutable reference `ref_2` to the vector `vec_1`.
- This is allowed because the lifetime of immutable reference `ref_1` starts at line 2 `let ref_1 = &vec_1;` and ends at the ends at the next line, `println!("ref_1: {:?}", ref_1);`.
- Similarly, the lifetime of mutable reference `ref_2` starts at line 4 `let ref_2 = &mut vec_1;` and ends at the end of the block.
- Lifetimes of `ref_1` and `ref_2` don't overlap, so there is no conflict.
- This is because rust uses another concept `Non-Lexical Lifetimes`.
- `Non-Lexical Lifetimes` aims to relax some of the strictness imposed by the typical lifetimes.
- This is achieved by analyzing the actual usage of references in the code, rather than solely relying on scopes.
- In simple terms, **`Non-Lexical Lifetimes` are lifetimes that are not tied to a scope.**
- By using this concept, compiler analyzed the code, and finds that `Non-Lexical Lifetimes` of `ref_1` and `ref_2` don't overlap, so it allows the code to compile.
- If we modify the code as follows
```rust
fn mutable_and_immutable_references() {
    let mut vec_1 = vec![6, 5, 8, 9];
    let ref_1 = &vec_1;
    let ref_2 = &mut vec_1;
    println!("ref_1: {:?}", ref_1);
    ref_2.push(3);
    println!("ref_2: {:?}", ref_2);
}
```
- If we exchange line 3 and 4, then the code will not compile.
- This is because the lifetime of `ref_1` and `ref_2` will overlap, even `Non-Lexical` ones, and this is not allowed in Rust.