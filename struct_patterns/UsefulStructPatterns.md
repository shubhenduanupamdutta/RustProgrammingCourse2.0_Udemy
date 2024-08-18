# Useful Struct Patterns
---------------------------------------
## Initializing Struct Instance
==================================================
- In programming a new instance of a struct is typically crated using a constructor.
- Rust doesn't have built in constructors.
- There is only one way to create a new instance of a struct in Rust, that is by specifying the struct name and then the initializing all of its field.
```rust
// lib.rs
#[derive(Debug)]
pub struct Student {
    pub age: u8,
    pub name: String,
}

// main.rs
use struct_patterns::Student;
fn main() {
    let student = Student {
        age: 20,
        name: String::from("John"),
    };
    println!("{:?}", student);
}
```
- This is the only way to create a new instance of a struct in Rust.
- But what is there is a private field in the struct and we want to initialize it with some value.
- We can't do that because we can't access the private field outside the module.
```rust
// lib.rs
#[derive(Debug)]
pub struct Student {
    id: u8,
    pub age: u8,
    name: String,
}
```
- Now we can't initialize the id field from `main.rs`, because its private.
- And we don't want to make it public, because we don't want anyone to change/set the id, instead it should be set automatically.
- To fix this, we can follow the rest convention of creating an associated function for initializing the struct instances, usually called `new`. This function will work like a constructor.
```rust
impl Student {
    pub fn new(name: String) -> Self {
        Self {
            id: 1,
            age: 20,
            name: name,
        }
    }
}
```
- Now we can create a new instance of the struct using this associated function.
```rust
fn main() {
    let student = Student::new(String::from("John"));
    println!("{:?}", student);
}
```
- An advantage of using `new` method is that we can check in advance for some preconditions before creating the instance.
- For instance, I would like to check if the name passed in contains all characters.
```rust
impl Student {
    pub fn new(name: String) -> Result<Self, String> {
        if name
            .chars()
            .all(|x| matches!(x, 'a'..='z' | 'A'..='Z' | ' '))
        {
            Ok(Self {
                id: 1,
                age: 20,
                name,
            })
        } else {
            Err("Name should only contain alphabets and spaces".to_string())
        }
    }
}
```
- Now we can check if the name is valid before creating the instance.
========================================================
### Default Constructors
========================================================
- The rust std library provides a `Default` trait that can be implemented for a struct to provide a default constructor. This returns the struct with default values of the fields.
```rust
impl Default for Student {
    fn default() -> Self {
        Self {
            id: 0,
            age: 20,
            name: String::from("Unknown"),
        }
    }
}
```
- Now when we call `new` with faulty arguments like `Shubhendu123`, and call `unwrap_or_default` on the result, it will return the default instance of the struct.
```rust
fn main() {
    let student = Student::new(String::from("Shubhendu123")).unwrap_or_default();
    println!("{:?}", student);
}
```
- This will print the default instance of the struct.
```bash
Student 2: Student {id: 0, age: 20, name: "Unknown"}
```
- `Default` trait can be automatically implemented via `#[derive(Default)]` macro.
```rust
#[derive(Default, Debug)]
pub struct Student {
    id: u8,
    pub age: u8,
    pub name: String,
}
```
- This macro will automatically implement the `Default` trait for the struct.
- Default value for `id` will be `0`, for `age` will be `0` and for `name` will be `""`.
- Because `String` defaults to `""`, `integer` defaults to `0` and `bool` defaults to `false`.
- For custom types, we need to implement the `Default` trait manually.
- We can manually set some of the fields, and other as default values.