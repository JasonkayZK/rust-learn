# **Rust中Future执行底层探秘**

关于Future Trait底层逻辑，Rust圣经中讲的已经非常到位了：

-   https://course.rs/async-rust/async/future-excuting.html

但是其中的例子举的是一个Sleep定时器的例子，基本上只会执行一次 `poll`，并且没有内部的逻辑；

后面我会再写另外的一个例子，虽然也没什么意义，但是增加了一些逻辑；

<br/>

## **Rust圣经中例子的实现**

### **概述**

首先我们先来实现 Rust 圣经中计时器的例子；

对于一个 Async 系统而言，主要分为这么几个部分：

-   **执行器 Executor：本质上是一个 mpsc Channel 的 Receiver + Waker 的实现；**
-   **唤醒器 Waker：Future 通过调用 waker 中的 wake 函数通知 Executor 调用 poll 函数推动 Future 任务完成；**
-   **Future（不知道怎么翻译，应该是期货的意思）：包装 Task 以及 Waker，由 Executor 推动惰性执行；**
-   **任务 Task：实际异步执行的任务，被包装在 Future 中，任务完成后通过 Future 返回结果；**

在 Rust 圣经中，上面的内容是穿插着讲解的，大体上是自底向上从 Future 开始；

这里会从 Executor 调度开始，自顶向下执行；

但是阅读下文之前，也希望你先阅读：

-   https://course.rs/async-rust/async/future-excuting.html

对 Rust 中的 Future 以及异步有一定的了解；

<br/>

### **执行器 Executor**

#### **Executor概述**

Rust 的 `Future` 是惰性的，只有调度器调用了在 Future Trait 中定义的 `poll` 方法，才会真正被执行；

其中一个推动它的方式就是在 `async` 函数中使用 `.await` 来调用另一个 `async` 函数；

但是那些最外层的 `async` 函数，谁来推动它们运行呢？答案就是：执行器 `executor` ！

**任何一个异步系统，最终都需要一个统一的 Executor 来统一管理这些 Future 任务；**

例如，futures 库中提供的 `block_on`：

```rust
use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ 你存在我深深的脑海里~~",
        song.author, song.name,
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;

    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
```

或者 `tokio` 框架：

```rust
async fn say_world() {
    println!("world")
}

#[tokio::main]
async fn main() {
    let op = say_world();

    println!("hello");

    op.await;
}

// 展开宏之后
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
async fn say_world() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["world\n"], &[]));
    }
}
#[allow(dead_code)]
fn main() {
    let body = async {
        let op = say_world();
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["hello\n"], &[]));
        };
        op.await;
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
```

**其中 `tokio::runtime::Builder::new_multi_thread()` 就初始化了一个 Executor，只不过这个 Executor 是整个异步系统都在用，因此叫做 runtime 更为合适！**

<br/>

<font color="#f00">**Executor 会管理一批 `Future` (最外层的 `async` 函数)，然后通过不停地 `poll` 推动它们直到完成；**</font>

执行的逻辑如下：

<font color="#f00">**最开始，Executor 会先 `poll` 一次 `Future` ，后面就不会主动去 `poll` 了，而是等待 `Future` 通过调用 `wake` 函数来通知它可以继续，它才会继续去 `poll`（类似于 Event-Loop）；**</font>

<font color="#f00">**这种wake 通知然后 poll的方式会不断重复，直到 `Future` 完成；**</font>

<br/>

#### **构建Executor**

**在 Rust 圣经的例子中构建的 Executor 更像是一个类似于线程池的调度器，性能其实并不会特别高；**

在实际生产环境下，tokio 实现的是类似于 Go 中 goroutine 调度器的方式，将大量的 Future 映射到为数不多的操作系统中，并且通过提供可插拔的多个类型的 runtime 供你使用；

>   **你可理解为，Rust 中的 tokio 既可以通过类似于 Java 中线程池的方式使用，也可以通过类似于 Go 中协程调度器的方式使用；**
>
>   **至于原因，当然是因为 `There is no silver bullet`，具体场景使用具体的模式；例如：嵌入式场景和服务器场景相差甚远！**

