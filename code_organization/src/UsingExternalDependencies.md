# Using External Dependencies

```rust
let product1 = Product::new(1, "Laptop".to_string(), 70_000.00, Category::Electronics);
    let product2 = Product::new(2, "Mouse".to_string(), 500.00, Category::Electronics);
    let product3 = Product::new(3, "T-Shirt".to_string(), 1_000.00, Category::Clothing);
    let product4 = Product::new(
        4,
        "Harry Potter and Philosopher's Stone".to_string(),
        1_500.00,
        Category::Books,
    );

    let set1 = vec![&product1, &product2, &product3];
    let set2: Vec<&Product> = vec![&product2, &product4];
```
- Let's look at the above code, we have two sets of product, `set1` and `set2`, defined as vectors of references to `Product` instances.
- We are interested in finding the common products in both sets, i.e. intersection of two sets.
- The rust standard library does not provide a direct way to find the intersection of two vectors.
- One way to find the intersection is to write the code for intersection yourself. This approach will involve 
    - The Cost to write the code,
    - The Cost to test the code,
    - The Cost to maintain the code.
- Which while may be suitable for this, but not for large problems not available in the standard library.
- The other way is to request the implementors to provide the intersection functionality. This may be even more costly.
- The third way is to use external dependencies. Rust has a package manager called `cargo` which can be used to manage dependencies.

## Using External Dependencies
- Rust has a public repository called `crates.io` which hosts a large number of crates, which can be used as dependencies in your project.
- After searching on `crates.io`, we found `array_tools` crate which provides the functionality to find the intersection of two vectors.
- To use the `array_tools` crate, add the following line to the `Cargo.toml` file.
```toml
[dependencies]
array_tools = "1.0.3"
```
- The above line tells `cargo` to download the `array_tools` crate with version `1.0.3` from `crates.io`.
- After adding the dependency, we can use the array_tools crate by adding the following line to the `main.rs` file.
```rust
use array_tools::vec::*;
```
- `*` imports all the functions and types from the `vec` module of the `array_tools` crate.
- Now, we can use the `intersection` function from the `array_tools` crate to find the intersection of two vectors.
```rust
let intersection = set1.intersect(set2);
```
- But this function will not work as such, because intersect function requires train `PartialEq`.
- So we modify the `Product` and `Category` structs to derive `PartialEq` and `Debug` traits.
```rust
// In product.rs
#[derive(PartialEq, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub category: Category,
}
```
```rust
// In category.rs
#[derive(PartialEq, Debug)]
pub enum Category {
    Electronics,
    Clothing,
    Books,
}
```
- Typically for a single task, we will find many available crates on `crates.io` which can be used as dependencies in your project.
- Using dependency has many advantages,
    - *Reusability:* If there are well tested library/crates available which has functionality you need, then its often better to use them rather than reinventing the wheel. This applies to common tasks like parsing JSON, working with databases, http, encryption, etc.
    - *Focus on Core Logic:* Depending on external crates for non-core functionality allows you to focus on the unique core logic of your application. This separation of concern can make your codebase more maintainable and easy to understand.
    - *Community Standard:* Certain crates are considered as community standard for certain tasks. For example, `serde` is a popular crate for serialization and deserialization in Rust. Using such crates can make your codebase more familiar to other Rust developers, improving code readability and maintainability.

## Consideration in Using External Dependencies
- **Maintenance:** Choose crates/libraries that are actively maintained.
- **Excessive Dependencies:** Be cautious of adding too many dependencies. Each dependency adds complexity to your project, and increases your binary size.
- **Understandability:** Prefer using those dependencies which are well documented and easy to understand. Sometimes, it becomes crucial to understand the internals of the dependency to debug issues.
