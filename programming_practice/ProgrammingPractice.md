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
---------------------------------------------------------
## String Reversal - An application of stack
---------------------------------------------------------
- We can use stack to reverse a string.
- The idea is to reverse the characters of the string by pushing them into the stack and then popping them out of the stack.
- We will first convert a string to character vector and then reverse the characters using stack.
```rust
use std::num::ParseIntError;

fn new_stack(max_size: usize) -> Vec<char> {
    Vec::with_capacity(max_size)
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped_values = stack.pop();
    // println!("The popped value is: {:?}", popped_values); No need here, because no menu is displayed, since no user input is required.
    popped_values
}

fn push(stack: &mut Vec<char>, value: char, max_size: usize) {
    println!("Pushing value: {:?}", value);
    if stack.len() == max_size {
        // println!("Stack is full. Cannot push value: {:?}", value);
        return;
    }
    stack.push(value);
    // println!("Current stack: {:?}", stack);
}

fn size(stack: &[char]) -> usize {
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
    let input_string = String::from("Welcome to Programming Practice");
    println!("The input string is: {:?}", input_string);
    let string_size = input_string.len();
    let mut stack = new_stack(string_size);

    let mut rev_string = String::new();

    for c in input_string.chars() {
        push(&mut stack, c, string_size);
    }

    for _ in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("The reversed string is: {:?}", rev_string);
}
```
---------------------------------------------------------
## Expression Evaluation - An application of stack
---------------------------------------------------------
### Algorithm for Expression Evaluation
=========================================================
- There are many ways a mathematical expression can be evaluated. 
- The method we will use is to 
    - first convert the given expression into a postfix expression.
    - And then using postfix expression evaluate the original expression.

- A mathematical expression consists of operand and operators.
    - **Operand**: Operands are variable or constant values that we used inside an expression such as `3`, `44`, `-5`, `x`, `y`, etc.
    - **Operator**: Operators are symbols that represent specific operations such as addition(`+`), subtraction(`-`), multiplication(`*`), division(`/`), exponent(`^`) etc.

- `Postfix Expression` - Given the operands and operators, a _postfix expression_ is an expression in which the operators come after their operands. For example, the infix expression `3 + 4` can be written as `3 4 +` in postfix notation.
- If expression grows, _postfix expression_ will become more complicated.
- The postfix expression is also known as `Reverse Polish Notation (RPN)`.
- `Infix Expression` - An expression in which the operators come between their operands. For example, `3 + 4`. The notation which we generally use in our daily life are _infix expressions_.
====================================================================
#### Converting Infix to Postfix Expression
- To convert an infix expression to a postfix expression, we follow few rules,
    - **Priorities of Operators**: 
        - **1.** `+` and `-` have the lowest priority .
        - **2.** `*` and `/` have higher priority than `+` and `-`.
        - **3.** `^` has the highest priority.
        - All the operators have higher priority than `(`.
    - If scanned operator is less than or equal to (`<=`) to the top of stack in priority, then we will pop the operators until we have low priority operator.
    - If we have `(` then will push it to the stack.
    - If we have `)` then we will pop the operators until we have `(`. We will not add `(` and `)` to the postfix expression, discarding them, and add all the popped operators to the postfix expression.
    - If we an operand we will just add it to the postfix expression.
- **Second Rule Explanation** - What happens in the algorithm, we will be scanning the expression from left to right and we will add elements to the stack one by one. If we scan an operator whose priority is equal to or less than the priority of the top of the stack, then we will pop elements from the stack until the top of the stack has lesser priority than the scanned character. _All the popped elements will be added to the postfix expression._ When this position is reached we will push the scanned operator to the stack.
=========================================================
### Example for Infix to Postfix Expression
=========================================================
- **Infix Expression**: (33 + 45/3 * (2 + 9) -50)

