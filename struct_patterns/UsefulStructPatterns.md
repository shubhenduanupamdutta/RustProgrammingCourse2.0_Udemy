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

========================================================
## Builder Pattern
========================================================
- Some data structures are complicated to construct because their construction requires a large number of inputs and optional configuration choices.
- This can easily lead to a large number of distinct constructors, each having many arguments.
- The builder pattern is used to make this manageable.
- Consider this `Customer` struct containing several fields.
```rust
#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: MembershipType,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Default, Clone)]
enum MembershipType {
    #[default]
    New,
    Casual,
    Loyal,
}
```
- `membership` field is an enum `MembershipType` which can be one of the three values. For the sake of illustration assume that it represents some type which may become complicated due to future requirements/modifications.
```rust
impl Customer {
    fn new(name: String) -> Self {
        Customer{
            name: name,
            ..Default::default()
        }
    }
}
```
- `new` constructor will create a basic customer, whose only name is known, everything else is set to default.
- Let us consider slightly advanced level of customer who is interested to open a login account with use also.
- To create an instance of such a customer, we require to have another constructor.
- Rust doesn't allow overloading of methods, therefor we will define another method `new_2`.
```rust
impl Customer {

    fn new(name: String) -> Self {
        Customer{
            name: name,
            ..Default::default()
        }
    }

    fn new_2(name: String, username: String) -> Self {
        Customer{
            name: name,
            username: username,
            ..Default::default()
        }
    }
}
```
- Next we may have a customer who, in addition to having an account with use, also has membership with us.
- We will have to define another constructor for this.
```rust
fn new_3(name: String, username: String, membership: MembershipType) -> Self {
    Customer{
        name: name,
        username: username,
        membership: membership,
        ..Default::default()
    }
}
```
- Let's call all these constructor in main.
```rust
pub fn main() {
    let new_user = Customer::new("John Doe".to_string());
    println!("New user: {:?}", new_user);

    let user_with_login = Customer::new_2("John Doe".to_string(), "johndoe".to_string());
    println!("User with login: {:?}", user_with_login);

    let loyal_user = Customer::new_3("John Doe".to_string(), "johndoe".to_string(), MembershipType::Loyal);
    println!("User with login and membership: {:?}", loyal_user);
}
```
- This code will compile and print the following output.
```bash
New user: Customer { name: "John Doe", username: "", membership: New, gender: '\0', country: "", age: 0 }
User with login: Customer { name: "John Doe", username: "johndoe", membership: New, gender: '\0', country: "", age: 0 }
User with login and membership: Customer { name: "John Doe", username: "johndoe", membership: Loyal, gender: '\0', country: "", age: 0 }
```
- This is a simple example, but as the struct grows, the number of constructors will grow exponentially.
- There is a better way to do this.
- We can use builder pattern to stop proliferation of constructors.
- Define a `CustomerBuilder` struct that will have all the fields of `Customer` struct, but all of them will be optional except the mandatory ones, here `name`.
```rust
#[derive(Default, Debug, Clone)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<MembershipType>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}
```
- The next part of the builder pattern are individual methods for setting the values for each field.
- There will be as many methods as there are optional fields in `CustomerBuilder` struct.
```rust
impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }
}
```
- The method `username` is for `username` field of `CustomerBuilder` struct.
- It will take a mutable reference to self, `&mut self` and a `String` as input.
- The output will be a mutable reference to self, `&mut Self`.
- Inside the method, we set the `username` field of `CustomerBuilder` struct to `Some(username)`.
- And then return `self`.
- The return value is mutable reference to self, this is because these individual methods will be called one after the other, in a sequence where each method requiring mutation of data.
```rust
impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: MembershipType) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.clone().unwrap_or_default(),
            country: self.country.clone().unwrap_or_default().clone(),
            age: self.age.clone().unwrap_or_default(),
        }
    }
}
```
- After defining functions to set all the optional fields, we will create a build method.
- This method will return an instance of `Customer` struct.
- It will clone the values of all the fields of `CustomerBuilder` struct and return a new instance of `Customer` struct.
- We clone the values we don't want to move out of the `CustomerBuilder` struct.
- Last since only `name` is mandatory, rest of the fields may not be provided, so we set them to default values, using `unwrap_or_default`.
- Now we can edit our implementation for `Customer` struct.
```rust
impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name: name,
            ..Default::default()
        }
    }
}
```
- Now we can use this builder pattern by calling the methods as such,
```rust
fn main() {
    let new_user = Customer::new("John Doe".to_string())
        .build();

    println!("New user: {:?}", new_user);

    let user_with_login = Customer::new("John Doe".to_string())
        .username("johndoe".to_string())
        .build();
    println!("User with login: {:?}", user_with_login);

    let loyal_user = Customer::new("John Doe".to_string())
        .username("johndoe".to_string())
        .membership(MembershipType::Loyal)
        .build();
    println!("User with login and membership: {:?}", loyal_user);
}
```
- This will print the same output as before.
- `new` method of `Customer` struct will return an instance of `CustomerBuilder` struct, and then we can call the methods of `CustomerBuilder` struct to set the optional fields as needed.
- Finally we call the `build` method to get the instance of `Customer` struct.
- This way functions are chained together, and instances are created in a more readable way.
- We also have a nice single interface for constructing instances of `Customer` struct.
- This pattern is seen frequently in Rust code, because Rust doesn't have overloading of methods, and this is a good way to manage multiple constructors.
