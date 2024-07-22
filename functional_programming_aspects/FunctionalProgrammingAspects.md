# Functional Programming Aspects
-------------------------------------------------------
## Closures
-------------------------------------------------------
- Consider some business which stores details regarding its user in a struct.
```rust
struct User {
    name: String,
    age: u8,
    salary: u32,
}
```
- The business is interested in some function which can be used to validate the users. Let's say if the `name` field is not empty then the user is valid.
```rust
fn validate_user(name: &str) -> bool {
    name.len() != 0
}


pub fn main() {
    let person_1 = User {
        name: String::from("Some One"),
        age: 35,
        salary: 100_000,
    };

    println!("User Validity: {}", validate_user(&person_1.name));

}
```
- Above code is perfectly fine, however there is an alternative to this.
- Instead of creating a function to validate users, we can store the validation logic in a `closure`.
- `closures` are anonymous variables which we can store in a variable or can pass as arguments to other functions.
- For instance let's add a variable `validate_user` which stores the closure.
```rust
let validate_user: impl Fn(&str) -> bool = |name: &str| name.len() != 0;
```
- If we look at the type of variable it is `impl Fn (&str) -> bool`.
- In rust each `closure` has a concrete type, `impl Fn()` followed by the signature of the closure.
- This closure has a type which implements the `Fn` trait which takes a `&str` and returns a `bool`.
- `Fn` trait is a special syntax in rust which is used to define the signature of the closure.
- Beside `Fn` trait there are two other traits `FnMut` and `FnOnce` which are used to define the signature of the closure.
- In certain cases, rust compiler will infer the argument and return types of the closure.
- This is unlike functions, where type of the input and return type must always be specified.
- Curly Braces around the body of closure is optional if the closure has a single expression.
- Closures can be called in the same way as functions.
- Writing variable name, storing the closure and passing the argument to the closure.
```rust
println!("User Validity: {}", validate_user(&person_1.name));
// User Validity: true
```
- One advantage of closure is that it can be passed around to functions, which is possible with the help of generics and trait bounds.
- Let's define another closure which checks the age for validity.
```rust
let validate_user_advance: impl Fn(u8) -> bool = |age: u8| age > 30;
```
- Now let's define a function, which will take `validate_user_advance` and `validate_user` as inputs and checks both of them for validity.
```rust
fn is_valid_user<V1, V2>(validate_user: V1, validate_user_advance: V2, name: &str, age: u8) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    validate_user(name) && validate_user_advance(age)
}
```
- This function takes two closures `validate_user` and `validate_user_advance` as input and checks both of them for validity.
- We can specify the type of the closure as `V1` and `V2` and then use `Fn` trait to define the signature of the closure.

- **One thing about closures is that they can capture variables from the scope, in which they are defined.**
- Variables may be captured 
    - By mutable borrow `&mut` -> `FnMut`
    - By immutable borrow `&` -> `Fn`
    - By transfer of ownership -> `FnOnce`
- When the environment variables are captured by `immutable` borrow, the closure is implementing `Fn` trait.
- When the environment variables are captured by `mutable` borrow, the closure is implementing `FnMut` trait.
- When the environment variables are captured by `transfer of ownership`, the closure is implementing `FnOnce` trait.
- For example, let's create a closure `banned user`.
```rust
let banned_user_name = String::from("Banned User");
let validate_user: impl FnOnce(&str) -> bool = |name: &str| {
    let banned_user = banned_user_name == name; // Transferring ownership of banned_user_name, hence FnOnce
    name.len() != 0 && name != banned_user
};
```
- Above closure is capturing `banned_user_name` by `transfer of ownership`, hence it is implementing `FnOnce` trait, instead of `Fn` trait.
- It is also capturing `banned_user_name` from its scope.
- If we use mutable borrow, then the closure will implement `FnMut` trait.
```rust
let mut banned_user_name = String::from("Banned User");
let validate_user: impl FnMut(&str) -> bool = |name: &str| {
    let banned_user = &mut banned_user_name;// Mutable borrow, hence FnMut
    name.len() != 0 && name != banner_user
};
```
- Trait is implemented for the closure based on the way the environment variables are captured.
- It is worth noting that every closure implements `FnOnce` trait, even if it is not capturing any environment variables, because every closure can be called at least once.
- If the closure is capturing multiple environment variable, to enforce ownership of all the variables, we can use `move` keyword.
- `move` converts any variable captured by `mutable` or `immutable` borrow to `transfer of ownership`.
```rust
let banned_user = String::from("Banned User");
let validate_user_move: impl FnOnce(&str) -> bool = move |name: &str| {
    let banned_user_new = &banned_user;
    name.len() != 0 && name != banned_user_new
};

println!("{banned_user}"); // Will throw an error
```
- Last print statement will throw an error, because `banned_user` has been moved to the closure, because of `move` keyword, even thought it is being used by reference in actual code.

-------------------------------------------------------
## Functional Pointers
-------------------------------------------------------
- **Functional Pointers are similar to closures, except they don't capture variables from the scope.**
```rust
struct User {
    name: String,
    age: u8,
    _salary: u32,
}

fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: FnOnce(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}

fn validate_user_simple(name: &str) -> bool {
    name.len() != 0
}

pub fn main() {
    let person_1 = User {
        name: String::from("Some One"),
        age: 35,
        _salary: 100_000,
    };

    // let validate_user_simple = |name: &str| name.len() != 0;
    let validate_user_advanced = |age: u8| age >= 30;

    println!(
        "User Validity: {}",
        is_valid_user(
            &person_1.name,
            person_1.age,
            validate_user_simple,
            validate_user_advanced
        )
    )
}
```
- Even if we change the code as above, i.e. instead of using `closures` we are using `functional pointers`, the code will still work.
- In the above, we are passing `validate_user_simple` to the `is_valid_user` function. But `validate_user_simple` is not a closure but a function, which is being passed as a `functional pointer`.
- `functional pointers` implement all the three closure traits `Fn`, `FnMut` and `FnOnce`.
- So you can pass regular functions anywhere closures are expected.
- Here `V1` is `Functional Pointer` which is pointing to `validate_user_simple` function.
- We can do the same with `validate_user_advanced` function.
- We can also change the `is_valid_user` to only accept `Functional Pointers` instead of `closures`.
```rust
fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}
```
- Above was the example of function taking `closures` as arguments, now let's modify it to take `functional pointers` as arguments.
- `Functional Pointers` are concrete types and denoted by `fn` instead of `Fn` which is for closures.
- A key requirement for `Functional Pointers` is that it must not use any variables from its scope/environment.
- To use variables from scope/environment, we can explicitly pass them as arguments to the function.
-------------------------------------------------------
## Iterators
-------------------------------------------------------
