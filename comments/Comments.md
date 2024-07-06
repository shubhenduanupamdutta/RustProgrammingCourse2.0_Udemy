# Comments, Print Commands, Input, Variable Convention and Statics

## Comments
- Comments are used to explain the code. Comments are ignored by the compiler.
- Single line comments start with `//`
- Multi-line comments start with `/*` and end with `*/`

```rust
// This is a single line comment
/* This is a 
multi-line
comment */
```

## Print Commands
- `println!` is used to print a line to the console.
- `print!` is used to print without a newline.

```rust
fn main() {
    println!("Hello, World!");
    print!("Hello, ");
    print!("World!");
}
```

## Escape Sequences
- `\n` is used to print a newline.
- `\t` is used to print a tab.
- `\\` is used to print a backslash.
- `\"` is used to print a double quote.
- `\r` is used to print a carriage return.

## Positional Arguments
- `{}` is used as a placeholder for the arguments passed to `println!`.

```rust
fn main() {
    println!("{} is a {}", "Rust", "systems programming language");
    println!("{0} is a {1} programming language", "Rust", "systems");
    println!("{1} is a {0} programming language", "systems", "Rust");
}
```
## Named Arguments
- `{variable_name}` is used as a placeholder for the arguments passed to `println!`.

```rust
fn main() {
    println!("{language} is a {type} programming language", language="Rust", type="systems");
}
```

## Input
#### We can use std::io::stdin() to take input from the user.
```rust
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("You entered: {}", input);
}
```

## Variable Convention
- Variable names should be in snake_case. 
- Constants should be in SCREAMING_SNAKE_CASE.
- `_` can be added before the variable name to avoid warnings about unused variables.
- `_` can be used in between any numerical types to make them more readable.

## Statics
- Statics are similar to constants.
- They are declared using the `static` keyword.
```rust
static LANGUAGE: &str = "Rust";
```
- Naming convention is similar to constants.
- Essential difference between statics and constants is that constants are inlined. That is they are replaced with their concrete values at compile time.
- Statics are not inlined. They are stored in memory and can be mutable.
- All reference to statics refer to the same location in memory.
- Usually constants are preferred over statics.
- Some reason to use statics are:
    - When you want to refer to some large amount of data in memory.
    - When you are interested in the interior memory.
