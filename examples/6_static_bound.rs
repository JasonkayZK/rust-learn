use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let _ = task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    })
    .await;
}
