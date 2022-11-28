use std::time::Duration;

use crate::component::executor::new_executor_and_spawner;
use crate::component::string_process_future::StringProcessFuture;
use crate::component::timer_future::TimerFuture;

mod component;

fn timer_demo() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}

fn string_process_demo() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        let res = StringProcessFuture::new(
            |s: &mut String| {
                s.push_str(" hello ");
            },
            |s: &str| -> bool { s.len() > 10 },
        )
        .await;
        println!("res: {}", res);
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}

fn main() {
    // println!("\ntimer_demo:");
    // timer_demo();

    println!("\nstring_process_demo:");
    string_process_demo()
}
