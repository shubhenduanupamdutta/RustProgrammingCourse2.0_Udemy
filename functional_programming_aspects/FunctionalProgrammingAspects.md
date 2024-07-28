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
- **Iterators:** design pattern allow different types to have a common interface for accessing the elements of a type sequentially.
- This abstracts away the details of how `iterator` is implemented internally, and how the type is laid down and defined internally.
- *Iterators* are heavily used in Rust programs, and therefore a must to understand.
- Rust standard library has a trait `Iterator` which is implemented by many types, can be used to iterate over the elements of the type.
- We can also implement the `Iterator` trait for our custom types.
- Definition of `Iterator` trait in Rust standard library looks something like this:
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
- `Iterator` trait has an associated type `Item` which is the type of the element that the iterator will yield.
- `next` method is used to get the next element from the iterator, and it must be implemented for the type.
- `next` method provides a way to get to next element in the iteration. 
- Calling `next` method repeatedly will return `Some` with the next element, and `None` when the iteration is over.
- `Iterator` trait has many default implementation, but `next` method must be explicitly implemented by the type.
- Let's see an example,
```rust
struct Employee {
    name: String,
    salary: u16,
}

struct EmployeeRecords {
    employee_db: Vec<Employee>,
}

impl Iterator for EmployeeRecords {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

pub fn main() {
    let mut emp_1 = Employee {
        name: String::from("Employee 1"),
        salary: 1000,
    };
    let mut emp_2 = Employee {
        name: String::from("Employee 2"),
        salary: 2000,
    };
    let mut emp_db = EmployeeRecords {
        employee_db: vec![emp_1, emp_2],
    };

    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
}
```
- In the above code, we have defined a struct `Employee` and `EmployeeRecords`.
- `EmployeeRecords` has a vector of `Employee` and we have implemented `Iterator` trait for `EmployeeRecords`.
- We have defined the associated type `Item` as `String` and implemented the `next` method.
- In the `next` method, we are removing the first element from the vector and returning it.
- `println` statements are used to print the elements of the iterator.
- We get output
```shell
Some("Employee 1")
Some("Employee 2")
None
```
- We can also use `for` loop to iterate over the elements of the iterator, since we have implemented `Iterator` trait for `EmployeeRecords`.
```rust
for employee in emp_db {
    println!("{employee}");
}
```
- Above code will print the elements of the iterator.
- For loop is smart enough to call the `next` method on the iterator and iterate over the elements.
- It will stop when `next` method returns `None`.
- `for loop` will be automatically end when the `None` variant is encountered.
- Furthermore, all the values will be unwrapped.
- We get output as
```shell
Employee 1
Employee 2
```
- We can also return our custom defined type from the `next` method, as long as we stick to basic rules of the `Iterator` trait.
- **Basic Rule of Iterator Trait:**
    - `Iterator` trait must have a mandatory function `next` which returns `Option<Self::Item>`.
    - `Items` returned must have to the same type.

-------------------------------------------------------
## IntoIterator
-------------------------------------------------------
- How the trait is defined in std library.
```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```
- There is a key difference between `Iterator` and `IntoIterator` trait. `Iterator` is implemented on a type over which you can iterate over, and `IntoIterator` is implemented on a type which can be converted into an iterator.
```rust
struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}

```
- `BookIterator` is created to iterate over the properties of the `Book` struct.
- First we need a struct, which will hold an `Iterator State`, and by state we mean the current item, which needs to be returned by the next method. And then we need to implement iterator trait for this struct.
- Here `BookIterator` is such a struct.
```rust
impl Iterator for BookIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.properties.is_empty() {
            Some(self.properties.remove(0))
        } else {
            None
        }
    }
}
```
- This is a simple implementation of the `Iterator` trait for the `BookIterator` struct.
- This will iterate over properties of the `Book` struct.
- Now let's implement `IntoIterator` trait for the `Book` struct.
```rust
impl IntoIterator for Book {
    type Item = String;
    type IntoIter = BookIterator;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator {
            properties: vec![self.title, self.author, self.genre],
        }
    }
}
```
- Above code will transform the `Book` struct into an iterator, i.e. a type which has implemented the `Iterator` trait, here `BookIterator`.
- Let's use it in main.
```rust
pub fn main() {
    let book_1 = Book {
        title: "Harry Potter and the Philosopher's Stone".to_string(),
        author: "J.K. Rowling".to_string(),
        genre: "Fantasy".to_string(),
    };

    let mut book_iterator = book_1.into_iter();

    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
}
```
- `into_iter()` method is used to convert the `Book` struct into an iterator, on which we can call `next` method.
- The main advantage of `IntoIterator` trait is that it allows us to convert a type into an iterator, which can be used in `for` loop or other iterator consuming context/methods.
```rust
for property in book_iterator {
    println!("{}", property);
}
```
- Above code will print the properties of the `Book` struct.
- **NOTE:** We have used `BookIterator` type when implementing `IntoIterator` trait for the `Book` struct, but we can also use `std::vec::IntoIter` type, which is a type of iterator returned by the `into_iter` method of a vector.
```rust
impl IntoIterator for Book {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>

    fn into_iter(self) -> Self::IntoIter {
        vec![self.title, self.author, self.genre].into_iter()
    }
}
```
- In this case we are using `std::vec::IntoIter` type, which is a type of iterator returned by the `into_iter` method of a vector.
- This is a more idiomatic way of implementing `IntoIterator` trait for the `Book` struct.
- This is because `std::vec::IntoIter` is a type which is used to iterate over the elements of a vector, and it is more efficient than the `BookIterator` type.
- And we wouldn't have to implement the `Iterator` trait for the `BookIterator` type, or even define the `BookIterator` type.
-------------------------------------------------------
## Iterating Through Collections
-------------------------------------------------------
- **A very common use case for an iterator is to iterate over the elements of a collection.**
- Collections in Rust standard library can be converted into iterators.
- Two commonly used collections in Rust are `Vec` and `HashMap`.
- There are three basic methods which are used to create an iterator from a collection, depending upon how you would reference the values in the collection.
    - `iter()` -> Gives us iterator over immutable references to items in the collection.
    - `iter_mut()` -> Gives us iterator over mutable references to items in the collection.
    - `into_iter()` -> Gives us iterator over owned items in the collection.
