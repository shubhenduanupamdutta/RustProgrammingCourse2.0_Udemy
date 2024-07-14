pub use category::Category;

/// This is a struct for storing product related information and details.
#[derive(Debug, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    price: f64,
    category: Category,
}

mod category;

impl Product {
    /// Constructor for product struct.
    /// # Example
    /// ```
    /// use code_organization::{product::Product, product::category::Category};
    /// let harry_potter_1 = Product::new( 1, "Harry Potter and the Philosopher's Stone".to_string(), 879.0, Category::Books);
    /// assert_eq!(harry_potter_1.id, 1);
    /// assert_eq!(harry_potter_1.name, "Harry Potter and the Philosopher's Stone".to_string());
    /// ```
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
