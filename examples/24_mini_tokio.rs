extern crate core;

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use futures::task;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();

    for i in 0..10 {
        mini_tokio.spawn(async move {
            let when = Instant::now() + Duration::from_millis(i * 1000);
            let future = Delay { when };

            let out = future.await;
            assert_eq!(out, "done");
        });
        mini_tokio.run();
    }
}

struct MiniTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output=()> + Send>>;

impl MiniTokio {
    pub fn new() -> Self {
        Self { tasks: VecDeque::new() }
    }

    fn spawn<F>(&mut self, future: F)
        where F: Future<Output=()> + Send + 'static {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}
