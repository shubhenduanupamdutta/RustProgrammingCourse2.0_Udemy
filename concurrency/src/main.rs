//--------------------------------------------------------------------
//           Concurrency
//--------------------------------------------------------------------

use concurrency::{basics_practice, messaging_through_channels, ownership_in_threads, threads_basics};


fn main() {
    println!("########### Concurrency ###########");
    println!();
    println!("############ Basics of Threads ############");
    threads_basics::main();
    println!();
    println!("############ Practice of Threads Basics ############");
    basics_practice::main();

    println!();
    println!("############ Ownership in Threads ############");
    ownership_in_threads::main();

    println!();
    println!("############ Message Passing Through Channels ############");
    messaging_through_channels::main();
}
