//----------------------------------------------------------------
//          Real Life Problems with Data Structures in Rust
//----------------------------------------------------------------

use real_life_problems_in_rust::{find_employee_no_meeting, highest_priced_stock, product_popularity, search_word_grouping};

fn main() {
    println!("Real Life Problems with Data Structures in Rust");
    println!("-------------------------------------------------");
    println!();
    println!("1. Search Results with Word Grouping using HashMap and Nested Loops");
    search_word_grouping::main();

    println!();
    println!("2. Product Popularity using HashMap, Loops, and Conditional if");
    product_popularity::main();

    println!();
    println!("3. Highest Priced Stock using MaxStacks, Structures, and Vectors");
    highest_priced_stock::main();
    
    println!();
    println!("4. Find an Employee with No Meeting using MultiDimensional Arrays, Nested Loops, and Conditional if");
    find_employee_no_meeting::main();
}