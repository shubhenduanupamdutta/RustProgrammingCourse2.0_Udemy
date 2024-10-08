//----------------------------------------------
//        Macros in Rust
//----------------------------------------------

use macros::{assignment_q_1, assignment_q_2, assignment_q_3, capturing_types, macros_basics, question_mark_operator, repeating_patterns};

fn main() {
    println!("############# Macros in Rust #############");

    println!();
    println!("########## 1. Macros Basics ##########");
    macros_basics::main();

    println!();
    println!("########## 2. Capturing Types ##########");
    capturing_types::main();

    println!();
    println!("########## 3. Repeating Patterns ##########");
    repeating_patterns::main();

    println!();
    println!("########### 4. Assignment Q1 ###########");
    assignment_q_1::main();

    println!();
    println!("########### 5. Assignment Q2 ###########");
    assignment_q_2::main();

    println!();
    println!("########### 6. Assignment Q3 ###########");
    assignment_q_3::main();

    println!();
    println!("########## 7. Question Mark Operator ##########");
    question_mark_operator::main();
}