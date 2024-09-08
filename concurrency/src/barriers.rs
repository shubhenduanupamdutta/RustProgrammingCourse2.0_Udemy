//----------------------------------------------------------------
//         Barriers
//----------------------------------------------------------------

use std::sync::{Arc, Barrier, Mutex};
use std::thread;

pub fn main() {
    let mut thread_handles = Vec::new();

    let tasks = Arc::new(Mutex::new(vec![]));

    let barrier = Arc::new(Barrier::new(5));

    for i in 0..5 {
        let tasks = Arc::clone(&tasks);
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            // Task 1
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, completed its part on Task 1."));

            // Barrier point for synchronization
            barrier.wait();
            
            // Task 2
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, completed its part on Task 2."));
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let task_lock = &*tasks.lock().unwrap();

    for contents in task_lock {
        println!("{}", contents);
    }
}
