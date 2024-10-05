//--------------------------------------------------------------------
//      Text Processing (RegEx), File Handling and Directory Handling
//--------------------------------------------------------------------

use file_directory_regex::{basic_file_handling, directory_and_path, regular_expressions, repetitions_quantifiers_capture_groups, string_literals};

fn main() {
    println!("########### Basic File Handling ###########");
    basic_file_handling::main();

    println!();
    println!("########### Directory and Path Handling ###########");
    directory_and_path::main();

    println!();
    println!("########### Regular Expressions ###########");
    regular_expressions::main();

    println!();
    println!("########### Repetitions, Quantifiers, Capture Groups ###########");
    repetitions_quantifiers_capture_groups::main();

    println!();
    println!("########### String Literals ###########");
    string_literals::main();

}