|Step| Symbol | Stack         | Postfix Expression       |
|----|--------|---------------|--------------------------|
|1   |  (     | (             |                          |
|2   | 33     | (             | 33                       |
|3   | +      | (, +          | 33                       |
|4   | 45     | (, +          | 33, 45                   |
|5   | /      | (, +, /       | 33, 45                   |
|6   | 3      | (, +, /       | 33, 45, 3                |
|7   | *      | (, +, *       | 33, 45, 3, /             |
|8   | (      | (, +, *, (    | 33, 45, 3, /             |
|9   | 2      | (, +, *, (    | 33, 45, 3, /, 2          |
|10  | +      | (, +, *, (, + | 33, 45, 3, /, 2          |
|11  | 9      | (, +, *, (, + | 33, 45, 3, /, 2, 9       |
|12  | )      | (, +, *       | 33, 45, 3, /, 2, 9, +    |
|13  | -      | (, -          | 33, 45, 3, /, 2, 9, +, *, + |
|14  | 50     | (, -          | 33, 45, 3, /, 2, 9, +, *, +, 50 |
|15  | )      |               | 33, 45, 3, /, 2, 9, +, *, +, 50, - |

- Step by Step Explanation:
    - **Step 1** - We have `(`, so we will push it to the stack.
    - **Step 2** - We have `33`, so we will add it to the postfix expression, since it is an operand.
    - **Step 3** - We have `+`, so we will push it to the stack.
    - **Step 4** - We have `45`, so we will add it to the postfix expression, since it is an operand.
    - **Step 5** - We have `/`, so we will push it to the stack, since its priority is higher than `+`.
    - **Step 6** - We have `3`, so we will add it to the postfix expression, since it is an operand.
    - **Step 7** - We have `*`, this has <= priority to `/` (actually equal priority) so we will pop `/` and we reach `+` which has lower priority than `*`, so we will push `*` to the stack. And popped `/` will be added to the postfix expression.
    - **Step 8** - We have `(`, so we will push it to the stack.
    - **Step 9** - We have `2`, so we will add it to the postfix expression, since it is an operand.
    - **Step 10** - We have `+`, so we will push it to the stack, since its priority is higher than `(`.
    - **Step 11** - We have `9`, so we will add it to the postfix expression, since it is an operand.
    - **Step 12** - We have `)`, so we will pop the operators until we reach `(`. We will not add `(` and `)` to the postfix expression, discarding them, and add all the popped operators to the postfix expression. So in here, we only need to pop `+` and add it to the postfix expression, and discard `(` and `)`.
    - **Step 13** - We have `-`, this has lower priority than `+`, so will pop the stack until we reach lower priority than `-`, in this case, until we reach `(`. So we will pop `*` and `+` and add them to the postfix expression. And then we will push `-` to the stack.
    - **Step 14** - We have `50`, so we will add it to the postfix expression, since it is an operand.
    - **Step 15** - We have `)`, so we will pop the operators until we reach `(`. We will not add `(` and `)` to the postfix expression, discarding them, and add all the popped operators `-` to the postfix expression.
    - Now last step is to empty the stack and add all rest of the operators to the postfix expression, but both `infix expression` and `stack` are already empty, so we are done.
- So final **Postfix Expression** is `33 45 3 / 2 9 + * + 50 -`

=========================================================
### Evaluating Postfix Expression
========================================================
- #### Rules for evaluating postfix expression
    - **Rule 1**: Scan the postfix expression from left to right.
    - **Rule 2**: If the scanned character is an operand, push it to the stack.
    - **Rule 3**: If we scan an operator, then pop up the two elements from the stack, apply the operator to the popped elements and push the result back to the stack. 
    ( **_NOTE:_ The order of the popped elements is important. For example, if we have `3 4 +`, then we will pop `4` first and then `3`, and apply `+` to them. That is first popped element will be the second operand and second popped element will be the first operand. So if we have `45 5 /` then we will pop `5` first and then `45` and apply `/` to them, and result will be `9`.**)
    - **Rule 4**: Repeat the above steps until the postfix expression is scanned completely.

=========================================================
### Example for Evaluating Postfix Expression
=========================================================
- **Postfix Expression**: 33 45 3 / 2 9 + * + 50 -
| Step |    Symbol  |      Stack          |
|------|------------|---------------------|
|  1   |  33        | 33                  |
|  2   |  45        | 33, 45              |
|  3   |  3         | 33, 45, 3           |
|  4   |  /         | 33, 15              |
|  5   |  2         | 33, 15, 2           |
|  6   |  9         | 33, 15, 2, 9        |
|  7   |  +         | 33, 15, 11          |
|  8   |  *         | 33, 165             |
|  9   |  +         | 198                 |
|  10  |  50        | 198, 50             |
|  11  |  -         | 148                 |



- Step by Step Explanation:
    - **Step 1** - We have `33`, so we will push it to the stack.
    - **Step 2** - We have `45`, so we will push it to the stack.
    - **Step 3** - We have `3`, so we will push it to the stack.
    - **Step 4** - We have `/`, so we will pop `3` and `45` from the stack, apply `/` to them and push the result `15` to the stack.
    - **Step 5** - We have `2`, so we will push it to the stack.
    - **Step 6** - We have `9`, so we will push it to the stack.
    - **Step 7** - We have `+`, so we will pop `9` and `2` from the stack, apply `+` to them and push the result `11` to the stack.
    - **Step 8** - We have `*`, so we will pop `11` and `15` from the stack, apply `*` to them and push the result `165` to the stack.
    - **Step 9** - We have `+`, so we will pop `165` and `33` from the stack, apply `+` to them and push the result `198` to the stack.
    - **Step 10** - We have `50` we will push it on the stack.
    - **Step 11** - We have `-`, so we will pop `50` and `198` from the stack, apply `-` to them. First operand will be `198` and second operand will be `50`. So the result will be `148`. We will push the result to the stack.
    - Now the postfix expression is scanned completely and we have only one element in the stack, which is the result of the expression. So the result of the expression is `148`.
- So the final result of the expression `(33 + 45/3 * (2 + 9) -50)` is `148`.
- `(33 + 15 + 11 - 50) = 148`
- Our Result is correct.
---------------------------------------------------------
## Implementing Expression Evaluation
---------------------------------------------------------
- We will implement the infix to postfix conversion and postfix expression evaluation using stack.
- We can't directly scan a string to get the actual numbers, since the numbers may have multiple characters.
- So we will create a function, which will return to use individual operands and operators (i.e. symbols) from the string expression.
- Then we will create a function to convert infix expression to postfix expression.
- Then we will create a function to evaluate the postfix expression.
```rust
/**
 * Rules for converting to postfix notation:
 *
 * 1. Priorities of operators:
 *   - 1. Open Parenthesis '('
 *   - 2. +, -
 *   - 3. *, /
 *   - 4. ^
 *
 * 2. If scanned character has priority <= to the operator at the top of the stack, then pop the
 * stack until lower priority operator is reaches. Add popped symbols to the postfix expression.
 *
 * 3. If "(" is encountered, push it to the stack.
 *
 * 4. If ")" is encountered, pop the stack until "(" is reached. Add popped symbols to the postfix.
 *
 * 5. If scanned character is an operand, add it to the postfix expression.
 */

fn new_stack(max_size: usize) -> Vec<String> {
    Vec::with_capacity(max_size)
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    stack.pop()
}

fn push(stack: &mut Vec<String>, item: String, max_size: usize) {
    if stack.len() == max_size {
        println!("Stack is full");
    } else {
        stack.push(item);
    }
}

fn size(stack: &[String]) -> usize {
    stack.len()
}

fn individual_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input = Vec::new();

    let input_chars: Vec<char> = input_expr.chars().collect();

    let mut temp: Vec<char> = Vec::new();

    for c in input_chars {
        match c {
            '/' | '*' | '+' | '-' | '^' | '(' | ')' => {
                if temp.is_empty() {
                    // If temp is empty, then push the current character to the tokenized_input
                    // Don't iter through the temp vector
                    tokenized_input.push(c.to_string());
                    continue;
                }
                tokenized_input.push(temp.iter().collect());
                tokenized_input.push(c.to_string());
                temp.clear();
            }
            _ => {
                temp.push(c);
            }
        }
    }

    // If after for loop, the string doesn't end with an operator, temp will not be empty
    if !temp.is_empty() {
        tokenized_input.push(temp.iter().collect());
    }

    println!(
        "Converted input expression to individual symbols: {:?}",
        tokenized_input
    );
    tokenized_input
}

fn infix_to_postfix(symbols: Vec<String>) -> Vec<String> {
    let size_expr = symbols.len();
    let mut stack = new_stack(size_expr);
    let mut postfix_expr: Vec<String> = Vec::new();

    for i in symbols {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                if stack.is_empty() {
                    push(&mut stack, i, size_expr);
                    continue;
                }
                let priority = operand_priority(&i);
                if priority > operand_priority(stack.last().unwrap()) {
                    push(&mut stack, i, size_expr);
                    continue;
                }
                while priority <= operand_priority(stack.last().unwrap()) {
                    postfix_expr.push(pop(&mut stack).unwrap());
                    if stack.is_empty() {
                        break;
                    }
                }
                push(&mut stack, i, size_expr);
            }
            "(" => {
                push(&mut stack, i, size_expr);
            }
            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix_expr.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack);    // Pop the open parenthesis
            }
            _ => {
                postfix_expr.push(i);
            }
        }
    }

    while !stack.is_empty() {
        postfix_expr.push(pop(&mut stack).unwrap());
    }

    println!(
        "Converted infix expression to postfix expression:\n=> {:?}",
        postfix_expr
    );
    postfix_expr
}

fn operand_priority(operator: &String) -> u8 {
    match operator.as_str() {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0,
    }
}

/**
 * Rules for evaluating postfix expression:
 * 
 * 1. If operand -> push to stack
 * 2. If operator -> pop two operands from stack, perform operation, push result to stack
 *    (operand1 operator operand2), operand2 is popped first, then operand1
 * 3. Continue until all symbols are processed
 * 4. The final result will be the top of the stack
 */
fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    let size_expr = size(&postfix);

    let mut result_stack = new_stack(size_expr);

    for symbol in postfix {
        match symbol.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                let operator = symbol;
                let operand2 = pop(&mut result_stack).unwrap();
                let operand1 = pop(&mut result_stack).unwrap();
                let result = operation(operand1, operand2, operator);

                push(&mut result_stack, result.to_string(), size_expr);
            }
            _ => {
                push(&mut result_stack, symbol, size_expr);
            }
        }
    }

    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}

fn operation(op1: String, op2: String, operator: String) -> f32 {
    let operand1 = op1.parse::<f32>().unwrap();
    let operand2 = op2.parse::<f32>().unwrap();

    match operator.as_str() {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" => operand1 * operand2,
        "/" => operand1 / operand2,
        "^" => operand1.powf(operand2),
        _ => 0.0,
    }
}

pub fn main() {
    let input_expr = String::from("(33+45/3*(2+9)-50)");
    println!(
        "The original mathematical expression to evaluate is:\n=> {}",
        input_expr
    );

    let tokenized_expression = individual_symbols(input_expr);
    let postfix_expression = infix_to_postfix(tokenized_expression);

    println!("The evaluated result is:\n=> {}", postfix_evaluation(postfix_expression));
}
```
---------------------------------------------------------
