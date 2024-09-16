//----------------------------------------------------------------
//           Highest Priced Stock
//              - Description
//                  - We have weekwise stock prices, we want to retrieve the highest
//                    stock price in any week in little to no time.
//              - Tools
//                  - MaxStacks, Structures, Vectors
//----------------------------------------------------------------
struct MaxStack {
    main_stack: Vec<i32>,
    max_stack: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            main_stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.main_stack.push(value);
        if !self.max_stack.is_empty() && (self.max_stack.last().unwrap() > &value) {
            self.max_stack.push(*self.max_stack.last().unwrap());
        } else {
            self.max_stack.push(value);
        }
    }

    fn pop(&mut self) {
        self.main_stack.pop();
        self.max_stack.pop();
    }

    fn max_value(&self) -> i32 {
        *self.max_stack.last().unwrap_or(&0)
    }
}

pub fn main() {
    let mut stack = MaxStack::new();
    stack.push(55);
    stack.push(80);
    stack.push(120);
    stack.push(99);
    stack.push(22);
    stack.push(140);
    stack.push(145);

    println!("Maximum value of stock: {}", stack.max_value());

    println!("After going back one week");
    stack.pop();
    println!("Maximum value of stock: {}", stack.max_value());
}
