use std::time::Duration;

#[tokio::main]
async fn main() {
    loop {
        // This will not result in async_op running at all.
        // This is because .await is never called.
        // async_op();

        // If the snippet is updated to use .await,
        // then the loop waits for the operation to complete before starting over.
        // Will not repeat until `async_op` completes
        async_op().await;
    }
}

async fn async_op() {
    println!("hello world");
    tokio::time::sleep(Duration::from_secs(3)).await;
}
