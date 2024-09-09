// ----------------------------------------------
//             Async/Await
// ----------------------------------------------

async fn printing() {
    println!("I am an async function.");
}

#[tokio::main]
pub async fn main() {
    println!("Using async/await in Rust.");
    let future = printing();

    println!("The future is not yet polled by runtime. Since async functions are lazy and only executed when awaited.");
    future.await;
}