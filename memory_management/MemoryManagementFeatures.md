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
========================================================
### Lifetime Elision
========================================================
- Consider the following program
```rust
pub fn main() {
    let str_1 = "some str";
    let received_str = return_str(&str_1);
}

fn return_str(s_1: &str) -> &str {
    s_1
}
```
- In the above code, we have a function `return_str` that takes a reference to a string and returns a reference.
- Even though we are taking in a reference and returning a reference, compiler doesn't complain about missing lifetime specifiers.
- This is because of `Lifetime Elision`.
- `Lifetime Elision` is a feature in Rust, that allows the compiler to infer the lifetimes of references in function and method signatures, making it more readable.
- Rust compiler follows three lifetime elision rules to determine the lifetimes of references.
    1. Each parameter that is a reference, gets its own lifetime parameter.
    2. If there is exactly one input lifetime parameter, then that lifetime is assigned to all output lifetime parameters.
    3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, then the lifetime of `self` is assigned to all output lifetime parameters.
- After applying the elision rules, if the lifetime of the reference can't be inferred, then the compiler will throw an error.
- Let's look at the above code, keeping in mind elision rules, rust compiler will treat the above code as
```rust
fn return_str<'a>(s_1: &'a str) -> &'a str {
    s_1
}
```
- So, the lifetime of output of `return_str` function is the same as the lifetime of input parameter `s_1`.
- Let's look at another example
```rust
fn return_str(s_1: &str, s_2: &str) -> &str {
    if s_1.len() > s_2.len() {
        s_1
    } else {
        s_2
    }
}
```
- In the above code, we have two input parameters `s_1` and `s_2`, and we are returning a reference to one of them.
- After applying elision rules, each parameter will have its own lifetime parameter, and since there are multiple input lifetime parameters, the compiler will throw an error.
```rust
fn return_str<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &str {
    if s_1.len() > s_2.len() {
        s_1
    } else {
        s_2
    }
}
```
- In this case we have two input lifetime parameters, so we need to specify the lifetime of the output reference.
- We can specify the lifetime of the output reference as `'a` or `'b`, because the output reference will have the same lifetime as either `s_1` or `s_2`.
```rust
fn return_str_2<'a, 'b: 'a>(s_1: &'a str, s_2: &'b str) -> &'a str {
    if s_1.len() > s_2.len() {
        s_1
    } else {
        s_2
    }
}
```
- In the above code, we have specified that the lifetime of `s_2` is at least as long as the lifetime of `s_1`.
- `'b: 'a` means that the lifetime of `s_2` is at least as long as the lifetime of `s_1`.

========================================================
### Lifetime Annotations in Structs
========================================================
- So far we have only seen struct with owned fields, but what if we have a struct with references.
- Let's look at an example
```rust
struct ArrayProcessor {
    data: &[i32],
}
```
- It is quite possible that the reference becomes invalid during the execution of the program, while the instance of the struct is still alive.
- In such cases, struct field will become a dangling reference.
- To prevent this, in Rust, all the struct field which contain reference to a value, must contain a generic lifetime annotation/specifier.
- Let's modify the above code with lifetime specifier.
```rust
struct ArrayProcessor<'a> {
    data: &'a [i32],
}
```
- Note that, unlike functions, lifetime elision are not defined for structs.
- It is quite obvious that the reference to which field is pointing to, should have a lifetime at least as long as that of the struct. We expect compiler to infer this automatically, but this is not the case.
- It's more of a language design issue.
- The reason why functions and methods have lifetime elision rules and struct do not, maybe because we tend to use references far more often in functions and methods than in structs.
- Let's create an implementation block for `ArrayProcessor` struct.
- Just like with generics, we must include generic lifetime annotation inside our impl block.
```rust
impl<'a> ArrayProcessor<'a> {
    fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
        let previous_data = self.data;
        self.data = new_data;
        &previous_data
    }
}
```
- In the above code, we have an implementation block for `ArrayProcessor` struct, and we have a method `update_data` that takes a reference to a slice of `i32` values and returns a reference to the previous data.
- We have included a generic lifetime annotation `'a` inside the impl block, because the method `update_data` takes a reference to a slice of `i32` values, and returns a reference to the previous data.
- With lifetime elision rules, the above code will be treated as
```rust
impl<'a> ArrayProcessor<'a> {
    fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'a [i32] {
        let previous_data = self.data;
        self.data = new_data;
        &previous_data
    }
}
```

