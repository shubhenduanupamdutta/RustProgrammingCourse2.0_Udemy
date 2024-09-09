# Concurrency
---------------------------------------------------------
## Threads Basics
---------------------------------------------------------
- **Concurrency refers to when a section of code runs in parallel, this means if we have some sort of code segments, concurrency will allow these segments to run at the same time, i.e. concurrently.**
- In most current OS and executed programs, code is run in a process, and the OS manages multiple process at once.
- Within a Rust program, you can also have independent parts that run simultaneously.
- This feature that runs code independently is called threads.
- #### In summary `concurrency` means execution of different parts of code simultaneously. The mechanism by which it is made possible in Rust is called `threads`.

There are many use cases of threads, for instance, one part of your program may wait for some input, thereby unnecessarily blocking the remaining parts of your code.

In such cases, we can make program efficient by creating threads where the input output related code may be assigned to a separate thread, and the remaining code may run independently on another thread. This way the input output related code will not block the remaining code.

- To use thread, we need to import the `thread` module from the standard library.
- To create a thread, we need to use the `spawn` function, which takes a closure as an input.

```rust
use std::thread;

pub fn main() {
    println!("This will be printed.");
    println!("This will also be printed.");
    println!("The concurrency will start after this line.");
    thread::spawn(|| {
        println!("Hello 1 from the thread.");
        println!("Hello 2 from the thread.");
        println!("Hello 3 from the thread.");
        println!("Hello 4 from the thread.");
        println!("Hello 5 from the thread.");
        println!("Hello 6 from the thread.");
        println!("Hello 7 from the thread.");
    });

    println!("Hello 1 from the main thread.");
    println!("Hello 2 from the main thread.");   
}
```
- Print statements from the top (top 3) will run sequentially and not concurrently.
- This is because, we are creating thread after the top 3 print statements. Therefore concurrency starts after the top 3 print statements.
- Each program in Rust, by default, only has one thread, which is called the main thread.
- When we call `spawn` function, it creates a new thread,in addition to the main thread.
- After `thread::spawn(...)` is called, the rest of code, either belongs to newly created thread or original main thread, and they will run simultaneously.
- In the above example, 2 print statements in main thread will run concurrently with the 7 print statements in the newly created thread.
- There are two things to note about execution or scheduling order of the threads:
    - The execution is not deterministic, meaning that each time you run your program, it may execute in a different order.
    - Secondly, the scheduling order of threads is handled by operating system, and not by the Rust language itself. The operating system will divide the CPU time in small chunks and will run different threads in these chunks, turn by turn.
- The output of the above program will be something like this:
```shell
This will be printed.
This will also be printed.
The concurrency will start after this line.
Hello 1 from the main thread.
Hello 2 from the main thread.
Hello 1 from the thread.
Hello 2 from the thread.
Hello 3 from the thread.
Hello 4 from the thread.
```
```shell
Hello 1 from the main thread.
Hello 2 from the main thread.
Hello 1 from the thread.
Hello 2 from the thread.
Hello 3 from the thread.
Hello 4 from the thread.
Hello 5 from the thread.
Hello 6 from the thread.
Hello 7 from the thread.
```
```shell
Hello 1 from the main thread.
Hello 2 from the main thread.
Hello 1 from the thread.
Hello 2 from the thread.
Hello 3 from the thread.
```

