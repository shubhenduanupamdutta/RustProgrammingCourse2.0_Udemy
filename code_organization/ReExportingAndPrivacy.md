# Re-Exporting and Privacy

## Re-Exporting
- We can use modules defined in main using `use` keyword, for example
```rust
use code_organization::customer::Customer;
```
- This will import the `Customer` struct from the `customer` module in the `code_organization` crate.
- But we shouldn't be making unnecessary items public.

#### There is a way to bringing some item from inside the module into the scope of outside the crate world, without making the module public. To do this, we can use `pub use` keyword.
- `pub use` allows us to `re-export` the items from the module, without making the module public.
- Only the items that are re-exported will be available to the outside world.
```rust
// in lib.rs
mod customer;
pub use customer::Customer;
```
```rust
// in main.rs
use code_organization::Customer;
```
- Now, `Customer` struct can be used in the main.rs file without making the `customer` module public.
- This way, we can control what items are available to the outside world.
- In main.rs, we can use `Customer` struct, but we can't use `customer` module.
- To export modules, which are not at the top level, we have to first re-export them at the top of the module level.
- For example in our code, we have to first re-export `Category` enum in the `product` module, to make it available in the `lib.rs` file.
```rust
//in product.rs
pub use category::Category
```
```rust
//in lib.rs
mod product;
pub use product::{Product, Category};
```
- This re-exports both `Product` and `Category` from the `product` module, event though `Category` is defined in the `category` module inside the `product` module.
```rust
// in main.rs
use code_organization::{Product, Category};
```
- Now, `Product` and `Category` can be used in the `main.rs` file.

## Privacy
- In Rust, by default, everything is private.
- In our case, if we use `Product` struct to create an item in main file, in this hierarchy
```rust
// in main.rs
use code_organization::Product;
fn main() {
    let product = Product {
        id: 1,
        name: String::from("Laptop"),
        price: 50000.00,
        category: Category::Electronics,
    };
}
```
- We will get an error `Fields are private`.
- Even though we have re-exported `Product` and `Category` in the `lib.rs` file, the fields of `Product` struct are private.
- In rust, making a struct public doesn't make its fields public.
- To get around this error, there are two approaches:
    - Make the fields public.
    - Provide public methods to access the fields.

```rust
// Approach 1: Make the fields public
// in product.rs
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub category: Category,
}
```
- This will make all the fields of the `Product` struct public, and we can access them in the main file.

```rust
// Approach 2: Provide public methods to access the fields
// in product.rs
impl Product {
    pub fn new(id: u32, name: String, price: f64, category: Category) -> Self {
        Self {
            id: id,
            name: name,
            price: price,
            category: category,
        }
    }
}
```
- In second approach, a shorthand method is to mention only once, no need to use `id:id` etc.
```rust
// in product.rs
impl Product {
    pub fn new(id: u32, name: String, price: f64, category: Category) -> Self {
        Self { id, name, price, category }
    }
}
```
- **NOTE**: Privacy of `Enums` is different from that of `Structs`. If an `Enum` is public, all its variants are public by default.