--------------------------------------------------------
## Smart Pointers
--------------------------------------------------------
- There are many different types of smart pointers in rust, serving different purposes.
- Key differences between a pointer and a smart pointer
- #### Simple Pointer
    - Simple pointer variable stores the memory address of some variable/value. We have been using such pointers throughout the course.
    - Usually indicated by `&` symbol.
    - They are also called `references`.
    - Other than referring to or pointing to some value, these references don't have any additional functionality.
- #### Smart Pointer
    - They are not just simple references, but they also have additional functionality.
    - They also include some metadata along with the memory address.

========================================================
### Box Smart Pointers
========================================================
- By default Rust allocates everything on the stack.

```rust
fn main() {
    let x: f64 = 5.0;
}
```
- In the above code, `x` is allocated on the stack.
- But what if we want to allocate `x` on the heap, then we can use `Box` smart pointer.
```rust
fn main() {
    let x: f64 = 5.0;
    let y: Box<f64> = Box::new(x);
}
```
- In the above code, `y` is allocated on the heap.
- Variable `y` is a `Box` smart pointer, and it contains the memory address of the value `5.0` which is allocated on the heap.
- Variable x will remain on the stack in this case.
```rust
fn main() {
    let x: f64 = 5.0;
    let y: Box<f64> = Box::new(x);
    let z = &x;
}
```
- In the above code, `z` is a reference to the value `5.0` which is allocated on the stack, not on heap.
- `Box` smart pointers are similar to unique pointers in C++.

- #### Use cases of box pointer
```rust
#[derive(Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
}
```
- The idea is that a list which may contain other lists.
- But above code will throw an error, `recursive type 'List' has infinite size`.
- The problem with recursive types is that the rust compiler is unable to know the exact size of the instance of such type at compile time.
- Rust needs to know the size of the instance at compile time, so that it can allocate memory for it.
- But the size of the instance of `List` is not known at compile time, because it may contain other instances of `List`.
- Let's look at a simple enum.
```rust
enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
    Walk,
}
```
- In the above code, to determine the size of the instance of `Conveyance`, rust compiler needs to know the size of the largest variant by size.
- Compiler will go through each of the variants and determine the size of the largest variant.
- In this case, the largest space will be size of `i32`.
- But in case of `List`, the size of the instance is not known at compile time, because it may contain other instances of `List`, and we don't know how much space `List` will take.
- To solve this problem, we can use `Cons` behind some kind of pointer.
```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

```
- In the above code, we have wrapped the `List` enum inside a `Box` smart pointer.
- So now the size of the instance of `List` is known at compile time, because the size of the instance of `Box` is known at compile time. So rust compiler can allocate memory for it.
- Above code works, but it can be improved.
- If we look at the enum, we can see that in case of `Cons`, we are always making a new heap allocation by calling the new function, irrespective of whether the next variant will be a `Cons` or `Nil`.
- `Nil` variant doesn't need a heap allocation, because it doesn't contain any data, it basically terminates the recursion, representing end of the list.
- In the last variant, `Nil` is unnecessary being wrapped in a `Box`, i.e. assigned to a heap allocation.
- This can we improved by using `Option` enum.
```rust
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
    Nil
}

let list = List::Cons(
    1,
    Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None))))))
);

println!("list: {:?}", list);
```
- Using above code, when we have a further cons variance, we will have `Some` boxed list, and when we want to terminate, we wil use `None` variant.
- This way we can avoid unnecessary heap allocations, and we don't need to have a not needed second variant `Nil`.
- `Nil` can be indicated by `None` variant of `Cons`.
- In the above case, list construction is more precise and efficient.
- #### Let's look at couple of more use cases, where `Box` may be beneficial.
- `Box` is also useful when copying large amounts of data, when transferring ownership.

