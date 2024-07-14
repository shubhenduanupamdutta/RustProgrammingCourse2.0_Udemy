mod shapes;
pub mod sorting;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};

mod product {

    pub use category::Category;
    #[derive(Debug, PartialEq)]
    pub struct Product {
        pub id: u64,
        pub name: String,
        price: f64,
        category: Category,
    }

    impl Product {
        /// Constructor for product struct.
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
            Product {
                id,
                name,
                price,
                category,
            }
        }

        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }

    mod category {
        #[derive(Debug, PartialEq)]
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Self {
            Customer { id, name, email }
        }
    }
}

mod order {
    use crate::customer::Customer;
    use crate::product::Product;
    pub struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Self {
            Order {
                id,
                product,
                customer,
                quantity,
            }
        }

        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * (self.quantity as f64);
            total_before_discount - (total_before_discount * discount)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
