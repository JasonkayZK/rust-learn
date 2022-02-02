use futures::executor::block_on;

async fn async_demo1() -> i32 {
    println!("55");
    55
}

async fn async_demo2() -> i32 {
    println!("44");
    44
}

async fn async_demo3() -> i32 {
    println!("1");
    1
}

async fn call_async() {
    let (r1, r2, r3) = futures::join!(async_demo1(), async_demo2(), async_demo3());

    println!("res: {:?}", r1 + r2 + r3)
}

fn main() {
    block_on(call_async());
}
