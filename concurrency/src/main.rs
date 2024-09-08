//--------------------------------------------------------------------
//           Concurrency
//--------------------------------------------------------------------

use concurrency::{barriers, basics_practice, messaging_through_channels, ownership_in_threads, sharing_states, threads_basics};


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

    println!();
    println!("############ Sharing States ############");
    sharing_states::main();

    println!();
    println!("############ Barriers ############");
    barriers::main();
}
