# Function Inputs and Coercion
-------------------------------------------------------
- #### We will be looking at an effective way on passing borrowed values into a function.
- Let's start by defining a **function which will accept a reference to a string as an input and will return the number of vowels in the string.**
```rust
pub fn main() {
    let name = String::from("Shubhendu");
    println!("Voewls in the name: {} are {}", name, vowels(&name));

    let new_str = "Shubhendu";
    // println!("Voewls in the name: {} are {}", new_str, vowels(new_str)); // This doesn't compile because of type mismatch
}

fn vowels(words: &String) -> u8 {
    words
        .chars()
        .into_iter()
        .filter(|&c| (c == 'a') | (c == 'e') | (c == 'i') | (c == 'o') | (c == 'u'))
        .count() as u8
}
```
- The above code works fine but it is not very flexible. We can only pass a `String` type to the function. If we try to pass a `&str` type, it will not compile.
- If we change the function signature to accept a `&str` type, it will work fine for both `String` and `&str` types.
```rust
fn vowels(words: &str) -> u8 {
    words
        .chars()
        .into_iter()
        .filter(|&c| (c == 'a') | (c == 'e') | (c == 'i') | (c == 'o') | (c == 'u'))
        .count() as u8
}
```
- **The above code works fine because of Rust's coercion feature.** Rust can coerce a `&String` to `&str` but not the other way around.
- General Rule for efficiency is that, if you have an input that can be of multiple types, always use the most general type. In this case, `&str` is more general than `&String`. Always choose coercion target type as the input type. This will avoid unnecessary type conversions and indirections.
- **Coercion is a feature in Rust that allows the compiler to automatically convert a value from one type to another.** It is similar to type casting in other languages but it is done implicitly by the compiler. Rust has a set of rules that it follows to coerce a value from one type to another.

- Let's look at some more coercion cases,
    - `&String` can be coerced to `&str`
    - `&Box<T>` can be coerced to `&T`
    - `&Vec<T>` can be coerced to `&[T]`
-------------------------------------------------------
# Some Efficient Programming Tips
-------------------------------------------------------
## Simplifying Syntax of Nested Conditions Using Match
=======================================================
 - __*Suppose we have some application where we want to display a suitable message to the user based on his personal condition. If he has a cancer and smokes, then we would advise him that his cancer may be due to smoking. But if he has cancer but doesn't smoke then we will advise him that cancer is not related to smoking. There may be other possible reasons and he should consult his doctor.*__
 - One way we can implementation is this
```rust
pub fn main() {
    let cancer = true;
    let smoking = false;

    match cancer {
        true => match smoking {
            true => println!("Your cancer is likely caused by smoking."),
            false => println!("Your cancer is not caused by smoking. You should consult your doctor for further advice."),
        },
        false => match smoking {
            true => println!("Smoking is dangerous and may cause cancer. You should consult your doctor for further advice."),
            false => println!("You don't have any disease."),
        }
    }
}
```
- But this can be simplified using a tuple pattern in the match statement.
```rust
fn main() {
    let cancer = true;
    let smoking = true;
    match (cancer, smoking) {
        (true, true) => println!("Your cancer is likely caused by smoking."),
        (true, false) => println!("Your cancer is not caused by smoking. You should consult your doctor for further advice."),
        (false, true) => println!("Smoking is dangerous and may cause cancer. You should consult your doctor for further advice."),
        (false, false) => println!("You don't have any disease."),
    }
}
```
- **The above code is more readable and concise.**
- **The tuple pattern in the match statement is a very powerful feature in Rust. It allows us to match multiple values at once.**
- **_In Summary, when we are checking multiple variables in a nested fashion using the match, then we can reduce some of extra match statement and replace them with a tuple pattern._**

=======================================================
## Using `.collect()` to get First Error Variant
=======================================================
- *Suppose we have a client who is communicating with the server and the server responds with a valid message of some error message. We are interested in the first error message that the server sends.*
```rust
fn using_collect() {
    let responses = vec![Ok(100), Err("Client Error"), Ok(300), Err("Server Error")];
    let result = responses.into_iter().collect::<Result<Vec<_>, _>>();
    println!("{:?}", result);
}
```
- In the above code, we have a vector of responses from the server. We are interested in the first error message that the server sends. We can use the `.collect()` method to get the first error message, directly by defining the type of the result as `Result<Vec<_>, _>`. And `.collect()` will return the first error message, if an error occurs.

