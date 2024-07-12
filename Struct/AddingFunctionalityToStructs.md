# Adding Functionality to Structs

#### Structs are a great way to organize data, but they can do more than just hold data. You can also add functions to structs to define behavior specific to the struct. These functions are sometimes called methods.

#### Here's an example of a struct with a method:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let square = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        square.area()
    ); // The area of the rectangle is 900 square pixels.
}

```
- The `impl` keyword, short for implementation, is used to define methods in Rust.
- The `area` method takes a single parameter `&self` because it doesn't need to modify the `Rectangle` instance it's called on.
- Methods can take ownership of `self`, borrow `self immutably`, or borrow `self mutably`, just like any other parameter. Where `self` is the instance of the struct the method is being called on.
    - `&self` - A mutable reference to the instance of the struct.
    - `&mut self` - A mutable reference to the instance of the struct.
    - `self` - The instance of the struct. This will consume the instance, and the instance will no longer be available after the method call.
- `Self` is an alias for the type we're defining methods on in the `impl` block. In this case, `Self` will be `Rectangle`.

## Associated Functions
- Associated functions are also sometimes called `static methods` in other languages.
- They're functions that are connected to the type but don't operate on the instance of the type.
- They also don't take `self`, `&self`, or `&mut self` as a parameter.
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
```
- Here we've defined two associated functions, `square` and `new`.
- We call associated functions with the `::` syntax.
```rust
let square = Rectangle::square(30);
let rectangle = Rectangle::new(30, 50);
```
- Frequently, we add a `new` associated function to a struct to create instances of the struct. But `new` is not enforced by the language. It's a convention, not a requirement.
- In the above example, we've defined a `new` associated function that takes two parameters and returns a `Rectangle` instance.