```rust
let data_1 = HugeData;
let data_2 = Box::new(HugeData);

let data_3 = data_1;
let data_4 = data_2;
```
- In the above code, `HugeData` is a large data structure.
- In the first case, `data_1` is allocated on the stack, and `data_2` is allocated on the heap.
- Then we are transferring ownership of `data_1` to `data_3`, and `data_2` to `data_4`.
- In the first case, `data_1` is copied to `data_3`, which is a costly operation, because `HugeData` is a large data structure, and it is residing on the stack.
- In the second case, `data_2` is copied to `data_4`, but since `data_2` is a `Box` smart pointer, it is just a memory address, which is a cheap operation.
- In this exact case, `HugeData` doesn't have anything to be copied, but suppose we have large data structure, then copying it will be a costly operation.
- #### Another use case of `Box` Smart Pointer
- Consider that we would like to create a vector of different types that implement some trait.

```rust

struct SmallData {}
struct HugeData {}

trait Storage{}

impl Storage for HugeData {}
impl Storage for SmallData {}
```
- Now suppose we want to define a vector of types that implement the `Storage` trait
```rust
let data_5 = Box::new(SmallData);
let data = vec![Box::new(data_3), data_4, data_5];
```
- `let data ...` line will throw an error, `mismatched type, expected Box<HugeData>, found Box<SmallData>`.
- The problem is that the vector `data` is expecting all the elements to be of the same type, but in this case we have different types.
- Remember that vector can only store value that have the same type.
- To enable the vector to store different types, that have same trait, we can tell the compiler to store any type in the vector that implements the `Storage` trait.
- This can be mentioned using `trait objects`.

========================================================
### RC (Reference Counting) Smart Pointer
========================================================
- According to Rust ownership rule, there must be exactly one owner of a value.
- There are however situations which demand to have multiple owners of a value.
- Suppose there is a `list A` which is pointed by `list B` and `list C`.
- `list A` should remain in memory as long as it is being pointed by either `list B` or `list C`.
- If we delete `list A` then the pointers in `list B` and `list C` which points to `list A` will become dangling pointers/invalid.
```rust
enum List {
    Cons(i32, Option<Box<List>>),
}

fn main() {
    let a = List::Cons(1, Some(Box::new(List::Cons(2, None))));
    let b = List::Cons(3, Some(Box::new(a)));
    let c = List::Cons(4, Some(Box::new(a)));
}
```
- For the above code, compiler will throw an error at `let c = ...` line, `use of moved value: 'a'`.
- This is because `a` is moved to `b`, and we are trying to use it again in `c`.
- We can't use a reference to a, because we are already creating a Box around it, and the inside, as per definition of struct, should be a value, not a reference.
- Here comes the `RC` smart pointer.
```rust
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Option<Rc<List>>),
}

pub fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    let b = List::Cons(3, Some(Rc::clone(&a)));
    let c = List::Cons(4, Some(Rc::clone(&a)));
}
```
- In the above code, we have used `Rc` smart pointer.
- To enable `List b` to point at `List a` `Rc` provides a clone function, which takes in a reference to the `Rc` smart pointer, and returns a new `Rc` smart pointer pointing to the same value.
- What the `Rc` did is that it now treats `a`, `b` and `c` as owners of the value. In other words, they share ownership of the value.
- `Rc` smart pointer is equivalent to `shared_ptr` in C++.
- Internally, `Rc::clone` doesn't make a deep copy, which is unlike default implementation of `clone` in Rust.
- `Rc::clone` just increments the reference count of the value, and returns a new `Rc` smart pointer pointing to the same value. Therefor it is computationally efficient/cheap.
- When we create `a` via the line `let a = Rc::new(...)`, the reference count of the list is set to 1.
- When we create `b` and `c` via the line `let b = ...` and `let c = ...`, the reference count of the list is incremented to 3.
- At the end of `main` function, all the variables will be dropped, in `LIFO` order.
- First `c` will be dropped, then `b`, and then `a`.
- As `c` is dropped, reference count of the list will be decremented to 2.
- As `b` is dropped, reference count of the list will be decremented to 1.
- As `a` is dropped, reference count of the list will be decremented to 0 and the memory is deallocated.
- #### Reference counter smart pointer has many use cases, like graph, trees, linked list, etc.
- for example, in graph data structure, we have multiple edges, which may point to the same node.
    - Conceptually, in a graph, a node is owned by all the edges that point to it.
    - A node shouldn't be cleaned up, unless it doesn't have any edges pointing to it and so has no owners.