- `iter()` method is used to iterate over the collection without consuming it, or mutating it.
- This can be seen using the `next()` method on the iterator.
```rust
pub fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    let mut vec_1_iter = vec_1.iter();
    let value_1 = vec_1_iter.next();
    println!("Value 1: {:?}", value_1);
}
```
- The type of `value_1` is `Option<&i32>`, because `iter()` method returns an iterator over immutable references to the items in the collection.
- `iter_mut()` method is used to iterate over the collection with mutable references to the items in the collection, it can be used to mutate the collection.
- When using the collection in a for loop, rust can infer the type of the iterator.
```rust
// Case I
for values in &vec_1 {
    println!("Value: {}", values);
}

// Case 2
println!("Iterating through mutable vector");
for values in &mut vec_1 {
    *values += 50;
    println!("Value: {}", values);
}

// Case 3
println!("Iterating through vector using into_iter");
let vec_2 = vec![45, 30, 85, 90, 41, 39];
for values in vec_2{
    println!("Value: {}", values);
}
```
- Rust compiler will automatically designate `values` in case as `&i32`, for case 2, it will designate `values` as `&mut i32`, and for case 3, it will designate `values` as `i32`.
- After using owning the values in the collection, in the third case, vector `vec_2` will be consumed and we won't be able to use it again.
=======================================================
- Let's analyze some examples with HashMap.
```rust
println!("Examples with HashMap");
let mut person: HashMap<String, i32> = HashMap::new();
person.insert("John".to_string(), 23);
person.insert("Doe".to_string(), 24);
person.insert("Smith".to_string(), 25);

println!("Iterating through HashMap, borrowing immutable reference");
for (name, age) in &person {
    println!("Name: {}, Age: {}", name, age);
}

println!("Iterating through HashMap, borrowing mutable reference");

for (name, age) in &mut person {
    *age += 1;
    println!("Name: {}, Age: {}", name, age);
}
```
- In the above code, rust will automatically choose the type of `name` and `age` based on the use of the values in the for loop.
- In the first case, `name` and `age` will be `(&String, &i32)`, and in the second case, `name` and `age` will be `(&String, &mut i32)`.
- We can see that even when using `mutable iterators` in second case, we still get `immutable reference` to the key of the HashMap.
- Because the key of the HashMap is immutable, and we can't change it.
- We can also own the values.
-------------------------------------------------------
## Combinators
-------------------------------------------------------

