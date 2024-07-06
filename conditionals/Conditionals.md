# Conditionals

## If Else Statements

```rust
fn main() {
    let num = 40;
    if num < 50 {
        println!("Number is less than 50");
    } else {
        println!("Number is greater than 50");
    }
}
```
- If the condition is true, the code block inside the `if` statement will be executed.
- If the condition is false, the code block inside the `else` statement will be executed.
- The `else` block is optional.
- The conditions in the `if` statement must be of type `bool`.

## If Else If Ladder
```rust
let marks = 95;
    let mut grade = 'N';

    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else {
        grade = 'F';
    }
```
- The `else if` statement is used to check multiple conditions.
- The `else if` statement must be preceded by an `if` statement.
- The `else if` statement is optional.
- Final `else` block is also optional, and must be at the end of the ladder.
- Final `else` block is executed if none of the conditions are true and it can be treated as the default.

#### If else and else if ladder are expressions in Rust, i.e. they return a value.
```rust
let marks = 95;

    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'F'
    };
```
- If we are using if else/ if else if ladder as an expression, then all the branches must return the same type, and the last curly braces must be followed by a semicolon.

## Match Expression
- They allow us to compare a value against a series of patterns and then execute code based on which pattern matches, the first one.
```rust
let marks = 95;
let grade = match marks {
    90..=100 => grade = 'A',
    80..=89 => grade = 'B',
    70..=79 => grade = 'C',
    _ => grade = 'F',
}
```
- The `match` keyword is followed by the value to match against.
- The `match` expression is followed by a series of `arms` or `branches`.
- Each `arm` has two parts: a pattern and some code.
- Right side is the pattern to match against.
- Left side is the code to execute if the pattern matches. If there are multiple lines of code is needed, they need to be enclosed in curly braces.
- *..=* is the range operator, which includes the end value.
- Rust enforces exhaustive pattern matching, i.e. all possible values must be covered.
- The `_` is a catch-all pattern, which matches anything that wasn't matched by the other patterns.

## Common Pitfalls to avoid
### Match
- The match arms must return the same type.
- The `_` pattern must be present in the match expression.
- The match arms must be exhaustive.
- **Unreachable Pattern:** It may happen that some of the arms are unreachable, and the compiler will throw an error. It will happen when all possible values are already covered by the previous arms.
```rust
let marks = 95;
match marks {
    90..=100 => grade = 'A',
    80..=89 => grade = 'B',
    70..=79 => grade = 'C',
    _ => grade = 'F',
    0..=69 => grade = 'F',  // Unreachable Pattern
}
```
- **Overlapping Patterns:** It may happen that some of the arms are overlapping, and the compiler will throw an error. It will happen when the range of values in one arm is already covered by the previous arms.
```rust
let marks = 95;
match marks {
    90..=100 => grade = 'A',
    80..=90 => grade = 'B', // Overlapping Pattern
    70..=79 => grade = 'C',
    _ => grade = 'F',
}
```