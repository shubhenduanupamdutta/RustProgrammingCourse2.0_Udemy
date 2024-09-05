//----------------------------------------------------------------
//           Message Passing Through Channels
//----------------------------------------------------------------
use std::{sync::mpsc, thread, time::Duration};

pub fn main() {
    let (tx, rx) = mpsc::channel(); // tx is the transmitter and rx is the receiver
    thread::spawn(move || {
        let i = "Shubhendu".to_string();
        println!("Sending value {i} through the channel");
        tx.send(i).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received value: {}", received);

    println!();
    println!("======= Multiple Transmitters and Single Receiver =======");
    let (tx1, rx1) = mpsc::channel();

    for i in 0..10 {
        let tx_clone = tx1.clone();
        thread::spawn(move || {
            println!("Sending value: {}", i);
            tx_clone.send(i).unwrap();
        });
    }

    drop(tx1); // This is necessary to drop the transmitter so that the receiver can stop waiting for more values
               // let received_val = rx1.recv().unwrap();
               // println!("Received value: {}", received_val);

    // let received_val = rx1.recv().unwrap();
    // println!("Received value: {}", received_val);

    for received_val in rx1 {
        println!("Received value: {}", received_val);
    }

    println!();
    println!("======= Part 2 =======");

    another_example();
}

fn another_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let x = "Shubhendu".to_string();
        println!("Sending value: {}", x);
        // thread::sleep(Duration::from_secs(5));
        tx.send(x).unwrap();
    });

    // let received = rx.recv().unwrap();
    // println!("Received value: {}", received);

    let mut received_status = false;
    while !received_status {
        match rx.try_recv() {
            Ok(received) => {
                println!("Received value: {}", received);
                received_status = true;
            }
            Err(_) => {
                println!("No value received yet");
                println!("Doing some other work...");
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}
