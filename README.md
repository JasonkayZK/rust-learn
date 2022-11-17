# **为Cargo编译的可执行文件增加commit版本号**

## **build.rs简要说明**

一些项目希望编译第三方的非 Rust 代码，例如 C 依赖库；一些希望链接本地或者基于源码构建的 C 依赖库；还有一些项目需要功能性的工具，例如在构建之前执行一些代码生成的工作等；

此时，可以使用项目根目录下的 `build.rs` 创建构建脚本；

例如：

build.rs

```rust
fn main() {
    // 以下代码告诉 Cargo ，一旦指定的文件 `src/hello.c` 发生了改变，就重新运行当前的构建脚本
    println!("cargo:rerun-if-changed=src/hello.c");
    // 使用 `cc` 来构建一个 C 文件，然后进行静态链接
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```

关于构建脚本的一些使用场景如下：

-   构建 C 依赖库；
-   在操作系统中寻找指定的 C 依赖库；
-   根据某个说明描述文件生成一个 Rust 模块；
-   执行一些平台相关的配置等等；

>   更详细见：
>
>   -   https://course.rs/cargo/reference/build-script/intro.html

<br/>

## **增加Commit版本号**

对于在可执行文件中增加commit版本号，我们可以：

-   首先，在 build.rs 中读取 Cargo.toml 里定义好的版本号，并执行 git 命令获取最新的 CommitId 写入到编译目录中的一个文件；

-   随后在源代码里中定义一个字符串常量，读取上面文件中的内容即可！

例如：

build.rs

```rust
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn get_git_version() -> String {
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    let child = Command::new("git").args(["describe", "--always"]).output();
    match child {
        Ok(child) => {
            let buf = String::from_utf8(child.stdout).expect("failed to read stdout");
            version + "-" + &buf
        }
        Err(err) => {
            eprintln!("`git describe` err: {}", err);
            version
        }
    }
}

fn main() {
    let version = get_git_version();
    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("VERSION")).unwrap();
    f.write_all(version.trim().as_bytes()).unwrap();
}
```

上面通过 `get_git_version` 函数获取到 version 字符串，并最后写入到 `${OUT_DIR}/VERSION` 文件中；

在  `get_git_version` 函数中：

-   首先，通过 `CARGO_PKG_VERSION` 环境变量获取到了 Cargo.toml 中定义的版本；
-   然后，调用 `git describe --always` 命令读取当前的 commitId；当然，如果获取失败则直接使用 Cargo 中的 version；

<br/>

## **在二进制中读取版本号**

上面在编译之前生成了版本号，因此我们可以在代码中读取到这个记录版本号的文件；

例如：

src/main.rs

```rust
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

fn main() {
    println!("Hello, world on build version: {}", VERSION);
}
```

上面的 `VERSION` 常量字符串, 就包含了 `$OUT_DIR/VERSION` 文件中的内容；

其中：

-   **`include_str!()` 宏用于读取 UTF-8 编码的文本文件，默认路径相对于当前源文件；**
-   **`concat!()` 宏用于合并字符串；**
-   **`env!()` 宏用于展开编译时的环境变量；**

最终测试结果：

```bash
$ cargo run    

Hello, world on build version: 0.1.0-9968d66
```

<br/>

# **附录**

文章参考：

-   https://blog.biofan.org/2019/08/cargo-build-script/
-   https://course.rs/cargo/reference/build-script/intro.html

