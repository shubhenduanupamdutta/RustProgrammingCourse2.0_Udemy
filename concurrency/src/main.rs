//--------------------------------------------------------------------
//           Concurrency
//--------------------------------------------------------------------

use concurrency::{async_await, barriers, basics_practice, messaging_through_channels, ownership_in_threads, scoped_threads, sharing_states, thread_parking, threads_basics, tokio_tasks, web_scraping_example};


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

    println!();
    println!("############ Scoped Threads ############");
    scoped_threads::main();

    println!();
    println!("############ Thread Parking ############");
    thread_parking::main();

    println!();
    println!("############ Async/Await ############");
    async_await::main();

    println!();
    println!("############ Tokio Tasks ############");
    tokio_tasks::main();

    println!();
    println!("############ Web Scraping Example ############");
    web_scraping_example::main().unwrap();
}
