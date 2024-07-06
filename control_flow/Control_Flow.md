# Control Flow
#### Loops 

## Loop `loop` 
```rust
loop {
    println!("Loop forever!");
}
```
- The `loop` keyword is used to create an infinite loop.
- To exit out of the loop, we can use the `break` keyword.
- The `break` keyword by default only breaks out of the innermost loop.
- To break out of other loops, we can use labels.
```rust
`outer : loop {
    println!("Outer loop");
    `inner : loop {
        println!("Inner loop");
        break `outer;
    }
}
```
- The `outer` label is used to break out of the outer loop.
- The `inner` label is used to break out of the inner loop.
- Loops are an expression and can return a value.
```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
println!("The result is {}", result);
```
- The `loop` block returns the value of the expression after the `break` statement.

## For loops
```rust
for number in 1..4 {
    println!("Number: {}", number);
}
```
- The `for` keyword is used to create a loop that iterates over a range.
- The range is specified using the `..` operator.
- They can also be used to iterate over collections such as vectors and arrays.
```rust
let numbers = vec![1, 2, 3, 4, 5];
for number in numbers {
    println!("Number: {}", number);
}
```

## While loops
```rust
let mut number = 3;
while number != 0 {
    println!("Number: {}", number);
    number -= 1;
}
```
- The `while` keyword is used to create a loop that runs as long as a condition is true.