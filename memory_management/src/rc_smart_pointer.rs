// ----------------------------------------------------
//         Reference Counting (RC) Smart Pointer
// ----------------------------------------------------

use std::rc::Rc;


#[derive(Debug)]
enum List {
    Cons(i32, Option<Rc<List>>),
}

pub fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("a: {:?}", a);
    println!("Reference Count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a)));
        println!("b: {:?}", b);
        println!("Reference Count after b: {}", Rc::strong_count(&a));
        {
            let c = List::Cons(4, Some(Rc::clone(&a)));
            println!("c: {:?}", c);
            println!("Reference Count after c: {}", Rc::strong_count(&a));
        }
        println!("Reference Count after c goes out of scope: {}", Rc::strong_count(&a));
    }
    println!("Reference Count after b goes out of scope: {}", Rc::strong_count(&a));

    let List::Cons(q, r) = &*a;
    println!("q: {}", q);
    println!("r: {:?}", r);

    println!("a: {:?}", a);
}
