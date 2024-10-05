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
    - `&Box<T>` can be coerced to `&T`
    - `&Vec<T>` can be coerced to `&[T]`
