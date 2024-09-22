//----------------------------------------------------------------
//          Real Life Problems with Data Structures in Rust
//----------------------------------------------------------------

use real_life_problems_in_rust::{
    fetching_top_products, find_employee_no_meeting, highest_priced_stock, items_in_range, items_suggestions, longest_non_stop_working_hours, participants_online, product_popularity, recently_used, search_word_grouping, storage_retrieval
};

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

    println!();
    println!("5. Longest Non-Stop Working Hours using HashSet, Vectors and Loops");
    longest_non_stop_working_hours::main();

    println!();
    println!("6. Items Suggestions using HashMap and Vectors");
    items_suggestions::main();

    println!();
    println!("7. Items in Range using Binary Search, Vectors, and Loops");
    items_in_range::main();

    println!();
    println!("8. Fetching Top Products using LinkedList, Iterators, and Loops");
    fetching_top_products::main();

    println!();
    println!("9. Efficient Storage and Retrieval of Words");
    storage_retrieval::main();

    println!();
    println!("10. Most Recently Used Product using HashMaps and Doubly Linked Lists");
    recently_used::main();

    println!();
    println!("11. Displaying Participants of an Online Meeting using BST and Stack");
    participants_online::main();
}
