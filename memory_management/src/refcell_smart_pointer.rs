//--------------------------------------------------------------------------------------------------
//          Refcell Smart Pointer
//--------------------------------------------------------------------------------------------------
use std::{cell::RefCell, rc::Rc};

pub fn main() {
    // ###### Following code will throw an error at compile time ######
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;

    // println!("x1: {}, x2: {}", x1, x2, x3);
    // ###############################################################

    // ###### Following code will not throw an error at compile time ######

    let a = RefCell::new(10);
    {
        let b = a.borrow(); // Immutable borrow
        let c = a.borrow();

        println!("b: {}, c: {}", b, c);
    }

    let d = a.borrow_mut(); // Mutable borrow
    println!("a: {:?}", a);
    println!("d: {}", d);

    // ####################################################################
    let a = RefCell::new(10);
    println!("a: {:?}", a);
    let mut b = a.borrow_mut();
    *b += 10;
    drop(b);
    println!("a: {:?}", a);

    println!("\n###### Combining RefCell with Rc Smart Pointer ######\n");
    let a = Rc::new(RefCell::new(String::from("C++")));

    let b = Rc::clone(&a);

    *b.borrow_mut() = String::from("Rust");
    println!("a: {:?}", a);
}
