# **Cargo命令及其扩展**

## **Cargo概述**

具体Cargo相关内容的学习，可以通过下面的网站学习：

-   https://doc.rust-lang.org/cargo/index.html
-   https://course.rs/cargo/intro.html

在阅读本文之前，强烈建议你先完成上面的学习或至少了解Rust中Cargo的基本使用；

Cargo不仅支持内建(built-in)的子命令，比如最常用的：

-   **cargo new**
-   **cargo build**
-   **cargo run**
-   **cargo clippy**

同时，Cargo 也支持第三方的插件工具，下面罗列了些比较常用的cargo plugin：

-   **cargo-expand：**在编译时展开标准宏和继承宏(derive，过程宏的一种)展开信息，便于调试宏；
-   **cargo-update：**更新你所安装的cargo插件；
-   **cargo-graph：**生成工程的依赖关系图；
-   **cargo-deb：**把当前工程编译成deb包，Debian/Ubuntu；
-   …

这些插件都可以通过如下命令来安装到本地，比如：

```bash
$ cargo install cargo-expand
```

官方列出了一些常用的第三方插件：

-   https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands

<br/>

## **Cargo常用插件(Plugins)**

这里推荐一些常用的 Cargo 插件；

### **cargo-update**

更新 `Cargo.lock` 中定义的依赖包的版本；

<br/>

### **cargo-vendor**

和 go 中的 vendor 模式一样，将项目的依赖包都缓存到本项目的 `vendor` 目录下；

>   这个特性对于 `CI/CD` 很有用处，不再需要每次自动编译时重新从 crates.io 下载源代码；

使用方法也很简单：只需要在根目录运行 `cargo vendor` 命令即可；

然后该命令会提醒我们修改本项目的 `cargo config` 文件：

```bash
$ mkdir .cargo
$ echo > .cargo/config << EOF
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF
```

但是，如果项目中依赖的包比较多，最终 `vendor/` 可能会比较大；

<br/>

### **cargo-edit**

这个插件提供了三个命令：

-   **cargo add：**向 `Cargo.toml` 加入新的包；
-   **cargo rm**：从 `Cargo.toml` 中移除包；
-   **cargo upgrade：**升级 `Cargo.toml` 中的包到最新版，同时会修改 `Cargo.toml` 中定义的依赖包的版本号；与之相对应的 `cargo update` 命令只会更新 `Cargo.lock` 这个文件中定义的版本；

<br/>

### **cargo-outdated**

用于展示工作区中有哪些依赖包是过期的；

```bash
$ cargo outdated
Name                            Project  Compat  Latest  Kind    Platform
----                            -------  ------  ------  ----    --------
miniz_oxide_c_api->miniz_oxide  0.2.2    0.2.3   0.2.3   Normal  ---
```

<br/>

### **cargo-script**

直接以脚本的方式运行 rust 程序，安装：

```bash
$ cargo install cargo-script
```

比如，`time.rs` 文件的内容是：

```rust
use std::time::SystemTime;

fn main() {
  println!("now: {:?}"，SystemTime::now());
}
```

现在执行这个文件：

```bash
$ cargo script time
```

<br/>

### **cargo-watch**

用于监控源码目录的更改，并运行指定的 cargo 命令；

比如，代码更改后自动重新编译并运行服务器代码：

```bash
$ cargo watch -x 'run --bin http_server'
```

该命令可以方便地与 vim/emacs 以及其它 IDE 集成；

<br/>

### **cargo-audit**

audit 工具可以检测 rust 项目中的依赖包是否有已知安全漏洞；

安装：

```bash
$ cargo install cargo-audit
```

然后在 rust 项目根目录运行一下：

```bash
$ cargo audit
```

即可扫描项目中的包依赖安全问题，并生成报告；

<br/>

### **cargo-tree**

`tree` 命令可以树形输入 crate 依赖关系；

安装：

```bash
$ cargo install cargo-tree
```

然后在项目根目录运行：