##### **Executor实现**

Executor 需要从一个消息通道( `channel` )中拉取事件，然后运行；

当一个任务准备好后（可以继续执行），会将自己放入 Executor 的消息 Channel 中，然后等待执行器 `poll` ；

下面是 Executor 的定义和实现：

```rust
/// 任务执行器，负责从通道中接收任务然后执行
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub(crate) fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}
```

可以看到，Executor 的实现是一个包含 `Task` 的 `Receiver` 用于接收来自 `Spawner` 创建的各个 Task 消息；

在 Executor 的 run 方法中，通过 `while let` 接收通道中的消息；

随后通过 `take` 将 Task 的所有权取出到 future 变量中，针对 Task 创建一个 Waker，并传入 Context 中；

最后，调用 future 的 poll 函数，推动任务执行，并且如果执行 poll 后仍然是 Pending 状态，则再次将 future 放回 Executor 的队列中去；

从上面可以看到，从 Executor 的角度，只能通过 ready_queue 获取 Future 来调度执行；

而创建并包装 Future 的工作是交给 Spawner 做的，下面来看；

<br/>

##### **实现Spawner**

`Spawner`负责创建新的`Future`，然后将它发送到任务通道中；

```rust
/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("Channel已关闭");
    }
}
```

显然，Spawner 的内部是一个 `SyncSender`，负责将传入的 Future 进行包装，然后通过 Channel 传入 Executor 中调度；

这正是 spawn 方法的逻辑：

首先，将 future 转为 Pinned Box，随后使用 Arc 进行了封装，最后通过 send 将包装好的任务发给了 Executor 执行；

>   **将 future 转为 Pinned Box 的原因见：**
>
>   -   https://course.rs/async-rust/async/pin-unpin.html
>
>   简单来说，对于存在自引用的类型，要求其内部指向的内存地址不可变！
>
>   <font color="#f00">**注意：`!Unpin` Trait 只是一个 Marker，他帮助编译器在编译阶段对内部指向进行检查，而非真正固定了内存！**</font>

下面是 Executor 和 Spawner 的构造函数：

```rust
pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 任务通道允许的最大缓冲数(任务队列的最大长度)
    // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}
```

<br/>

##### **抽象Task**

Future 本质上就是一个 Task 任务的封装，下面来看：

```rust
/// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
    /// 进行中的Future，在未来的某个时间点会被完成
    ///
    /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
    /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
    /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
    ///
    /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}
```

**在 Task 中，包装了 future 和一个从 Spawner clone 来的 task_sender，有了这个 task_sender，我们就可以在 Waker 中将自己发送给 Executor 执行；**

在我们**为 Task 实现 ArcWake Trait 时就是这么做的；**

<br/>

### **唤醒器 Waker**

在异步场景下，Waker 是一个很重要的组件，因为我们不能让 Executor 不断的去轮询所有的 Future 来查看任务是否完成，而是应当在 Future 执行完成后主动的向 Executor 报告！

并且通常情况下，对于 `Future` 来说，第一次被 `poll` 时无法完成任务是很正常的；因此它需要确保在未来一旦准备好时，可以通知 Executor 再次对其进行 `poll`，进而继续往下执行，该通知就是通过 `Waker` 类型完成的；

