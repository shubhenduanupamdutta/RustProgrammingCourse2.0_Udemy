# Function and Code Blocks

## Functions
- Functions are a way to group code together and give it a name.
- Naming conventions for functions in rust are snake case, i.e. all lowercase with underscores between words.
```rust
fn function_name() {
    // code here
}
```
- Functions can take parameters.
- Parameters are defined like variables, that means you need to specify the type of the parameter.
```rust
fn function_name(parameter1: i32, parameter2: i32) {
    // code here
}
```
- The return value of a function is specified using arrow syntax followed by the return type.
```rust
fn function_name(parameter1: i32, parameter2: i32) -> i32 {
    // code here
    return 0;
}
```
- **Statement**: Statement are line of code that perform some action and do not return a value.
- **Expression**: Expressions evaluate to a resulting value.
```rust
fn multiplication(x: i32, y: i32) -> i32 {
    println!("Multiplying {} and {}", x, y);    // Statement
    x * y   // Expression (return value as tail expression)
}
```
- **Tail Expression**: The last expression in a function is the return value of the function. It is not necessary to use the `return` keyword. There must not be a semicolon at the end of the expression.
- Returned value can be stored in another variable.
```rust
let result = multiplication(5, 10);
```
- Multiple values can be returned from a function using a tuple.
```rust
fn return_multiple_values() -> (i32, i32) {
    (5, 10)
}
```

## Code Blocks
- Code blocks are used to group statements together.
- Code blocks are defined using curly braces `{}`.
- Code blocks can be used to define the scope of variables.
```rust
{
    let x = 5;
    println!("Value of x: {}", x);
}
```
- Like functions, code blocks can also return values.
- The last expression in a code block is the return value of the code block.
```rust
let y = {
    let x = 5;
    x + 1
};
println!("Value of y: {}", y);
```
```rust
let full_name = {
    let first_name = "Shubhendu";
    let last_name = "Dutta";
    format!("{} {}", first_name, last_name)
};
```
- `format!` is a macro that returns a `String` type. It is used for string formatting in rust.

## Notes
- Code blocks share some similarities with functions.
    - Code blocks are used to group statements together.
    - Code blocks can return values.
    - May have variables which are limited in scope to their bodies.
- However, there are some key differences
    - Code blocks are not designed for reuse, but functions are.
    - Code blocks don't have a explicit parameters or return types.
    - All the variable in the scope, in which code block is defined, are accessible inside the code block.
    - Functions have an explicit list of parameters, and can only have variables which are explicitly defined as parameters or locally defined.