=======================================================
## Organizing Struct Using a HashMap with Key being One of the Field
=======================================================
- *Suppose we have a struct that has multiple fields and we want to organize the struct based on one of the fields. We can use a HashMap to organize the struct based on one of the fields.*
- This will allow us to access the struct based on the key field in O(1) time complexity.
```rust
fn organizing_hash_maps() {
    let person_1 = Person {
        name: "Joseph".to_string(),
        _age: 25,
    };

    let person_2 = Person {
        name: "Jane".to_string(),
        _age: 30,
    };

    let person_3 = Person {
        name: "Michael".to_string(),
        _age: 35,
    };

    let persons = vec![person_1, person_2, person_3];

    let person_hash = persons_by_name(persons);
    println!("{:?}", person_hash);

    for (name, details) in &person_hash {
        println!("Person {:?} has details {:?}", name, details);
    }
}

fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person> {
    persons
        .into_iter()
        .map(|person| (person.name.clone(), person))
        .collect()
}
```
- We have used a `HashMap` to organize the `Person` struct based on the `name` field. We have defined a function `persons_by_name` that takes a vector of `Person` structs and returns a `HashMap` with the `name` field as the key and the `Person` struct as the value.
- We have used `.clone()` method to clone the `name` because we don't want key to be a reference and we don't want partial move of the value from the struct.

=======================================================
## Using `reverse()` to reverse a range.
=======================================================
- *Suppose we have a range of numbers and we want to reverse the range. We can use the `reverse()` method to reverse the range.*
- We will not be directly use `10..0` because it will not work. We will use `(0..10).rev()`
```rust
fn using_reverse() {
    println!();
    println!("When using normal range operator '0..10'");
    for i in 0..10 {
        println!("{}", i);
    }

    println!();
    println!("When using reverse range operator '10..0'");
    for i in 10..0 {
        println!("{}", i);
    }
    println!("The output is empty because the range operator '10..0' is not valid.");

    println!();
    println!("For reverse range, use '0..10.rev()' instead");
    for i in (0..10).rev() {
        println!("{}", i);
    }
}
```
-------------------------------------------------------
# Todo Macro and Some Useful Extensions
-------------------------------------------------------
## Todo Macro
=======================================================
- Some times you may want to write the bigger blocks of code first that will make up your project to help you imagine your project.
- You can't write code in all the blocks you have defined and therefore you may wish to add code to each block in sequential order, one after the other. This will mean that initially some block has to be left empty.
- You can use the `todo!()` macro to mark the blocks that you have not yet implemented.
- The `todo!()` macro allows us to create empty blocks of code segments which you may later on complete. This way you can have an complete view of your project and keep track of the progress.
- Another advantage of using `todo!()` macro is that it will panic if you are calling the function which has `todo!()` macro in it. This will help you to keep track of the progress of your project.
```rust

#[derive(Default)]
struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
    salary: u32,
    nationality: String,
}

impl Student {
    fn some_fn_1(&self) -> String {
        todo!()
    }

    fn some_fn_2(&self) -> String {
        todo!()
    }
}


fn todo_macro() {
    let student = Student::default();
    student.some_fn_1();
}
```
- We can also choose to return a `"".to_string()` from `some_fn_1` and `some_fn_2` functions. However this has some drawbacks.
    1. Firstly, it's more code to write, specially when return value is complicated and return more items.
    2. Secondly, it is bit harder to understand that it is a dummy value. And at runtime if we call this function, it will return a value which may not be expected.
    3. Thirdly, dummy values may have different forms, which may or may not be valid. But `todo!()` macro will always panic if called, and has consistent form and behavior.
