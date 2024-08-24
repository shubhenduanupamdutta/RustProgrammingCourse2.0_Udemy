//---------------------------------------------
//        - Sized Trait
//        - Optionally Sized Trait
//---------------------------------------------

use negative_impl::negative_impl;

#[derive(Debug, Default)]
struct ABC;

#[negative_impl]
impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}

fn optionally_sized<T: ?Sized>(x: &T) -> &T {
    x
}

pub fn main() {
    let x: i32 = Default::default();
    println!("Checking default value of i32: {}", x);

    println!();
    let abc = ABC;
    println!("Checking Send and Sync for ABC: {:?}", abc);

    println!();
    let arr = [1, 2, 3, 4, 5];
    let _result = optionally_sized(&arr);

}