- Consider the following vector containing names of the fruit.
```rust
let words = vec!["apple", "banana", "grape", "orange", "watermelon", "pineapple", "mango", "strawberry"];
```
- Suppose we want to obtain all the fruit names, that starts with either letter `a` or letter `b`.
- Additionally all such fruit names should be in uppercase.
- One way to do this will be to use the for loops.
```rust
pub fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "watermelon", "pineapple", "mango", "strawberry", "beet", "blueberry", "cherry", "apricot"];

    let mut result: Vec<String> = vec![];

    for word in words {
        if word.starts_with("a") || word.starts_with("b") {
            let uppercase_word = word.to_uppercase();
            result.push(uppercase_word);
        }
    }

    println!("Result: {:?}", result);
}
```
- Above code will give us the desired result, but it is not very concise.
- We can make it look more precise and more readable/clean by using iterators and combinators.
- **Combinators** are compact pure functions created for specific tasks, which can be chained together to perform complex operations.
- `Iterators` come with variety of `combinators` which can be used to perform operations on the elements of the iterator.
- `filter()` is also a useful `combinator` which can be used to filter out the elements of the iterator based on some condition.
- `filter()` takes a closure as an argument, which is used to filter out the elements, it will execute the closure for each element of the iterator, and if the closure returns `true`, the element will be included in the result.
- `map()` is another useful `combinator` which can be used to transform the elements of the iterator.
- `map()` converts the elements of the iterator into another type, by applying the closure to each element.
- `collect()` is used to collect the elements of the iterator into a collection.
- `collect()` combinator requires the type of the collection to be specified, and it will collect the elements of the iterator into that collection.
- There are two ways to specify the type of the collection, either by using turbofish syntax `::<Type>` or by using type inference.
```rust
let fruits = vec!["apple", "banana", "grape", "orange", "watermelon", "pineapple", "mango", "strawberry", "beet", "blueberry", "cherry", "apricot", "avocado", "blackberry", "cantaloupe", "coconut", "fig", "kiwi", "lemon", "lime", "lychee", "nectarine", "papaya", "peach", "pear", "plum", "pomegranate", "raspberry", "tangerine", "tomato", "watermelon"];

let result = fruits
.into_iter()
.filter(|&word| word.starts_with("a") || word.starts_with("b"))
.map(|word| word.to_uppercase())
.collect::<Vec<String>>();
```
- In the above code, we have used `filter()` and `map()` combinators to filter out the elements of the iterator and transform them.
- In the above case, advantage of using combinators is not that apparent, because it was a simple example.
- But, in general, combinators will result in lesser, clearer and cleaner code.
=======================================================
- **FEW NOTES**
    - `Iterators` and method on iterators i.e. `combinators` such as `filter`, `map`, `collect` are lazy. This means that they don't do anything until you call `next` method on the iterator or a method that ultimately calls `next` method.
    - For instance in the above code, no work is done until `collect` method is called. `collect` method calls the `next` method under the hood, which in turn calls the `filter` and `map` methods.
    - There are a lot of combinators, we have only seen a few of them. Some of the other combinators are `enumerate`, `skip`, `take`, etc.
-------------------------------------------------------
## Iterating Through Options
-------------------------------------------------------
- Consider a couple of variables, first variable contains information regarding a product, wrapped around by a variant.
- We also have a vector of products.
- We want to add a logic, that if `some_product` is not `None`, then we want to add it to the vector of products.
- We can do this using `match` statement or `if let` statement.
```rust
pub fn main() {
    let some_product = Some("Product 1");
    let mut products = vec!["Product 2", "Product 3", "Product 4"];

    match some_product {
        Some(product) => products.push(product),
        None => (),
    }

    println!("Products: {:?}", products);
}
```
- Above code will add the product to the vector if `some_product` is not `None`.
- We can also use `if let` statement to achieve the same.
```rust
pub fn main() {
    let some_product = Some("Product 1");
    let mut products = vec!["Product 2", "Product 3", "Product 4"];

    if let Some(product) = some_product {
        products.push(product);
    }

    println!("Products: {:?}", products);
}
```
- However there is even more simpler way, that is by using `extend` method on the vector.
- `extend` method extends the collection with the content of an iterator.
- If the iterator is empty, then nothing will be added to the collection.
```rust
pub fn main() {
    let some_product = Some("Product 1");
    let mut products = vec!["Product 2", "Product 3", "Product 4"];

    products.extend(some_product);

    println!("Products: {:?}", products);
}
```
- Here passing an `Option<>` works because, option implements an `IntoIterator` trait.
- Since `Option` enum implements `IntoIterator` trait, we can use it where an iterator is expected.
- For instance we can use it to chain it with some other iterator, and build a larger iterator.
```rust
let product_iter = products.iter().chain(some_product.iter());

for product in product_iter {
    println!("Product: {}", product);
}
```
- Let's look at one more use case.
- Consider a vector containing some `Option` values, containing some products.
```rust
let products = vec![Some("Charger"), Some("Battery"), None, Some("Headphones"), None, Some("Cable")];
```
- We want to filter out the `None` values from the vector and collect the `Some` values into a new vector.
- Let's use a for loop first
```rust
let mut valid_products: Vec<&str> = vec![];
for p in products {
    if p.is_some() {
        valid_products.push(p.unwrap());
    }
}
```
- We can also use iterators and combinators to achieve the same.
```rust
let valid_products = products
    .into_iter()
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<&str>>();
```
- There is a more easy way to do this, by using `flatten` method on the iterator.
```rust
let valid_products = products.into_iter().flatten().collect::<Vec<&str>>();
```