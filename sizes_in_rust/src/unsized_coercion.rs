//----------------------------------------------
//      Unsized Coercion in Rust
//----------------------------------------------

use std::any::type_name;

fn str_slice_fn(s: &str) {
    println!("{}", s);
}

fn array_slice_fn<T>(_s: &[T]) {}

trait SomeTrait {
    fn method(&self);
}

impl<T> SomeTrait for [T] {
    fn method(&self) {}
    // This method can be called on different types due to deref and unsized coercion
}

pub fn main() {
    let some_string = "Calling with String".to_string();
    str_slice_fn(&some_string);

    let slice: &[i32] = &[1];
    let vec = vec![1];
    let array = [1, 2, 3];

    array_slice_fn(&vec); // Deref Coercion happens
    array_slice_fn(&array); // Unsized Coercion happens
    array_slice_fn(slice);

    slice.method();
    vec.method(); // Deref Coercion happens
    array.method(); // Unsized Coercion happens
}
