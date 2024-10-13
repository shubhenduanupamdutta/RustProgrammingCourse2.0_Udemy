//----------------------------------------------------------------
//              Programming Practice
//----------------------------------------------------------------

use programming_practice::{expression_evaluation, stack, string_reversal};

fn main() {
    println!("############### Programming Practice ###############");

    println!();
    println!("############ 1. Stack Implementation ############");
    stack::main();

    println!();
    println!("############ 2. String Reversal ############");
    string_reversal::main();

    println!();
    println!("############ 3. Expression Evaluation ############");
    expression_evaluation::main();
}