- #### The output may vary each time you run the program. Above are outputs of 3 different runs.
- #### First 3 lines, which are before concurrency starts, are always printed first.
- #### There is no unique order in which the print statements inside the thread and main are executed.
- #### Each time I ran the program, the output was different. Because the operating system decides the order of execution of threads and it may vary each time you run the program.
- #### Lastly, in all the execution results, the print statements at the end of the main are always executed.
- #### However, some of the print statements in the other parallel are not executed, this is because main thread will always go to completion before the termination of the program.
- #### **Once the main thread completes, all spawn threads are shut down, whether or not they have finished running their code.**
- To allow a spawned thread to go to completion, two approaches can be used.
### Approach 1: Using `thread::sleep` function.
- **First Approach** is to use the `thread::sleep` function.
- Call to `thread::sleep` inside a thread forces that thread to stop its execution for a specified time, allowing other threads to run.
- In our case, we can add a call to `thread::sleep` in the main thread, so that the spawned thread can complete its execution.
```rust
use std::{thread, time::Duration};

pub fn main() {
    println!("This will be printed.");
    println!("This will also be printed.");
    println!("The concurrency will start after this line.");
    thread::spawn(|| {
        println!("Hello 1 from the thread.");
        println!("Hello 2 from the thread.");
        println!("Hello 3 from the thread.");
        println!("Hello 4 from the thread.");
        println!("Hello 5 from the thread.");
        println!("Hello 6 from the thread.");
        println!("Hello 7 from the thread.");
    });

    thread::sleep(Duration::from_millis(1000));
    println!("Hello 1 from the main thread.");
    println!("Hello 2 from the main thread.");   
}
```
- Output for multiple run after using `thread::sleep`:
```shell
This will be printed.
This will also be printed.
The concurrency will start after this line.
Hello 1 from the thread.
Hello 2 from the thread.
Hello 3 from the thread.
Hello 4 from the thread.
Hello 5 from the thread.
Hello 6 from the thread.
Hello 7 from the thread.
Hello 1 from the main thread.
Hello 2 from the main thread.
```
- When we run the program, the main thread will sleep for 1 ms, allowing the spawned thread to complete its execution of 7 print statements, since the main thread is sleeping for 1 ms, the spawned thread will complete its execution before the main thread wakes up.
- After delay of 1 ms, the main thread will wake up and print the last 2 print statements.
- #### The call to `thread::sleep` only ensures that the calling thread is blocked for certain amount of time. It does not guarantee that the spawned thread will complete its execution.
- #### In summary, this approach of using `thread::sleep` is not reliable, it doesn't guarantee that the spawned thread will always complete its execution.

### Approach 2: Using `join` method.
- **Second Approach** is to use the `join` method. It guarantees that the spawned thread will complete its execution before the main thread terminates.
- Each call to `thread::spawn` returns a `join` handle, which can be used to wait for the spawned thread to complete its execution.
- We can call the `join` method on the thread handle.
```rust
use std::{thread, time::Duration};

pub fn main() {
    println!("This will be printed.");
    println!("This will also be printed.");
    println!("The concurrency will start after this line.");
    let handle = thread::spawn(|| {
        println!("Hello 1 from the thread.");
        println!("Hello 2 from the thread.");
        println!("Hello 3 from the thread.");
        println!("Hello 4 from the thread.");
        println!("Hello 5 from the thread.");
        println!("Hello 6 from the thread.");
        println!("Hello 7 from the thread.");
    });

    // thread::sleep(Duration::from_millis(1000));

    handle.join();
    
    println!("Hello 1 from the main thread.");
    println!("Hello 2 from the main thread.");   
}
```
- A call to `join` method will block the execution of the thread in which it is called, until the thread for which it is called completes its execution.
- In our case, main thread will be blocked at `handle.join()` line, until the spawned thread completes its execution.
- After the thread completes its execution, the main thread will continue its execution.
```shell
This will be printed.
This will also be printed.
The concurrency will start after this line.
Hello 1 from the thread.
Hello 2 from the thread.
Hello 3 from the thread.
Hello 4 from the thread.
Hello 5 from the thread.
Hello 6 from the thread.
Hello 7 from the thread.
Hello 1 from the main thread.
Hello 2 from the main thread.
```
- Print statements in the spawned thread will complete first, because main is blocked before the print statements in the main thread.
- If we move, `handle.join()` to end of the main thread, the now the spawned and main thread will take turns in printing the statements.
- And the main thread will wait before exiting until the spawned thread completes its execution.
- #### The `join` method is a reliable way to ensure that the spawned thread completes its execution before the main thread terminates.
```rust

pub fn main() {
    println!("This will be printed.");
    println!("This will also be printed.");
    println!("The concurrency will start after this line.");
    let handle = thread::spawn(|| {
        println!("Hello 1 from the thread.");
        println!("Hello 2 from the thread.");
        println!("Hello 3 from the thread.");
        println!("Hello 4 from the thread.");
        println!("Hello 5 from the thread.");
        println!("Hello 6 from the thread.");
        println!("Hello 7 from the thread.");
    });

    // thread::sleep(Duration::from_millis(1000));

    // handle.join();
    
    println!("Hello 1 from the main thread.");
    println!("Hello 2 from the main thread.");

    handle.join().unwrap();
}
```
- Output:
```shell
This will be printed.
This will also be printed.
The concurrency will start after this line.
Hello 1 from the main thread.
Hello 2 from the main thread.
Hello 1 from the thread.
Hello 2 from the thread.
Hello 3 from the thread.
Hello 4 from the thread.
Hello 5 from the thread.
Hello 6 from the thread.
Hello 7 from the thread.
```
- In this case, the main thread will wait for the spawned thread to complete its execution before exiting.
- But the spawned thread outputs are always after main thread, because spawning threads has some overhead, during that type, main thread, which only has two print statements, completes its execution.
- If we add a sleep of one microsecond in the main thread, we will see main and spawned threads printing statements in turns, non-deterministically.
```rust
//----------------------------------------------------------------
//             Threads Basics
//----------------------------------------------------------------
use std::{thread, time::Duration};

pub fn main() {
    println!("This will be printed.");
    println!("This will also be printed.");
    println!("The concurrency will start after this line.");
    let handle = thread::spawn(|| {
        println!("Hello 1 from the thread.");
        println!("Hello 2 from the thread.");
        println!("Hello 3 from the thread.");
        println!("Hello 4 from the thread.");
        println!("Hello 5 from the thread.");
        println!("Hello 6 from the thread.");
        println!("Hello 7 from the thread.");
    });

    // thread::sleep(Duration::from_millis(1000));

    // handle.join();
    thread::sleep(Duration::from_micros(1));
    println!("Hello 1 from the main thread.");
    println!("Hello 2 from the main thread.");

    handle.join().unwrap();
}
```
- Output:
```shellHello 1 from the main thread.
Hello 1 from the thread.
Hello 2 from the main thread.
Hello 2 from the thread.
Hello 3 from the thread.
Hello 4 from the thread.
Hello 5 from the thread.
Hello 6 from the thread.
Hello 7 from the thread.
```
- `.join()` method returns a `Result`, `Ok` when there is no panics or errors, and `Err` when there is a panic or error.

