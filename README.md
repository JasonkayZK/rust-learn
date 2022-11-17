# **Recover**

Rust 中可以通过在 `Cargo.toml` 中的 `profile` 中增加 panic 相应的配置来修改 panic 的行为；

例如：

-   **默认情况下 panic 时，进程会打印当前出错的位置，然后退出；**
-   **panic = "unwind"，允许抓取异常；**
-   **panic = "abort"，出错 panic 时，直接 SigAbort 退出进程；**

下面是配置：

```toml
[profile.dev]
panic = "unwind"

[profile.release]
panic = "abort"
```

测试的代码如下：

```rust
extern crate core;

use std::panic;

fn main() {
    let res = panic::catch_unwind(|| {
        println!("not panic");
    });
    assert!(res.is_ok());

    let res = panic::catch_unwind(|| {
        panic!("panicked!");
    });
    assert!(res.is_err());

    println!("End of main()");
}
```

此时，在 dev 模式下：

```bash
$ cargo run

not panic
thread 'main' panicked at 'panicked!', src\main.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
End of main()
```

程序打印：`End of main()`，并正常退出；

如果在 release 模式下：

```bash
$ cargo run --release
not panic
thread 'main' panicked at 'panicked!', src\main.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\release\rust-learn.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
```

则会直接退出执行，不会打印最后的输出！

<br/>

# **附录**

相关博文：

-   [Rust从panic中恢复](https://jasonkayzk.github.io/2022/11/17/Rust从panic中恢复/)

参考：

-   https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/aborting-on-panic.html
