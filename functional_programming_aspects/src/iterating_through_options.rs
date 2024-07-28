// -----------------------------------------------------------------------------
//             Iterating through Options in Rust
// -----------------------------------------------------------------------------;

pub fn main() {
    let some_product = Some("Laptop");
    let mut products = vec!["Smartphone", "Tablet", "Smartwatch", "Battery", "Charger"];

    // match some_product {
    //     Some(product) => products.push(product),
    //     _ => {},
    // }

    // if let Some(product) = some_product {
    //     products.push(product);
    // }

    products.extend(some_product);

    // println!("{:?}", products);

    let product_iter = products.iter().chain(some_product.iter());

    for product in product_iter {
        println!("Product: {}", product);
    }

    let all_products = vec![Some("Laptop"), None, Some("Smartphone"), Some("Tablet"), None, Some("Smartwatch"), Some("Battery"), Some("Charger")];

    println!("\nAll Products: {:?}", all_products);

    let valid_products = all_products
    .into_iter()
    .flatten()
    .collect::<Vec<&str>>();

    println!("\n Valid Products: {:?}", valid_products);
}
