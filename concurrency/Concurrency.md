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
