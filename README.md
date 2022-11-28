# **通过一个例子学习Cargo Features**

## **前言**

本文只是通过一个例子来讲解如何使用 Cargo Features，并非讲解 Cargo Features 的全部特性以及底层实现；

同时，在阅读本文之前，请先阅读：

-   [条件编译 Features](https://course.rs/cargo/reference/features/intro.html#条件编译-features)
-   [Conditional compilation](https://doc.rust-lang.org/stable/reference/conditional-compilation.html#conditional-compilation)

对 Cargo Features 有一定的基础，再来学习本文；

<br/>

## **实现一个含有feature的lib**

和上面 [条件编译 Features](https://course.rs/cargo/reference/features/intro.html#条件编译-features) 的内容保持一致，在例子中会定义这么几个 feature：

```toml
[features]
bmp = []
png = []
ico = ["bmp", "png"]
webp = []
```

bmp、png、ico、webp，并且 ico 依赖 bmp 和 png 特性；

创建 `feature-demo` crate：

```bash
cargo new feature-demo --lib
```

修改 Cargo.toml，增加 feature 定义：

feature-demo/Cargo.toml

```diff
+ [features]

+ default = []

+ full = [
+     "bmp",
+     "png",
+     "ico",
+     "webp"
+ ]

+ bmp = []
+ png = []
+ ico = ["bmp", "png"]
+ webp = []
```

同时实现各个 feature：

```rust
// feature-demo/src/lib.rs
pub mod bmp;
pub mod ico;
pub mod png;
pub mod webp;

// feature-demo/src/bmp.rs
#[cfg(feature = "bmp")]
pub fn process_bmp() {
    println!("Processing bmp");
}

// feature-demo/src/png.rs
#[cfg(feature = "png")]
pub fn process_png() {
    println!("Processing png");
}

// feature-demo/src/webp.rs
#[cfg(feature = "webp")]
pub fn process_webp() {
    println!("Processing webp");
}

// feature-demo/src/ico.rs
use crate::bmp::process_bmp;
use crate::png::process_png;

#[cfg(all(feature = "png", feature = "bmp"))]
#[cfg(feature = "ico")]
pub fn process_ico() {
    println!("Before process_ico: ");
    process_bmp();
    process_png();
    println!("Processing ico")
}
```

各个文件中的内容如上所示，其中 `#[cfg(...)]` 的意思是，只有定义了这个 feature 才编译下面的代码；

>   **关于 cfg：**
>
>   -   https://doc.rust-lang.org/stable/reference/conditional-compilation.html#the-cfg-attribute

同时，由于 ico 特性依赖 bmp、png 特性，因此需要先使用了 png、bmp 特性，同时使用了 ico 特性才会编译相应的代码；

至此，我们带有 feature 的 lib 编写完成；

<br/>

## **使用一个含有feature的lib**

下面来使用我们刚刚编写的含有 feature 的 lib；

首先引入这个 crate：

Cargo.toml

```toml
[dependencies]
feature-demo = { path = "./feature-demo", features = ["bmp", "ico"] }
```

**这里引入了 bmp 和 ico 特性，但是由于 ico 特性依赖了 png 特性，因此实际上我们也是可以使用 png 特性的！**

在 main 中使用这个 lib：

src/main.rs

```rust
use feature_demo::bmp::process_bmp;
use feature_demo::ico::process_ico;
use feature_demo::png::process_png;

fn main() {
    process_bmp();

    process_png();

    process_ico();
}
```

执行后输出：

```
Processing bmp
Processing png
Before process_ico:
Processing bmp
Processing png
Processing ico
```

但是，由于我们没有引入 webp 特性，因此这部分代码实际上不会被编译，并且我们也无法使用！

例如，添加下面的代码，会造成编译失败：

```diff
use feature_demo::bmp::process_bmp;
use feature_demo::ico::process_ico;
use feature_demo::png::process_png;
+ use feature_demo::webp::process_webp;

fn main() {
    process_bmp();

    process_png();

    process_ico();
+
+    process_webp();
}

```

在添加了相关的 feature 之后，编译成功：

```diff
- feature-demo = { path = "./feature-demo", features = ["bmp", "ico"] }
+ feature-demo = { path = "./feature-demo", features = ["bmp", "ico", "webp"] }
```

<br/>

# **附录**

文章参考：

-   https://course.rs/cargo/reference/features/intro.html

<br/>
