// This package is code_organization
// ----------------------------------------------------
//                Privacy In Modules
// ----------------------------------------------------
use array_tool::vec::*;  // This is an external dependency
use code_organization::{Category, Customer, Order, Product};
fn main() {
    let product = Product::new(1, "Laptop".to_string(), 70_000.00, Category::Electronics);

    let customer = Customer::new(1, "John Doe".to_string(), "johndoe@gmail.com".to_string());

    let order = Order::new(1, product, customer, 10);

    println!("Total cost of the order is: ${}", order.total_bill());

    // Following code is code along for Using External Dependencies
    println!("### Using External Dependencies ###");
    let product1 = Product::new(1, "Laptop".to_string(), 70_000.00, Category::Electronics);
    let product2 = Product::new(2, "Mouse".to_string(), 500.00, Category::Electronics);
    let product3 = Product::new(3, "T-Shirt".to_string(), 1_000.00, Category::Clothing);
    let product4 = Product::new(
        4,
        "Harry Potter and Philosopher's Stone".to_string(),
        1_500.00,
        Category::Books,
    );

    let set1 = vec![&product1, &product2, &product3];
    let set2: Vec<&Product> = vec![&product2, &product4];

    let intersection = set1.intersect(set2);

    // {:#?} is used to print the debug information in a pretty format
    println!("Intersection of set1 and set2: {:#?}", intersection);
}