=========================================================
### Difference between concurrency and parallelism
=========================================================
- **Concurrency** is about multiple tasks which start, run and complete in overlapping time periods, in no specific order.
- **Parallelism** is about doing multiple tasks that  run simultaneously, i.e. literally at the same time, on a hardware with multiple computing resources, like multiple CPUs or cores.
- From programming perspective, we are interested in **concurrency**, but in hardware perspective, we are interested in **parallelism**.
---------------------------------------------------------
## Ownership in Threads
---------------------------------------------------------
```rust
use std::thread;

pub fn main() {
    let x = "some string".to_string();

    thread::spawn(|| {
        println!("{}", x);
    });
}
```
- Spawning the thread, and accessing `x` inside the closure will give an error.
- Error - `closure may outlive the current function, but it borrows 'x', which is owned by the current function`
- `println!("{}", x);` is executed in a spawned thread. The closure is not updating `x`, so rust  will infer that inside the closure it will be used immutably. (Closure uses the variables in the environment with least possible permissions).
- We already know, that thread execution is non-deterministic and controlled by OS. Rust therefore, can't tell how long the spawned thread will live.
- Spawned thread, may live longer than the main thread, that will lead to a problem, because this closure borrows `x` immutably.
- At the end of the main thread, `x` will be dropped, but the spawned thread may still be running, and it will try to access `x`, which is already dropped. Which will lead to a dangling reference.
- To fix this, we can move `x` into the closure, so that the spawned thread takes ownership of `x`.
```rust
use std::thread;

pub fn main() {
    let x = "some string".to_string();

    thread::spawn(move || {
        println!("{}", x);
    });
}
```
- Now, the closure takes ownership of `x`, and the error is resolved.
- But since thread will take ownership to `x`, we can't use `x` after the `thread::spawn` call.
- If we try to use `x` after the `thread::spawn` call, we will get an error.
```rust
use std::thread;

pub fn main() {
    let x = "some string".to_string();

    thread::spawn(move || {
        println!("{}", x);
    });

    println!("{}", x);
}
```
- Error - `value used here after move`
- But if `x` value is a primitive, then this error will not occur, because primitive types implement `Copy` trait, and they are copied instead of moved.
- `move` keyword will not be required if the closure is implementing `FnOnce` trait, because `FnOnce` trait consumes the variables it captures by taking ownership of them.
```rust
use std::thread;

fn main() {
    let x = "some string".to_string();

    thread::spawn(|| {
        let y = x;
        println!("{}", y);
    });
}
```
- Above code will also compile, because, in this case closure is taking ownership of `x`, i.e. it is implementing `FnOnce` trait.
- #### In summary, threads in Rust are isolated from each other automatically due to ownership rules. This ensures that data races will never occur in Rust.
---------------------------------------------------------
## Message Passing Through Channels
---------------------------------------------------------
- In the previous part, we learned that rust ownership system isolates the threads from each other.
- **But sometimes, we needs some mechanism to communicate between threads to solve some complex problems.**
- #### There are two strategies to communicate between threads:
    - **Shared State Concurrency**
    - **Message Passing Concurrency**