- NOTE: As a side note, we have already been using a couple of smart pointers that are `String` and `Vec`. Having the special information of capacity they occupy in memory, and both of them also owns the data they are referring to in the memory.
========================================================
### RefCell Smart Pointer
========================================================
- Like `Box` pointer, it provides single ownership of data.
- An interesting property of `RefCell` is that it enforces borrowing rules to be checked at runtime, not at compile time.
- It means that if you violate the borrowing rules at runtime, then the program will panic and exit.
```rust
pub fn main() {
    let mut x = 50;
    let x1 = &x;
    let x2 = &x;
    let x3 = &mut x;

    println!("x1: {}, x2: {}, x3: {}", x1, x2, x3);
}
```
- This code will not compile, because we are trying to borrow `x` as mutable and immutable at the same time. It is a violation of borrowing rules.
```rust
use std::cell::RefCell;

fn main() {
    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    let d = a.borrow_mut();

    println!("b: {}, c: {}", b, c);
}

```
- In the above code, we have used `RefCell` smart pointer.
- `RefCell` provides two methods `borrow` and `borrow_mut` to borrow the value.
- In the above code, we have borrowed the value `a` as immutable twice, and then as mutable.
- This code will pass compiler checks and will panic at runtime.
- The advantage of checking borrowing rules at compile times are:
    - Errors will be caught sooner in the development cycle.
    - There is no runtime overhead.
    - So checking borrowing rules at compile time is default behavior in Rust.
- Advantage of checking borrowing rules at runtime are:
    - Certain memory safe scenarios are allowed, which are not possible with compile time checks.
    - Because certain properties of the code is impossible to detect using static analysis.
- The references to `RefCell` do not follow the non-lexical lifetimes, this means they are tied to the scope in which they are defined.
- In above code, `b`, `c` and `d` are tied to the scope in which they are defined, remain in memory till the end of the scope.
- We can call `drop` method on `RefCell` to explicitly drop the value.
```rust
use std::cell::RefCell;

fn main() {
    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    let d = a.borrow_mut();

    // println!("b: {}, c: {}", b, c);

    drop(a);
    drop(b);
}
```
- Above code will compile and run without any issues.
- An alternative to drop will be to define variable `b` and `c` in a separate scope.
```rust
use std::cell::RefCell;

fn main() {
    let a = RefCell::new(10);
    {
        let b = a.borrow();
        let c = a.borrow();
    }
    let d = a.borrow_mut();

    println!("a: {}", a);
}
```
- In the above code, `b` and `c` are defined in a separate scope, so they will be dropped at the end of the scope.
- Printing in above case will output `RefCell { value: <borrowed> }`. This is because when this print was called, `d` was borrowing the value, which can be changed.
- If we drop `d` before the print statement, then the output will be `RefCell { value: 10 }`.
- The checking of borrowing rules at runtime allows something called `Interior Mutability`.
```rust
// Rust doesn't allow the following
let x = 5;
let y = &mut x;
```
- Using `RefCell` smart pointer, we can mutate the value, even if the value itself is immutable.
- Data is mutable on the inside, but we don't expose that mutability to the outside. So the outside world doesn't know that data can be mutated.

```rust
use std::cell::RefCell;

fn main () {
    let a = RefCell::new(10);
    let mut b = a.borrow_mut();
    *b = 15;
}
```
- In the above code, we have borrowed the value `a` as mutable, and then changed the value.
- `RefCell` doesn't implement a `Deref` trait, so we need to use `*` to dereference a borrowed value like `b` not `a` itself.
- `RefCell` smart pointer is not that powerful by itself, but when combined by `RC smart pointer` some amazing things can happen?
- #### Combining `RC` and `RefCell` smart pointers
```rust
use std::cell::RefCell;
use std::rc::Rc;

let a = Rc::new(RefCell::new(String::from("C++")));
```
- The `Rc` provides multiple immutable ownership, but `RefCell` providing mutable access to internal data. Multiple owners are given the ability to mutate the internal data.
- This is a powerful combination, and can be used in many scenarios.
- For instance, let's create owner of data,
```rust
let b = Rc::clone(&a);

*b.borrow_mut() = String::from("Rust");
println!("a: {:?}", a);
```
- `Box`, `Rc` and `RefCell` are the fundamental types of smart pointers, that covers majority of our use cases.
