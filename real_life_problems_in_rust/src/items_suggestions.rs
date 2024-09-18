//--------------------------------------------------------------------
//        Suggesting Items for Special Shopping Card
//             - Description
//                - Given a list of prices, return a couple of items with their sum matching the target price.
//             - Tools
//                - HashSets, Vectors
//--------------------------------------------------------------------

use std::collections::HashSet;

pub fn main() {
    let product = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];
    println!(
        "Product pairs which amount to 50 are: {:?}",
        product_suggestions(product, 50)
    );
}

fn product_suggestions(product_prices: Vec<i32>, amount: i32) -> Vec<Vec<i32>> {
    let mut prices_hash = HashSet::new();
    let mut offers = Vec::new();

    for price in product_prices {
        let diff = amount - price;
        if !prices_hash.contains(&diff) {
            prices_hash.insert(price);
        } else {
            offers.push(vec![price, diff]);
        }
    }

    offers
}