- We will discuss the **Message Passing Concurrency** in this part.
- ### Rust achieves message passing using the concept of channels.
=========================================================
### Basics of Channels
=========================================================
- We will use an analogy of a water stream to understand the basics of channels. Assume with water stream we are talking about a river or a stream.
- If you put something like a boat on the stream, it will travel down the stream to end of the waterway.
- **A channel has two ends, a sender and a receiver. Also called as transmitter and receiver.**
- Going back to the river analogy, the transmitter is the upstream location where you would place the boat, and the receiver is the downstream location where the boat will end up and will be received.
- **Rust provides implementation of channels in its `std::sync::mpsc` module.**
- `mpsc` stands for multiple producer, single consumer.
- As the name suggests, this allows us to have multiple senders, but only one receiver.
- Let's create a channel,
```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```
- `mpsc::channel()` function returns a tuple, where the first element is the transmitter and the second element is the receiver.
- `tx` is the transmitter, and `rx` is the receiver. These are traditional names used in the literature.
- Now, one part of our case will call methods on the transmitter, passing in the data we want to send, and another part of our code is listening to the receiver for arriving messages.
- **Channels are closed when either the transmitter or receiver is dropped.**
- Let's create a thread, which will send a message to the main thread.
```rust
use std::{sync::mpsc, thread};

pub fn main() {
    let (tx, rx) = mpsc::channel(); // tx is the transmitter and rx is the receiver
    thread::spawn(move || {
        let i = "Shubhendu".to_string();
        println!("Sending value {i} through the channel");
        tx.send(i).unwrap();
    });
}
```
- In the above code, we are sending a string through the channel.
- First we created a string `i`, that we want to send through the channel.
- `.send()` method on the transmitter `tx` will allow us to send out a value.
- `move` is necessary because we need ownership of `tx` inside the closure, to send out messages through the channel.
- `.send()` return a `Result`, `Ok` when the message is sent successfully, and `Err` when there is an error, most likely when the receiver is dropped, and there is nowhere to send the message.
- We are using `unwrap()` to panic if there is an error, but in a real program, you should handle the error gracefully.
- Now, we need to receive the message in the main thread.
```rust
//<--------- **** snip **** -----------> original code
// Receiver part
let received_val = rx.recv().unwrap();
println!("Received value: {received_val}");
```
- `.recv()` called **receive** allows for receiving a message from the channel.
- This method `.recv()` blocks the thread which calls it, until a message is received.
- `.recv()` also returns a `Result`, `Ok` when a message is received, and `Err` when the transmitter is dropped before sending out a useful message.
- We are using `unwrap()` to panic if there is an error, but in a real program, you should handle the error gracefully.
- **Once a value is being sent out by the thread, it is no more available and its ownership is no more with the sender.**
- **There are no issues with stack data, since they are not moved but copied.**
- **We can have multiple threads sending out messages through the same channel.**
- Let's create multiple thread using a loop,
```rust
use std::{sync::mpsc, thread};

for i in 0..5 {
    thread::spawn(move || {
        let (tx, rx) = mpsc::channel();
        tx.send(i).unwrap();
    });
}
```
- This code will not compile, because we have moved `tx` into the closure, in the first iteration. And therefore it is not available in the next iteration, for the next thread.
- To make sure that the other threads are able to send out messages using the sender, we will pass on a clone of the sender, `tx` to each thread.
```rust
use std::{sync::mpsc, thread};
fn main() {
    let (tx1, rx1) = mpsc::channel();

    for i in 0..10 {
        let tx_clone = tx1.clone();
        thread::spawn(move || {
            println!("Sending value: {}", i);
            tx_clone.send(i).unwrap();
        });
    }

    let received_val = rx1.recv().unwrap();
    println!("Received value: {}", received_val);
}
```
- In the above code, we are cloning the transmitter `tx1` and passing the clone to each thread.
- Now, each thread has its own transmitter, and they can send out messages through the channel.
- **But when we run the above code, output is not as expected.**
```shell
Sending value: 0
Sending value: 1
Sending value: 2
Sending value: 3
Sending value: 4
Sending value: 5
Sending value: 6
Sending value: 7
Sending value: 8
Received value: 0
Sending value: 9
thread '<unnamed>' panicked at src\messaging_through_channels.rs:25:30:
called `Result::unwrap()` on an `Err` value: SendError { .. }
```
- This is because the call to `.recv()` only receives a single message, and then the main thread exits.
- **Let's try to receive second time.**
```rust
use std::{sync::mpsc, thread};
fn main() {
    let (tx1, rx1) = mpsc::channel();

    for i in 0..10 {
        let tx_clone = tx1.clone();
        thread::spawn(move || {
            println!("Sending value: {}", i);
            tx_clone.send(i).unwrap();
        });
    }

    let received_val = rx1.recv().unwrap();
    println!("Received value: {}", received_val);

    let received_val = rx1.recv().unwrap();
    println!("Received value: {}", received_val);
}
```
- Another thing to note is that the channels works like a `queue` which follows **first in first out (FIFO)** order.
- Sender or transmitter, may send messages in any order, but the receiver will always receive messages in the order they were sent.
- **To properly receive all the messages, we can treat receiver as an iterator.**
```rust
for received_val in rx1 {
    println!("Received value: {}", received_val);
}
```
- In this case, the main thread will be blocked until new messages become available.
- **When all the transmitters are dropped, and thus channel is closed, the receiver will return `None` and the loop will exit.**
- But in the above code, the output is,
```shell
Sending value: 0
Sending value: 1
Sending value: 2
Sending value: 3
Sending value: 4
Sending value: 5
Sending value: 6
Sending value: 7
Sending value: 8
Received value: 0
Received value: 1
Sending value: 9
Received value: 2
Received value: 3
Received value: 4
Received value: 5
Received value: 6
Received value: 7
Received value: 8
Received value: 9
```
- **And the program is not stopping, it continues to run.**
- Receiver end of the channel will stop listening for new messages, when all the transmitters are dropped.
- During each iteration, we are creating a clone of the transmitter, and moving it inside a thread, and then dropping it at the end of the iteration, when the thread completes its execution.
- However, when all the threads in `for` loop are completed, the original transmitter `tx1` still remains, because it is not consumed by any thread.
- To make sure that the original transmitter `tx1` is also dropped, we can drop it explicitly after the `for` loop.
```rust
use std::{sync::mpsc, thread};

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
}
```
- Now, the program will stop after all the messages are received.
- #### Unlike transmitter, we can't clone the receiver.
=========================================================
- Let's look at a code,
```rust
fn another_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let x = "Shubhendu".to_string();
        println!("Sending value: {}", x);
        tx.send(x).unwrap();
    });

    let received = rx.recv().unwrap();
}
```
- In the above code, we are creating a thread where we are sending out a value to main through a channel.
- Call to `.recv()` will block the main thread until a message is received.
- To clearly see this in action, we can add a dealy in spawned thread of 5 seconds.
```rust
fn another_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let x = "Shubhendu".to_string();
        println!("Sending value: {}", x);
        thread::sleep(Duration::from_secs(5));
        tx.send(x).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received value: {}", received);
}
```
- In the above code, the main thread will be blocked for 5 seconds, until the spawned thread sends out a message.
- We can see that last print statement is blocked till `rx.recv()` receives a message. 
- **This is not very efficient, specially in cases where the time may be substantial, and the remaining code is not dependent on the received message.**
- Rust provides a non-blocking way to receive messages, using the `try_recv` method.
- `try_recv` method will return immediately, with a `Result` value, `Ok` when a message is received, and `Err` when no message is available.
- We could write a loop that calls try, receive after some intervals, handles a message if one is available, and otherwise does other work for a little while before checking again.
```rust
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
```
- In this case, output was,
```shell
No value received yet
Doing some other work...
Sending value: Shubhendu
Received value: Shubhendu
```
- Now our code is able to do other stuff, until a message is received.
- #### You need to carefully inspect the code part that we want to execute, while waiting for the messages. It must not depend on the received messages from the thread, and in general doesn't depends on the result produced by the thread.
---------------------------------------------------------
## Shared State Concurrency
---------------------------------------------------------
- **Shared State Concurrency** is another way to communicate and share data between threads.
- Message passing is one way data flow, in which sender thread passes a message to a receiving thread, the ownership of the message is transferred from sender to receiver.
- **In shared thread concurrency, we have some piece of code residing inside the memory, that multiple threads can access by acquiring lock on the data.**
- **Rust provides a type called `Mutex` to achieve `lock` required for shared state concurrency.**
```rust
use std::sync::Mutex;
use std::thread;

pub fn main() {
    let m = Mutex::new(5);
}
```
- `Mutex` stands for mutual exclusion, and it is a type that provides a lock around the data it holds. That is, data wrapped by a mutex can only be accessed by one thread at a time.
- To gain access to a locking mechanism is used.
- In particular, **when a thread wants to gain access to the data behind a mutex, it will acquire a `lock` on the mutex. Once a `lock` is acquired, the thread has exclusive access to the data, and no other thread can access the data until the lock is released.**
- **Usually the lock will only be released once the thread is done with the data, allowing other threads to acquire the lock and access the data.**
```rust
use std::sync::Mutex;
use std::thread;

pub fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
}
```
- `lock()` method on the mutex will return a `Result`, `Ok` when the lock is acquired, and `Err` when there is an error, usually when the lock is already acquired by another thread and that thread panics.
- `lock()` method will block the thread until the lock is acquired.
- If there are multiple threads trying to call the lock, then only the first thread who makes a call to `lock()` will get the lock, and all other threads will be blocked until the lock is released.
- `Mutex` have a reputation for being hard to manage, each time you acquire a lock, you have to make sure that you unlock it explicitly in order to make it available for other threads.
- Fortunately, Rust type system, and ownership rules guarantees that you can't get locking and unlocking wrong.
- **When the lock goes out of scope, Rust will automatically release the lock.**
- **In the above code, lock will be automatically released when the `num` goes out of scope, or explicitly dropped.**
- For instance, after the code block, we can acquire the lock again, and modify the value.
```rust
use std::sync::Mutex;
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
    
    println!("Trying to acquire the lock in same main thread, without releasing the lock");
    let lock_m1 = m.lock().unwrap();
    println!("This code is blocked because the lock is held by previous Mutex guard {:?}", lock_m);
}
```
```shell
Acquiring the lock in a custom block
Acquiring lock in the main thread
m = 6
Trying to acquire the lock in same main thread, without releasing the lock
```
- And the program is still running, because the lock is not released, so the main thread, and therefore program, is blocked at `let lock_m1 = m.lock().unwrap();` line.
- **To make sure that program completes, we can explicitly drop the lock.**
```rust
use std::sync::Mutex;
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
    
    println!("Dropping the lock in the main thread");
    drop(lock_m);
    
    println!("Trying to acquire the lock in same main thread, after releasing the lock");
    let lock_m1 = m.lock().unwrap();
    println!("This code is not blocked because the lock is released {:?}", lock_m1);
}
```
- Output:
```shell
Acquiring the lock in a custom block
Acquiring lock in the main thread
m = 6
Dropping the lock in the main thread
Trying to acquire the lock in same main thread, after releasing the lock
This code is not blocked because the lock is released by previous Mutex guard 6
```
- A call to `drop(lock_m);` will release the lock, and the main thread will be able to acquire the lock again.
=========================================================
### Sharing Mutex between Threads
=========================================================
- Consider a files which needs to be accessed by multiple threads, for simplicity we will assume file contains `File` contains a single field of text, containing some textual data.
```rust
struct File {
    text: Vec<String>;
}

fn sharing_states() {
    let file = Mutex::new(File { text: vec![] });
}
```
- In the above code, we have a `File` struct, which contains a `text` field, which is a vector of strings.
- To make sure that only a single thread has access to the data at a time, we are wrapping the `File` struct in a `Mutex`.
- This will make sure that only a single thread can access the `File` struct at a time, and `text` field can be modified by only one thread at a time.
- Let's create few threads, where each thread will modify the `text` field of the `File` struct.
```rust
fn sharing_states() {
    let file = Mutex::new(File { text: vec![] });

    let mut thread_handles = Vec::new();

    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("Thread {} is trying to acquire the lock", i);
            let mut file = file.lock().unwrap();
            file.text.push(format!("Thread {} wrote this line", i));
        });
        thread_handles.push(handle);
    }
}
```
- We have created a vector `thread_handles` to store the handles of the threads, so that we can join them later.
- Each thread will try to acquire the lock, and then write a line to the `text` field of the `File` struct.
- But this code, as it is, will not compile, because we are trying to move `file` into the closure, and then trying to use it again in the next iteration.
- To fix this, we can clone the `file` and pass the clone to each thread.
- But since we want to modify the same file, we can't go with that approach.
- In this case, we want shared ownership of the `file`, for that we usually use `Rc` type, but `Rc` can't be send across threads safely, and we get compiler error.
- **In order to get thread safe shared ownership, we need to use the `Arc` smart pointer.**
- `Arc` stands for atomic reference counting, and it is a type that provides shared ownership of a value, and ensures that the value is not dropped until all the references to the value are dropped.
- `Arc` is thread safe, and can be sent across threads safely.
- Let's modify the code to use `Arc`.
```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct File {
    text: Vec<String>,
}

fn shared_states() {
    let file = Arc::new(Mutex::new(File { text: vec![] }));

    for i in 0..10 {
        let file = Arc::clone(&file);
        thread::spawn(move || {
            println!("Thread {} is trying to acquire the lock", i);
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Thread {} wrote this line", i));
        });
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let file_lock = file.lock().unwrap();

    for line in &file_lock.text {
        println!("{}", line);
    }
}
```
- `Arc` makes sure that reference count is updated atomically, and it is thread safe.
- **NOTE:** Initial `file` variable is not mutable, but we are able to modify the `text` field inside for loop, because `Mutex` uses interior mutability pattern.
---------------------------------------------------------
## Synchronization Through Barriers
---------------------------------------------------------
- `Barriers` enable multiple threads to synchronize the beginning of some computation.
- It is a point in code which halts the execution of calling threads until all the threads have executed the code up to that particular point.
- Consider a scenario, where we have some computationally expensive tasks. To efficiently complete the task, we divide the task into multiple threads.
- However, due to dependency among the tasks, the individual tasks has to be processed in sequential order, meaning task 2 can only commence once task 1 is completed.
- To simulate this, we will create a variable tasks, and wrap it around by `Arc` and `Mutex`.
```rust
use std::sync::{Arc, Barrier, Mutex};
use std::thread;

pub fn main() {
    let mut thread_handles = Vec::new();

    let tasks = Arc::new(Mutex::new(vec![]));

    for i in 0..5 {
        let tasks = Arc::clone(&tasks);
        let handle = thread::spawn(move || {
            // Task 1
            tasks.lock().unwrap().push(
                format!("Thread {i}, completed its part on Task 1.")
            );

            // Task 2 
            tasks.lock().unwrap().push({
                format!("Thread {i}, completed its part on Task 2.")
            });
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
```
- **Task 2** can only be done, when all the threads have completed **Task 1**.
- Here we have created a vector `tasks`, which is a empty vector, wrapped by `Mutex`, which is then wrapped by `Arc`.
- This will allow us to share the `tasks` vector among multiple threads.
- During each loop, we will first clone the `tasks` vector (with shared ownership), and then push a string to the vector.
- In **Task 1** we are pushing a string to the vector, and in **Task 2** we are pushing another string to the vector.
- We can call `lock()` method on the `Mutex` to acquire the lock, which gives us `MutexGuard`. We can access the data inside the `MutexGuard` using `*` operator, but we can't take ownership, so we are using `&*` to access the data. First dereference `MutexGuard` to get the data, and then take a reference to the data, to not take ownership.
- Let's run the code as it is.
```shell
Thread 0, completed its part on Task 1.
Thread 0, completed its part on Task 2.
Thread 1, completed its part on Task 1.
Thread 1, completed its part on Task 2.
Thread 2, completed its part on Task 1.
Thread 2, completed its part on Task 2.
Thread 3, completed its part on Task 1.
Thread 3, completed its part on Task 2.
Thread 4, completed its part on Task 1.
Thread 4, completed its part on Task 2.
```
- We can see that all the threads are completing both the tasks, and there is no synchronization between the tasks.
- Note that, we need to make sure that all the threads have completed **Task 1**, before starting **Task 2**.
- `Barriers` can be used to achieve this synchronization.
- To create a barrier, we will use `Barrier` type from `std::sync` module.
- `Barrier` can be called with a number, which is the number of threads that need to reach the barrier, before the barrier is released.
- Since `Barrier` will be used in multiple threads, we will wrap it around `Arc`.
- Let's modify the code to use `Barrier`.
```rust
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
```
- In the above code, we have created a barrier with 5 threads, and we are cloning the barrier for each thread.
- We are calling `wait()` method on the barrier, which will block the thread until all the threads have reached the barrier.
- Let's run the code.
```shell
Thread 0, completed its part on Task 1.
Thread 1, completed its part on Task 1.
Thread 2, completed its part on Task 1.
Thread 3, completed its part on Task 1.
Thread 4, completed its part on Task 1.
Thread 4, completed its part on Task 2.
Thread 0, completed its part on Task 2.
Thread 3, completed its part on Task 2.
Thread 1, completed its part on Task 2.
Thread 2, completed its part on Task 2.
```
- We can see that all the threads are completing **Task 1** before starting **Task 2**.
---------------------------------------------------------
## Scoped Threads
---------------------------------------------------------
```rust
use std::thread;

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(|| {
        println!("{:?}", vec);
    });
}
```
- The above code will not compile, because rust can't tell how long the spawned thread will live, and therefore it can't guarantee that the `vec` will be available when the spawned thread tries to access it.
- Usually to fix this, we can move the `vec` into the closure, so that the spawned thread takes ownership of `vec`.
- But in some cases, we may want to borrow the `vec` immutably, and we don't want to move the `vec` into the closure.
- Because moving prevents further use of vector in subsequent code.
- `Scoped Threads` was introduced in Rust `1.63.0` to solve this problem.
- It enhances the ability of threads to borrow data from the parent thread.
- In particular, it provides clearer control over lifetime of borrowed variables for compiler, which can't simply be cleared using `.join()`.
- To use scoped threads, we use `thread::scope` function, which takes a closure, and executes the closure in a new thread.
```rust
use std::thread;

pub fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Thread 1: {:?}", vec);
        });
    });
}
```
- In the above code, we are using `thread::scope` function, which takes a closure. Input to the closure, here `scope` can be any name and used to spawn threads.
- `scope.spawn()` function is used to spawn a thread, and the closure passed to it will be executed in the spawned thread.
- All the thread spawned inside the `scope` will be automatically joined when the `scope` goes out of scope.
- And for compiler, this gives a clear understanding of the lifetime of the borrowed variables.
- We can't violate the borrowing rules inside the scope, and we only have to consider the borrowing rules inside the scope.
- **Scoped threads are useful when we want to borrow data from the parent thread, and we don't want to move the data into the closure.**
---------------------------------------------------------
## Thread Parking
---------------------------------------------------------
- Let's look at a code,
```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    let thread_1 = thread::spawn(|| {});
    let thread_2 = thread::spawn(|| {});
}
```
- In the above code, we are creating two threads, `thread_1` and `thread_2`, and both right now are doing nothing.
- We are interesting in adding following functionality,
    - `thread_1` should do some work, then read in some shared data, which is being updated by `thread_2`.
    - `thread_2` will also do some work and then update the shared data.
    - `thread_1`, however, should only reads in shared data when it is updated.
