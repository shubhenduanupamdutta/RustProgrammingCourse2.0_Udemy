//----------------------------------------------------------------
//         Product Popularity
//            - Description
//                 - Given some products along with its respectively popularity scores,
//                   we want to determine if the popularity is fluctuating, increasing or decreasing.
//            - Tools
//                 - HashMap, Loops, Conditional if
//----------------------------------------------------------------

use std::collections::HashMap;

fn popularity_analysis(scores: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for i in 0..scores.len() - 1 {
        if scores[i] > scores[i + 1] {
            increasing = false;
        } else if scores[i] < scores[i + 1] {
            decreasing = false;
        }
    }

    return increasing || decreasing;
}


pub fn main() {
    let mut products = HashMap::new();

    products.insert("product_1", vec![1, 2, 2, 3]);
    products.insert("product_2", vec![4, 5, 6, 3, 4]);
    products.insert("product_3", vec![8, 8, 7, 6, 5, 4, 4, 1]);

    for (product_id, popularity) in products {
        if popularity_analysis(popularity) {
            println!("{} popularity is increasing or decreasing", product_id);
        } else {
            println!("{} popularity is fluctuating", product_id);
        }
    }
}