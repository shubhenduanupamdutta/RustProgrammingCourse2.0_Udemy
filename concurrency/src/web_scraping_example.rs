//----------------------------------------------------------------
//          Project: Web Scraping
//----------------------------------------------------------------
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use ureq::AgentBuilder;

#[tokio::main]
pub async fn main() -> Result<(), ureq::Error> {
    let webpages = vec![
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
    ];

    // Sending out simple get request to get the webpage

    // Create an agent to keep stat between requests
    // Building the agent with most of the default values
    let agent = AgentBuilder::new().build();

    println!("Getting webpages without threads or async/await...");
    // To compute the time taken to fetch webpages, we will use Instant
    let now = Instant::now();

    // Next, reading all the textual information from individual webpage inside a loop
    for webpages in webpages.clone() {
        let _web_body = agent.get(webpages).call()?.into_string()?; // this will block the thread
    }

    println!(
        "Time taken without threads or async/await: {:.2?}",
        now.elapsed()
    );
    // {:.2?} is used to print the time in seconds with 2 decimal points

    println!();
    println!("Getting webpages with threads...");
    let now = Instant::now(); // Resetting the time
    let agent = Arc::new(agent); // Making the agent Arc, to use in multiple threads
    let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new(); // Vector to store handles of threads

    // Looping through the webpages and creating a thread for each webpage
    for webpage in webpages.clone() {
        let agent_thread = agent.clone();
        let t = thread::spawn(move || {
            let _web_body = agent_thread.get(webpage).call()?.into_string()?;

            Ok(())
        });
        handles.push(t);
    }

    // Collecting the results from all the threads, calling join on each thread to make sure that each thread has finished
    for handle in handles {
        handle.join().unwrap()?;
    }

    println!("Time taken with threads: {:.2?}", now.elapsed());


    // Using async/await to fetch webpages
    println!();
    println!("Getting webpages with async/await...");
    let now = Instant::now(); // Resetting the time
    let agent = Arc::new(agent); // Making the agent Arc, to use in multiple threads

    // Creating a vector to store the futures
    let mut futures: Vec<tokio::task::JoinHandle<Result<(), ureq::Error>>> = Vec::new();

    // Looping through the webpages and creating a future for each webpage
    for webpage in webpages.clone() {
        let agent_future = agent.clone();
        let f = tokio::spawn(async move {
            let _web_body = agent_future.get(webpage).call()?.into_string()?;
            Ok(())
        });
        futures.push(f);
    }

    // Collecting the results from all the futures
    for future in futures {
        future.await.unwrap()?;
    }

    println!("Time taken with async/await: {:.2?}", now.elapsed());

    Ok(())
}