- To code this, we will first create a shared data source using `Mutex` wrapped by an `Arc`.
```rust
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

    thread_1.join().unwrap();
    thread_2.join().unwrap();
}
```
- In the above code, we are creating a shared data source using `Arc` and `Mutex`.
- We are cloning the `Arc` to pass it to `thread_2`, because we can't move the `Arc` into the closure, as it will be moved into the closure, and we can't use it in the next thread.
- We are updating the shared data in `thread_2`, and reading the shared data in `thread_1`.
- But the output is not as expected.
```shell
Inside thread 1: I am doing some work
Inside thread 2: I am doing some work
Inside thread 1: I am doing some more work
Inside thread 1: First task finished.
Thread 1: Printing the updated shared data.
Thread 1: Data: 5
Inside thread 2: Working on updating the shared data.
Inside thread 2: Updated the shared data.
```
- We wanted `thread_1` to wait until `thread_2` updates the shared data, but it is not happening.
- There can be different solutions to this problem
    - **Possible Solution 1:** We can use `thread::sleep` in `thread_1` to wait for `thread_2` to update the shared data. In our case we can use `thread::sleep(Duration::from_secs(3));` to wait for 3 second, and then read the shared data. However its is not ideal, because in this, we will need to know how much time `thread_2` will take to update the shared data, which can be a variable quantity, and we can't predict it. If we overestimate, we will waste CPU cycles, and if we underestimate, we will get wrong results.
    - **Possible Solution 2:** We can use `thread::park` and `thread::unpark` functions to solve this problem. A call to `thread::park` will block that thread, until a call to `thread::unpark` is made by some other thread. We can use this to make `thread_1` wait until `thread_2` updates the shared data.
