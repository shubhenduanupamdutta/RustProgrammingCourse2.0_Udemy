//----------------------------------------------------------------
//        Thread Parking
//----------------------------------------------------------------

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    let data = Arc::new(Mutex::new(5));
    let data_clone = data.clone();

    let thread_1 = thread::spawn(move || {
        println!("Inside thread 1: I am doing some work");
        thread::sleep(Duration::from_millis(20));
        println!("Inside thread 1: I am doing some more work");
        println!("Inside thread 1: First task finished.");

        // Parking the thread
        println!("Parking the thread, until thread 2 finishes its work.");
        // thread::park(); // This will park the thread indefinitely until it is unparked by another thread

        // Parking the thread for 5 seconds or until it is unparked by another thread
        thread::park_timeout(Duration::from_secs(5));

        // Printing value of shared data
        println!("Thread 1: Printing the updated shared data.");
        println!("Thread 1: Data: {:?}", *data.lock().unwrap());
    });

    let thread_2 = thread::spawn(move || {
        println!("Inside thread 2: I am doing some work");
        thread::sleep(Duration::from_secs(1));
        println!("Inside thread 2: Working on updating the shared data."); // This simulates that the thread is undergoing some heavy computation
        *data_clone.lock().unwrap() = 10;
        println!("Inside thread 2: Updated the shared data.");
    });

    thread_2.join().unwrap();
    println!("Thread 2 finished its work."); // That means shared data is updated
    thread_1.thread().unpark();
    thread_1.join().unwrap();
}