下面是标准库中 Future Trait 和 Context 的定义：

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub struct Context<'a> {
    waker: &'a Waker,
    // Ensure we future-proof against variance changes by forcing
    // the lifetime to be invariant (argument-position lifetimes
    // are contravariant while return-position lifetimes are
    // covariant).
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
```

可以看到，poll 方法的参数 cx 中包含了 waker，这正是上面我们在实现 Executor 中放入的；

当 Future 任务完成后，会调用 waker 的 wake 方法，注意到在前面我们为 Task 实现了 ArcWake Trait；

因此调用 waker 的 wake 方法后，会执行 Task 对应的方法：

```rust
fn wake_by_ref(arc_self: &Arc<Self>) {
  // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
  let cloned = arc_self.clone();
  arc_self.task_sender.send(cloned).expect("任务队列已满");
}
```

最后再次将任务发送给了 Executor，由 Executor 调用 poll 推动 Future 修改状态！

<font color="#f00">**需要注意的是：waker 调用 wake 后会消耗掉自己：**</font>

```rust
pub fn wake(self) {
  // The actual wakeup call is delegated through a virtual function call
  // to the implementation which is defined by the executor.
  let wake = self.waker.vtable.wake;
  let data = self.waker.data;

  // Don't call `drop` -- the waker will be consumed by `wake`.
  crate::mem::forget(self);

  // SAFETY: This is safe because `Waker::from_raw` is the only way
  // to initialize `wake` and `data` requiring the user to acknowledge
  // that the contract of `RawWaker` is upheld.
  unsafe { (wake)(data) };
}
```

看到函数签名 `wake(self)`！

<br/>

### **具体任务封装 Future**

实际上，**编写 Future 的任务是由编译器来做的：我们编写的每个 `async` 块在编译结束后都返回一个 Future 对象；**

当然，这里为了学习，我们是手动创建的 Future：

```rust
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}
```

TimerFuture 的结构非常简单，仅仅维护了一个 `completed` 状态和一个waker：

-   **当睡眠结束后，可以修改 completed 的状态，标注任务结束；**
-   **同时，通过 waker 来通知 Executor 推进任务状态；**

 下面是具体的实现：

```rust
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置`waker`，这样新线程在睡眠(计时)结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作,
            //
            // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
            // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
            // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
```

可以看到，poll 中判断了是否是已完成：

-   如果已完成，标注当前 Future 已完成；
-   否则标记 Pending，并放回 waker（这是因为前面说到的<font color="#f00">**waker 调用 wake 后会消耗掉自己：**</font>）

>   **poll方法中只做具体 Future 状态的变化；**
>
>   **这个例子不好的一点在于，我们的 Future 中只有单个任务， 因此只有 Ready、Pending 状态；**
>
>   **如果存在多个顺序的协程，则会存在 Pending1、Pending2、…等等，多个状态切换；**
>
>   **还有一个不好的地方在于，他的任务只是简单的Sleep，甚至看不出来异步函数的逻辑究竟是在哪执行的！**

既然 poll 方法只是推动 Future 的状态，那具体的 async 函数的逻辑在哪里执行呢？

答案是在 Future 本身被定义：

```rust
impl TimerFuture {
    /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 创建新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            // 睡眠指定时间实现计时功能
            thread::sleep(duration);
            println!("sleep task completed!");
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
```

上面是 Future 的构造函数，可以看到首先初始化了 shared_state，随后在一个新的线程中执行了任务（暂且把 Sleep 看作是一个高耗时的IO任务）；

前面说到了，这里任务的提交是非常简单的，就是创建一个新的线程；

**实际上，任务的提交后的调度是非常复杂的，很可能需要任务窃取、任务切换等一系列处理；**

下面是 tokio 中 `spawn` 的实现，比我们这里复杂太多了：

```rust
pub(crate) fn spawn<F>(&self, future: F, id: Id) -> JoinHandle<F::Output>
where
F: Future + Send + 'static,
F::Output: Send + 'static,
{
  match self {
    Handle::CurrentThread(h) => current_thread::Handle::spawn(h, future, id),

    #[cfg(all(feature = "rt-multi-thread", not(tokio_wasi)))]
    Handle::MultiThread(h) => multi_thread::Handle::spawn(h, future, id),
  }
}

impl Handle {
    /// Spawns a future onto the thread pool
    pub(crate) fn spawn<F>(me: &Arc<Self>, future: F, id: task::Id) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::bind_new_task(me, future, id)
    }

    pub(crate) fn shutdown(&self) {
        self.close();
    }

    pub(super) fn bind_new_task<T>(me: &Arc<Self>, future: T, id: task::Id) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let (handle, notified) = me.shared.owned.bind(future, me.clone(), id);

        if let Some(notified) = notified {
            me.schedule_task(notified, false);
        }

        handle
    }
}

