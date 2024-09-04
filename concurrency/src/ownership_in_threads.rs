//--------------------------------------------------------------------------------------------------
//          Ownership in Threads
//--------------------------------------------------------------------------------------------------

use std::thread;

pub fn main() {
    let x = "some string".to_string();

    let _t = thread::spawn(move || {
        println!("{}", x);
    });
    _t.join().unwrap();
    // println!("{}", x); // This will give an error because x has been moved to the spawned thread

    let y = 5;

    thread::spawn(move || {
        println!("{}", y);
    });

    println!("{}", y);  // This will work because y is an i32, which implements the Copy trait

    // Implementing FnOnce trait closure will not require `move` keyword
    let z = "another string".to_string();
    let _t = thread::spawn(|| {
        let u = z;
        println!("{}", u);
    });
    _t.join().unwrap();
}