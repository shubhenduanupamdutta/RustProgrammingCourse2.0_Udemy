//----------------------------------------------------------------
//        Expression Evaluation
//----------------------------------------------------------------

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
