//---------------------------------------------------------------------------------
//    - Pointers to Sized vs Unsized Types
//---------------------------------------------------------------------------------

use std::mem::size_of;

trait Shape {
    fn print(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

pub fn main() {
    println!();
    println!(
        "Size of a reference or sized type: {}",
        size_of::<&[i32; 3]>()
    );
    println!(
        "Size of a reference to an unsized type: {}",
        size_of::<&[i32]>()
    );

    println!();
    let num_1: &[i32; 3] = &[1, 2, 3];
    let num_2: &[i32] = &[1, 2, 3];

    let mut sum_1 = 0;
    for num in num_1 {
        sum_1 += num;
    }

    println!("Sum of the array num_1: {}", sum_1);

    let mut sum_2 = 0;
    for num in num_2 {
        sum_2 += num;
    }
    println!("Sum of the slice num_2: {}", sum_2);

    // Let's check the sizes of two struct and reference to trait object
    println!("************************************");
    let new_circle = Circle;
    let new_rectangle = Rectangle;
    new_circle.print();
    new_rectangle.print();
    println!("************************************");

    println!();
    println!("Size of reference to Circle: {}", size_of::<&Circle>());
    println!("Size of reference to Rectangle: {}", size_of::<&Rectangle>());
    println!("Size of a reference to a trait object: {}", size_of::<&dyn Shape>());
}
