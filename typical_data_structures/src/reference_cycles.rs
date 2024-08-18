use std::{cell::RefCell, rc::Rc};

//--------------------------------------------------------------------------------------------------
//      Reference Cycles
//--------------------------------------------------------------------------------------------------
#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node, {:?}", self);

    }
}

pub fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    println!("a count: {:?}", Rc::strong_count(&a));

    let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a))}));
    println!("B is created.\na count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&b))}));
    println!("C is created.\na count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));
    println!("c count: {:?}", Rc::strong_count(&c));

    // Now let's create a reference cycle
    // c -> b -> a -> c

    (*a).borrow_mut().next = Some(Rc::clone(&c));   // This lines create cycle
    println!("After creating reference cycle.");
    println!("a count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));
    println!("c count: {:?}", Rc::strong_count(&c));

    // Let's trigger an overflow
    // println!("a: {:?}", a); // This will trigger a stack overflow

}