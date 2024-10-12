# Programming Practice
---------------------------------------------------------
## Stack Implementation
---------------------------------------------------------
### What is stack?
- Stack is a linear data structure which follows a particular order in which the operations are performed. The order it follows is Last In First Out (LIFO).
- The operations can only be performed at one end of the stack which is called the top.
- The operations that can be performed on a stack are:
    - Push: Adds an item to the top stack
    - Pop: Removes an item from the top of the stack
- Some simple application of stack are:
    - String reversal
    - Expression evaluation
    - Backtracking
    - Syntax parsing

=========================================================
- In rust there is many useful collections which provides behavior and functionality similar to that of stack, and other important abstract data types such as heap and linked list.
- We can add some wrapping around these collections to make them behave like stack. Stack can be implemented using `Vec`, `LinkedList` and `VecDeque` in rust.
- We will implement stack using `Vec`.
```rust
fn new_stack(max_size: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_values = stack.pop();
    println!("The popped value is: {:?}", popped_values);
    popped_values
}

fn push(stack: &mut Vec<u32>, value: u32, max_size: usize) {
    println!("Pushing value: {:?}", value);
    if stack.len() == max_size {
        println!("Stack is full. Cannot push value: {:?}", value);
        return;
    }
    stack.push(value);
    println!("Current stack: {:?}", stack);
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}
```
- We are using reference to the vector in `push` and `pop` functions because we want to modify the vector in these functions, but we don't want to take ownership of the vector.
```rust
use std::num::ParseIntError;

fn new_stack(max_size: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    if stack.is_empty() {
        println!("Stack is empty. Cannot pop any value.");
        return None;
    }
    let popped_values = stack.pop();
    println!("The popped value is: {:?}", popped_values.unwrap());
    popped_values
}

fn push(stack: &mut Vec<u32>, value: u32, max_size: usize) {
    println!("Pushing value: {:?}", value);
    if stack.len() == max_size {
        println!("Stack is full. Cannot push value: {:?}", value);
        return;
    }
    stack.push(value);
    println!("Current stack: {:?}", stack);
}

fn size(stack: &[u32]) -> usize {
    stack.len()
}

fn input() -> Result<u32, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input).map_err(|e| e.to_string())?;
    let input: u32 = input
        .trim()
        .parse()
        .map_err(|e: ParseIntError| e.to_string())?;
    Ok(input)
}

pub fn main() {
    println!("Let us first create a stack for our use.");
    println!("please mention the size of the stack you want to create.");
    let stack_size = match input() {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    let mut stack = new_stack(stack_size as usize);
    println!("Stack created with size: {:?}", stack_size);

    loop {
        display_menu();
        let choice = match input() {
            Ok(value) => value,
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        };
        match choice {
            1 => {
                println!("Please enter the value you want to push:");
                let value = match input() {
                    Ok(value) => value,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        continue;
                    }
                };
                push(&mut stack, value, stack_size as usize);
            }
            2 => {
                pop(&mut stack);
            }
            3 => {
                println!("The size of the stack is: {:?}", size(&stack));
            }
            4 => {
                println!("The stack is: {:?}", stack);
            }
            5 => {
                println!("Exiting the stack program.");
                break;
            }
            _ => {
                println!("Invalid option. Please select a valid option.");
            }
        }
    }
}

fn display_menu() {
    println!("********** Menu **********");
    println!("1. Push");
    println!("2. Pop");
    println!("3. Size");
    println!("4. Display");
    println!("5. Exit");
    println!("Enter your choice: ");
}
```
