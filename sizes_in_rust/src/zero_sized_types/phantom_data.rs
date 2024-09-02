//----------------------------------------------------------------
//          Phantom Data
//----------------------------------------------------------------

use std::{marker::PhantomData, rc::Rc};

struct A;

struct ABC {
    _ensuring_no_send_sync: Rc<()>,
}

struct ABC2 {
    _ensuring_no_send_sync: PhantomData<Rc<()>>,
}

pub fn main() {
    println!("Size of Unit Struct A: {}", std::mem::size_of::<A>());

    println!("Size of Struct ABC, removing send and sync trait: {}", std::mem::size_of::<ABC>());

    println!("Size of Struct ABC2, using phantom data and Rc to remove send and sync: {}", std::mem::size_of::<ABC2>());
}