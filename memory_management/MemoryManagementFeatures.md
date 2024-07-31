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

========================================================
### Generic Lifetimes
========================================================
```rust
pub fn main() {
    let int1 = 5;
    let int2 = 10;
    let picked_value = picking_int(&int1, &int2);
    println!("Picked value: {}", picked_value);
}

fn picking_int(i: &i32, j: &i32) -> i32 {
    if rand::random() {
        *i
    } else {
        *j
    }
}
```
- Consider the above program. In the program above we have `picking_int` function that takes two references to `i32` values and returns one of them based on a random number.
- Above program will compile and run without any issues.
- But what if we want to return a reference to one of the values instead of the value itself.
- If we modify the program as follows
```rust
fn picking_int(i: &i32, j: &i32) -> &i32 {
    if rand::random() {
        i
    } else {
        j
    }
}
```
- The above code will not compile, and will throw an error `missing lifetime specifier`.
- Let's think about it from borrow-checker's perspective.
- How borrow-checker (B-C) will determine if the `picked_value` is not a dangling reference, when we print it after the function call.
- To figure it out B-C will look at lifetime of the `picked_value`.
- In this code since we are returning a reference of either `i` or `j`, the `picked_value` will have a lifetime of either `i` or `j`. But B-C doesn't know which one, so that results in an error.
- `Lifetime Specifiers` are also known as `generic lifetime annotations`.
- It is a way to describe the relationship between lifetimes of references.
- They shouldn't be confused with `concrete lifetimes`, but are used to describe the relationship between `concrete lifetimes`.
- Let's modify the above code to include lifetime specifiers.
```rust
fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}
```
- In the above code, we have added a lifetime specifier `'a` to the function definition and the return type.
- So what does this tell about lifetime of `picked_value`?
- It tells that the lifetime of `picked_value` is the same as the shorter of the lifetimes of `i` and `j`.
- If we change the main function as follows
```rust
pub fn main() {
    let int1 = 5;
    {
        let int2 = 10;
        let picked_value = picking_int(&int1, &int2);
        println!("Picked value: {}", picked_value);
    }
}
```
- In the above code, the lifetime of `int2` is limited to the block in which it is defined.
- So lifetime of `picked_value` will be the same as the lifetime of `int2`, since it has shorter lifetime.
```rust
pub fn main() {
    let int1 = 5;
    let picked_value;
    {
        let int2 = 10;
        picked_value = picking_int(&int1, &int2);
        println!("Picked value: {}", picked_value);
    }
    println!("Picked value: {}", picked_value);
}
```
- Above code will not compile, because the lifetime of `picked_value` is limited to the block in which it is defined.
- Essentially last print statement can become a dangling reference if `picking_int` returns reference to `int2`. So rust doesn't allow this code to compile.
- There can be different types of relationship, based on what your function does.
- For example, function always returns reference to first parameter.
```rust
fn picking_int<'a>(i: &'a i32, j: &i32) -> &'a i32 {
    i
}
```
- It is important to note that typically lifetime of returned value should be linked to lifetime of input parameters.
- This is because when a function returns a reference, it should point to one of the inputs provided in the argument.
- If a function returns reference to a value, that is created inside the function, then the reference becomes invalid as soon as the function ends.
- If we really want to return a reference to a value created inside the function, then we can use `static` lifetime.
```rust
fn picking_int<'a>(i: &'a i32, j: &i32) -> &'static i32 {
    let y: &'static i32 = &5;
    y
}
```