//----------------------------------------------------------------
//             Tokio Tasks
//----------------------------------------------------------------
use tokio::time::sleep;
use std::time::Duration;

async fn printing(_num: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("I am an async function, executing in task {}", _num);
}

// #[tokio::main(flavor = "current_thread")]
#[tokio::main]
pub async fn main() {

    let mut handles = Vec::new();

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {i}, printing first time");
            printing(i).await;
            println!("Task {i}, printing second time");
            printing(i).await;
            println!("Task {i} finished");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("All tasks finished");
}