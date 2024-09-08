//----------------------------------------------------------------
//          Sharing States
//----------------------------------------------------------------

use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    let m = Mutex::new(5);

    {
        println!("Acquiring the lock in a custom block");
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("Acquiring lock in the main thread");
    let lock_m = m.lock().unwrap();
    println!("m = {:?}", lock_m);

    // println!("Trying to acquire the lock in same main thread, without releasing the lock");
    // let lock_m1 = m.lock().unwrap();
    // println!("This code is blocked because the lock is held by previous Mutex guard {:?}", lock_m);

    println!("Dropping the lock in the main thread");
    drop(lock_m);

    println!("Trying to acquire the lock in same main thread, after releasing the lock");
    let lock_m1 = m.lock().unwrap();
    println!(
        "This code is not blocked because the lock is released by previous Mutex guard {:?}",
        lock_m1
    );

    println!();
    println!("=======================================");
    println!("Using Mutex to share states between threads");
    println!("=======================================");
    sharing_states();
}

struct File {
    text: Vec<String>,
}

fn sharing_states() {
    let file = Arc::new(Mutex::new(File { text: vec![] }));

    let mut thread_handles = Vec::new();

    for i in 0..10 {
        let file = Arc::clone(&file);
        let handle = thread::spawn(move || {
            println!("Thread {} is trying to acquire the lock", i);
            let mut file = file.lock().unwrap();
            file.text.push(format!("Thread {} wrote this line", i));
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let file_lock = file.lock().unwrap();
    
    for t in &file_lock.text {
        println!("{}", t);
    }
}