- **In Summary, `todo!()` macro is a very useful tool to keep track of the progress of your project. It allows you to create empty blocks of code segments which you may later on complete.**
- We can use a todo extension in VSCode to keep track of the `todo!()` macro in the code. This will help you to keep track of the progress of your project.
- We can also use `Todo Tree` extension in VSCode to keep track of the `todo!()` macro in the code. This will help you to keep track of the progress of your project.
- Both will require some customization to work with `todo!()` macro. Adding regex of `todo!()` macro in the settings of the extension will help to keep track of the `todo!()` macro in the code.
-------------------------------------------------------
# Performance Lints
-------------------------------------------------------
## Performance Improvements using Clippy Provided Lints
=======================================================
- **Lints** are warnings or suggestions provided by the Rust compiler to help you write more idiomatic and efficient code. Lints can catch common mistakes and code patterns that may lead to bugs or suboptimal performance.
- There are many lints provided by the Rust compiler and the Clippy tool that can help you write better code. Some of these lints are related to performance improvements.
- You can also create custom lints using the `#[warn(clippy::...)]` attribute. This will allow you to catch common mistakes and code patterns that may lead to bugs or suboptimal performance.
- We can see available lints at [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/index.html)
- There are many lints available. Each lints is associated with a level of severity. The levels are:
    - `allow`: Allow the lint. Does not do anything by default.
    - `warn`: Warn about the lint. Produces and Warning.
    - `deny`: Deny the lint. Throws an error if the lint is triggered.
    - `None`: Disable the lint. Does not do anything.
- Lints are also divided into categories. Some of the categories are:
    - `correctness`: Lints that catch common mistakes and code patterns that may lead to bugs.
    - `style`: Lints that enforce a particular style or convention.
    - `complexity`: Lints that catch code that may be too complex or hard to understand.
    - `perf`: Lints that catch code that may be suboptimal in terms of performance.

```rust
struct A {
    values: Box<Vec<i32>>,
}

pub fn main() {

}
```
- We will get a clippy warning for the above code. 
```bash
warning: you seem to be trying to use `Box<Vec<..>>`. Consider using just `Vec<..>`
 --> src\performance_lints.rs:5:13
  |
5 |     values: Box<Vec<i32>>,
  |             ^^^^^^^^^^^^^
  |
  = help: `Vec<..>` is already on the heap, `Box<Vec<..>>` makes an extra allocation
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#box_collection
  = note: `#[warn(clippy::box_collection)]` on by default
```
- **The above warning is suggesting that we should use `Vec<i32>` instead of `Box<Vec<i32>>`. This is because `Vec<i32>` is already on the heap and `Box<Vec<i32>>` makes an extra allocation.**

```rust
let x: Box<i32> = Box::new(Default::default());
```
- We will get a clippy warning for the above code.
```bash
  |
9 |     let x: Box<i32> = Box::new(Default::default());
  |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `Box::default()`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#box_default
  = note: `#[warn(clippy::box_default)]` on by default
```
- Because we can reduce the calls to function by using
```rust
let x: Box<i32> = Box::default();
```
- **The above warning is suggesting that we should use `Box::default()` instead of `Box::new(Default::default())`. This is because `Box::default()` is more idiomatic and concise.**

```rust
let _y = String::from("Nouman");
    let _z = "Nouman";
    if _y == _z.to_owned() {
        println!("Equal");
    }
```
- We will get a clippy warning for the above code.
```bash
warning: this creates an owned instance just for comparison
  --> src\performance_lints.rs:15:14
   |
15 |     if _y == _z.to_owned() {
   |              ^^^^^^^^^^^^^ help: try: `*_z`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#cmp_owned
   = note: `#[warn(clippy::cmp_owned)]` on by default
```
- **The above warning is suggesting that we should use `*_z` instead of `_z.to_owned()`. This is because `_z` is already a reference and we don't need to create an owned instance just for comparison.**

```rust
let _y = String::from("Nouman");
    let _z = "Nouman";
    if _y == _z {
        println!("Equal");
    }
```
```rust
let mut a = vec![1, 2, 3];
let mut b = vec![4, 5, 6];
a.extend(b.drain(..));
```
- We are trying to extend the vector `a` with the elements of vector `b` by draining the elements of vector `b`. This is a common pattern in Rust to move the elements of one vector to another vector.
- We will get a clippy warning for the above code.
```bash
warning: use of `extend` instead of `append` for adding the full range of a second vector
  --> src\performance_lints.rs:26:5
   |
26 |     a.extend(b.drain(..));
   |     ^^^^^^^^^^^^^^^^^^^^^ help: try: `a.append(&mut b)`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#extend_with_drain
   = note: `#[warn(clippy::extend_with_drain)]` on by default
```
- **The above warning is suggesting that we should use `a.append(&mut b)` instead of `a.extend(b.drain(..))`. This is because `append` is more idiomatic and concise.**

```rust
let mut a = vec![1, 2, 3];
let mut b = vec![4, 5, 6];
a.append(&mut b);
```
- `.append()` is better if we want to move the elements of one vector to another vector. It is more idiomatic and concise.

```rust