```rust
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
        thread::park();

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
```
- In the above code, we are using `thread::park()` to block the `thread_1`.
- In the main thread, we first call `thread_2.join().unwrap();` to make sure that `thread_2` has finished its work, and updated the shared data.
- Then we call `thread_1.thread().unpark();` to unpark the `thread_1`, and then call `thread_1.join().unwrap();` to make sure that `thread_1` has finished its work.
- Output:
```shell
Inside thread 1: I am doing some work
Inside thread 2: I am doing some work
Inside thread 1: I am doing some more work
Inside thread 1: First task finished.
Parking the thread, until thread 2 finishes its work.
Inside thread 2: Working on updating the shared data.
Inside thread 2: Updated the shared data.
Thread 2 finished its work.
Thread 1: Printing the updated shared data.
Thread 1: Data: 10
```
- We can see that `thread_1` is blocked until `thread_2` updates the shared data. And we get the expected output.
- **`thread::park()` and `thread::unpark()` are useful when we want to block a thread until some condition is met.**
- There is also similar, `thread::park_timeout` function, which will block the thread for a specified duration, and then unpark the thread. It will also unpark the thread if the thread is unparked by some other thread.
- **It is important to know that, there is a clear distinction between `thread::park_timeout()` and `thread::sleep()`.**
    - `thread::sleep()` unconditionaly blocks the thread for a specified duration, and then wakes up the thread.
    - `thread::park_timeout()` blocks the thread for a specified duration, but it can be woken up by some other thread, before the specified duration.