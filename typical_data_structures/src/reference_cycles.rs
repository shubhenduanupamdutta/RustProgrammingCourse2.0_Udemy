use std::{cell::RefCell, rc::{Rc, Weak}};

//--------------------------------------------------------------------------------------------------
//      Reference Cycles
//--------------------------------------------------------------------------------------------------
#[derive(Debug)]
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node, {:?}", self);

    }
}

pub fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    println!("a strong count: {:?}", Rc::strong_count(&a));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    let b = Rc::new(RefCell::new(Node { next: Some(Rc::downgrade(&a))}));
    println!("B is created.\na strong count: {:?}", Rc::strong_count(&a));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    println!("b count: {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(Node { next: Some(Rc::downgrade(&b))}));
    println!("C is created.\na strong count: {:?}", Rc::strong_count(&a));
    println!("b strong count: {:?}", Rc::strong_count(&b));
    println!("c strong count: {:?}", Rc::strong_count(&c));
    println!("c weak count: {:?}", Rc::weak_count(&c));
    println!("b weak count: {:?}", Rc::weak_count(&b));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    // Now let's create a reference cycle
    // c -> b -> a -> c

    // Using weak reference to avoid reference cycle

    (*a).borrow_mut().next = Some(Rc::downgrade(&c));   // This lines create cycle
    println!("After creating weak reference cycle.");
    println!("a strong count: {:?}", Rc::strong_count(&a));
    println!("b strong count: {:?}", Rc::strong_count(&b));
    println!("c strong count: {:?}", Rc::strong_count(&c));
    println!("c weak count: {:?}", Rc::weak_count(&c));
    println!("b weak count: {:?}", Rc::weak_count(&b));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    // Now following will not cause stack overflow
    println!("a: {:?}", a); // This will trigger a stack overflow

}