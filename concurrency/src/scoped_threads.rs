//----------------------------------------------------------------
//         Scoped Threads
//----------------------------------------------------------------

use std::thread;

pub fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Reading vec inside Thread 1: {:?}", vec);
        });
    });

    println!("Scope finished");
    println!("Main Thread: {:?}", vec);
    vec.push(6);
    println!("Pushed 6 into vec, mutating it");
    // Doesn't throw an error, because immutable vector inside the threads have clear scope defined, and their lifetime is guaranteed to be within the scope.
    println!("Main Thread: {:?}", vec);

}