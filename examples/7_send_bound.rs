use std::rc::Rc;

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        tokio::task::yield_now().await;
    })
    .await;
}
