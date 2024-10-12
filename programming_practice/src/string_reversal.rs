//----------------------------------------------------------------
//           String Reversal
//----------------------------------------------------------------

fn new_stack(max_size: usize) -> Vec<char> {
    Vec::with_capacity(max_size)
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    stack.pop()
}

fn push(stack: &mut Vec<char>, value: char, max_size: usize) {
    // println!("Pushing value: {:?}", value);
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