```bash
$ cargo tree                 
cargo-date v0.1.0 (/Users/kylinkzhang/self-workspace/rust-learn)
└── chrono v0.4.23
    ├── iana-time-zone v0.1.53
    │   └── core-foundation-sys v0.8.3
    ├── num-integer v0.1.45
    │   └── num-traits v0.2.15
    │       [build-dependencies]
    │       └── autocfg v1.1.0
    │   [build-dependencies]
    │   └── autocfg v1.1.0
    ├── num-traits v0.2.15 (*)
    └── time v0.1.44
        └── libc v0.2.137
```

<br/>

### **cargo-count**

`count` 命令用于统计 rust 源代码行数，安装：

```bash
$ cargo install cargo-count
```

查看一下当前项目：

```bash
$ cargo count --all
Gathering information...
         Language  Files  Lines  Blanks  Comments  Code
         --------  -----  -----  ------  --------  ----
         TOML      1      14     2       1         11
         D         4      30     6       0         24
         Rust      29     10994  1104    956       8934
         Python    2      79     24      8         47
         --------  -----  -----  ------  --------  ----
Totals:            36     11117  1136    965       9016
```

<br/>

### **cargo-fix**

常用的代码自动修正工具，基于 Rust 编译器以及像 clippy 这样的工具的提示.

```bash
$ cargo install cargo-fix
```

<br/>

### **其他插件**

还有一些其他插件：

-   **cargo-asm：**展示 rust 函数生成的汇编代码；
-   **cargo-bloat：**计算 rust 各部分代码在可执行文件中占的空间大小；
-   **cargo-sweep：**清除没用的编译结果文件；
-   **cargo-geiger：**检测所使用crate中的 unsafe 代码；

<br/>

## **开发一个Cargo插件**

上面分享了一些第三方 Cargo 插件，那么我们如何编写自己的 Cargo 插件呢？

官方文档中是有说明的：

-   https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands

当我们输入：

```bash
cargo ${command}
```

**实际上Cargo 会在 `$PATH` 下面寻找 `cargo-${command}` 可执行文件；**

**因此，开发一个 Cargo 插件非常简单，我们只需要将该可执行文件命名为 `cargo-${command}` 即可！**

下面来看一个例子，`cargo-date` 用于打印当前的系统时间；

首先，使用 Cargo 创建一个项目：

```bash
$ cargo new cargo-date
```

并增加配置：

Cargo.toml

```diff
[package]
name = "cargo-date"
version = "0.1.0"
edition = "2018"

+ [[bin]]
+ name = "cargo-date"
+ path = "src/main.rs"

+ [dependencies]
+ chrono = "0.4"
```

随后修改 main.rs 增加实现：

src/main.rs

```bash
use chrono::Local;

fn main() {
    let date = Local::now();
    println!("{}"，date.format("[%Y-%m-%d] [%H:%M:%S]"));
}
```

最后，编译并安装：

```bash
$ cargo install --path .
```

默认安装在：`~/.cargo/bin` 目录下：

```bash
$ ll ~/.cargo/bin 
total 270224
drwxr-xr-x  19 kylinkzhang  staff   608B 11 23 10:54 .
-rwxr-xr-x   1 kylinkzhang  staff   596K 11 23 10:54 cargo-date
-rwxr-xr-x   1 kylinkzhang  staff   4.1M 11 23 10:22 cargo-install-update
-rwxr-xr-x   1 kylinkzhang  staff   1.4M 11 23 10:22 cargo-install-update-config
-rwxr-xr-x   1 kylinkzhang  staff   7.0M  8 14 14:31 cargo-expand
...
```

之后，我们就可以执行这个插件了：

```bash
$ cargo date
[2022-11-23] [11:52:09]
```

<br/>

# **附录**

文章参考：

-   https://doc.rust-lang.org/cargo/index.html
-   https://course.rs/cargo/intro.html
-   https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands
-   https://blog.biofan.org/2019/07/cargo-commands/
-   https://dengjianping.github.io/2019/02/22/%E5%A6%82%E4%BD%95%E4%B8%BAcargo%E5%86%99%E4%B8%80%E4%B8%AAplugin.html
