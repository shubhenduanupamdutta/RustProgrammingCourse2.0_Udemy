//! # Online Business
//! This is a rust library for online store management.

mod customer; // This is to insure that the customer module in organized in the crate.
mod order;
mod product;

pub use customer::Customer;
pub use product::{Category, Product};
pub use order::Order;

