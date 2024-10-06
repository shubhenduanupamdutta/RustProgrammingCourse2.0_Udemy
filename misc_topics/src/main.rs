//---------------------------------------------------------------------------------
//       Miscellaneous Topics
//---------------------------------------------------------------------------------

use misc_topics::{function_inputs_coercion, performance_lints, programming_tips, todo_and_extensions};

fn main() {
    println!("######### Function Inputs and Coercion #########");
    function_inputs_coercion::main();

    println!();
    println!("######### Programming Tips #########");
    programming_tips::main();

    println!();
    println!("######### ToDo macro and Some Useful Extensions #########");
    todo_and_extensions::main();

    println!();
    println!("######### Performance Lints #########");
    performance_lints::main();
}
