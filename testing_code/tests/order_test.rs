use testing_code::{Category, Customer, Order, Product};
mod helpers;
#[test]
fn test_total_bill_without_discount() {
    helpers::common_setup();
    let product = Product::new(1, "Harry Potter Book 1".to_string(), 100.0, Category::Books);

    let customer = Customer::new(1, "Bob".to_string(), "bob_mumbai@gmail.com".to_string());

    let order = Order::new(2, product, customer, 2);

    assert_eq!(format!("{:.2}", order.total_bill()), "220.00");
}

#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(1, "Harry Potter Book 1".to_string(), 100.0, Category::Books);

    let customer = Customer::new(1, "Bob".to_string(), "bob_mumbai@gmail.com".to_string());

    let order = Order::new(2, product, customer, 6);

    assert_eq!(format!("{:.2}", order.total_bill()), "594.00");
}