impl Handle {
    pub(super) fn schedule_task(&self, task: Notified, is_yield: bool) {
        CURRENT.with(|maybe_cx| {
            if let Some(cx) = maybe_cx {
                // Make sure the task is part of the **current** scheduler.
                if self.ptr_eq(&cx.worker.handle) {
                    // And the current thread still holds a core
                    if let Some(core) = cx.core.borrow_mut().as_mut() {
                        self.schedule_local(core, task, is_yield);
                        return;
                    }
                }
            }

            // Otherwise, use the inject queue.
            self.shared.inject.push(task);
            self.shared.scheduler_metrics.inc_remote_schedule_count();
            self.notify_parked();
        })
    }
}
...
```

<br/>

### **使用Future**

让我们把视线收回来，既然手动写完了 Future 让我们来测试一下：

```rust
fn timer_demo() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)),
    );

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    executor.run();
}

fn main() {
    println!("\ntimer_demo:");
    timer_demo()
}
```

首先创建了 executor 和 spawner，这和 tokio 中的初始化环境类似；

随后，通过 `spawner.spawn` 传入了一个 Future 任务：回忆我们之前的代码，这里会直接向 Channel 中发送一个消息，但是还没有调用 `executor.run`，因此消息不会被消费！

>   <font color="#f00">**这也是 Rust 中异步为是惰性的原因！**</font>

随后，我们释放 spawner，因为所有的消息都已经发送完毕了；

最后，调用 `executor.run()`，开始消费 spawner 发送的消息！

<br/>

## **一个添加字符串的例子**

前面的例子只是 Sleep，并没有什么实际意义，下面我们来写一个稍微有那么点意义的 Future；

```rust
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
        (self.complete_checker)(&self.s)
    }

    pub fn process(&mut self) {
        while !self.check_task() {
            (self.processor)(&mut self.s);
            println!("cur len: {}", self.s.len());
            thread::sleep(Duration::new(1, 0));
        }
    }
}

impl Future for StringProcessFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut task = self.task.lock().unwrap();
        if task.check_task() {
            Poll::Ready(task.s.clone())
        } else {
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
            // Do real "IO" cost task
            let mut shared_state = shared_task.lock().unwrap();
            shared_state.process();
            // 通知执行`poll`对应的`Future`
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        StringProcessFuture { task }
    }
}
```

StringProcessFuture 要求传入：

-   **构建一个字符串的方法：`processor: fn(&mut String)`；**
-   **以及一个校验字符串已经满足的条件：`complete_checker: fn(&str) -> bool`；**

在实际的任务中，我们依然使用 Sleep 模拟了 IO 操作，但是和之前的例子不同，我们将 Future 操作提取到了 `process` 方法中，使得逻辑更加清晰：

```rust
pub fn process(&mut self) {
  while !self.check_task() {
    (self.processor)(&mut self.s);
    println!("cur len: {}", self.s.len());
    thread::sleep(Duration::new(1, 0));
  }
}
```

异步处理的 Task：

```rust
thread::spawn(move || {
  // Do real "IO" cost task
  let mut shared_state = shared_task.lock().unwrap();
  shared_state.process();
  // 通知执行`poll`对应的`Future`
  if let Some(waker) = shared_state.waker.take() {
    waker.wake()
  }
});
```

测试一下我们的代码：

```rust
fn string_process_demo() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("before StringProcessFuture!");
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

    spawner.spawn(async {
        println!("before StringProcessFuture!");
        // 创建定时器Future，并等待它完成
        let res = StringProcessFuture::new(
            |s: &mut String| {
                s.push_str(" world ");
            },
            |s: &str| -> bool { s.len() > 20 },
        )
        .await;
        println!("res: {}", res);
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    executor.run();
}

fn main() {
    println!("\nstring_process_demo:");
    string_process_demo()
}
```

执行后输出如下：

```
string_process_demo:
before StringProcessFuture!
before StringProcessFuture!
cur len: 7
cur len: 7
cur len: 14
cur len: 14
cur len: 21
res:  hello  hello 
res:  world  world  world 
```

可以看到，两个 Future 是并行被调度的（因为我们开了两个线程）；

<br/>

# **附录**

文章参考：

-   https://course.rs/async-rust/async/future-excuting.html

<br/>