//--------------------------------------------------------------------
//      Text Processing (RegEx), File Handling and Directory Handling
//--------------------------------------------------------------------

use file_directory_regex::{basic_file_handling, directory_and_path};

fn main() {
    println!("########### Basic File Handling ###########");
    basic_file_handling::main();

    println!();
    println!("########### Directory and Path Handling ###########");
    directory_and_path::main();
}
