use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

pub struct StringProcessFuture {
    task: Arc<Mutex<StringProcessTask>>,
}

pub struct StringProcessTask {
    s: String,
    processor: fn(&mut String),
    complete_checker: fn(&str) -> bool,
    waker: Option<Waker>,
}

impl StringProcessTask {
    pub fn check_task(&self) -> bool {
        println!("cur len: {}", self.s.len());
        (self.complete_checker)(&self.s)
    }

    pub fn process(&mut self) {
        (self.processor)(&mut self.s)
    }
}

impl Future for StringProcessFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut task = self.task.lock().unwrap();
        thread::sleep(Duration::new(1, 0));
        task.process();
        if task.check_task() {
            Poll::Ready(task.s.clone())
        } else {
            // 设置`waker`，这样新线程可以唤醒当前的任务，接着再次对`Future`进行`poll`操作,
            // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
            // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
            // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            task.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl StringProcessFuture {
    pub fn new(processor: fn(&mut String), complete_checker: fn(&str) -> bool) -> Self {
        let task = Arc::new(Mutex::new(StringProcessTask {
            s: String::new(),
            processor,
            complete_checker,
            waker: None,
        }));

        // 创建新线程
        let shared_task = task.clone();
        thread::spawn(move || {
            let mut shared_state = shared_task.lock().unwrap();
            // 通知执行`poll`对应的`Future`
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        StringProcessFuture { task }
    }
}
