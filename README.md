# **Rust反射之过程宏**

## **前言**

在Rust中，宏是一个非常大的话题，在这里我不打算以非常大的篇幅来讲述如何编写过程宏；

而是使用过程宏来进行类似于反射的AOP实现；

关于过程宏的开发，可以参考：

-   [如何编写一个过程宏(proc-macro)](https://dengjianping.github.io/2019/02/28/%E5%A6%82%E4%BD%95%E7%BC%96%E5%86%99%E4%B8%80%E4%B8%AA%E8%BF%87%E7%A8%8B%E5%AE%8F(proc-macro).html)
-   [Rust过程宏系列教程 | Proc Macro Workshop 之 Builder 实现](https://rustmagazine.github.io/rust_magazine_2021/chapter_5/proc_macro_workshop_guide_for_builder_project.html#rust过程宏系列教程--proc-macro-workshop-之-builder-实现)
-   https://github.com/dtolnay/proc-macro-workshop/
-   [Macro 宏编程](https://course.rs/advance/macro.html#macro-宏编程)

<br/>

## **使用过程宏实现AOP**

### **实现前说明**

使用过Spring框架的同学应该都用过AOP的特性吧，对Python熟悉的同学也应该对包装器不陌生；

在本文中会使用过程宏，实现和AOP类似的功能，而我们要实现的是计算函数的执行时间 `elapsed`；

实现逻辑其实非常简单，就是：

```rust
fn some_func() {
  use std::time;
  let start = time::Instant::now();
  
  // some logic...
  
  println!("time cost {:?}", start.elapsed());
}
```

即在函数执行前初始化当前时间，在执行结束后计算经过的时间即可；

在Spring框架中，我们可以动态的创建一个代理类，将方法的调用包装在这个类中，并在调用的前后插入相应的逻辑；

在 Rust 中，我们无法在运行时通过反射获取函数的定义，但是我们可以在编译器进行！

<br/>

### **实现`elapsed`过程宏**

#### **初始化项目**

首先创建一个 macro 的 lib 项目：

```bash
cargo new my-macro --lib
```

这一点非常重要：

<font color="#f00">**目前，当创建过程宏时，它的定义必须要放入一个独立的包中，且包的类型也是特殊的；**</font>

>   <font color="#f00">**事实上，根据[这个说法](https://www.reddit.com/r/rust/comments/t1oa1e/what_are_the_complex_technical_reasons_why/)，过程宏放入独立包的原因在于它必须先被编译后才能使用，如果过程宏和使用它的代码在一个包，就必须先单独对过程宏的代码进行编译，然后再对我们的代码进行编译，但悲剧的是 Rust 的编译单元是包，因此你无法做到这一点；**</font>

随后需要修改配置：

```toml
[lib]
proc-macro = true

[dependencies]
quote = "1"
syn = { version = "1.0.56", features = ["full"] }
```

在 stable 版本里，我们需要借助两个crate：

-   [**syn**](https://docs.rs/syn/1.0.1/syn/)：用来解析语法树(AST)、各种语法构成；
-   [**quote**](https://docs.rs/quote/1.0.0/quote/)：解析语法树，生成rust代码，从而实现你想要的新功能；

**同时，还需要在 `[lib]` 中将过程宏的开关开启 : `proc-macro = true`；**

<br/>

#### **实现`elapsed`逻辑**

目前过程宏必须在 crate root 下声明（lib.rs中），如果在非 root 下使用 `#[proc_macro_attribute]` 等进行标注则会报错：

```rust
functions tagged with `#[proc_macro_attribute]` must currently reside in the root of the crate
```

**而为了使具体逻辑和宏定义注册分离，我们可以在 crate root 中只做声明，而调用其他 mod 中具体逻辑的实现；**

修改 `lib.rs` 增加声明：

my-macro/src/lib.rs

```rust
use proc_macro::TokenStream;

mod elapsed;

/// A proc macro for calculating the elapsed time of the function
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn elapsed(args: TokenStream, func: TokenStream) -> TokenStream {
    elapsed::elapsed(args, func)
}
```

具体的实现在：`elapsed::elapsed` 中；

在 crate 的 src 目录下创建 `elapsed.rs`：

my-macro/src/elapsed.rs

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

pub(crate) fn elapsed(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis; // like pub
    let func_block = &func.block; // { some statement or expression here }

    let func_decl = func.sig;
    let func_name = &func_decl.ident; // function name
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let caller = quote! {
        // rebuild the function, add a func named is_expired to check user login session expire or not.
        #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
            use std::time;

            let start = time::Instant::now();
            #func_block
            println!("time cost {:?}", start.elapsed());
        }
    };

    caller.into()
}
```

我们通过 `pub(crate)` 指定了该函数仅在当前crate中可见，随后在 elapsed 函数中实现了我们的逻辑；

首先通过 `parse_macro_input!(func as ItemFn)` 将我们的 AST Token 转为函数定义 `func`；

随后获取了函数的各个部分：

-   **vis：可见性；**
-   **block：函数体；**
-   **func.sig：函数签名：**
    -   **ident：函数名；**
    -   **generics：函数声明的范型；**
    -   **inputs：函数入参；**
    -   **output：函数出参；**

随后，我们通过 `quote!` 创建了一块新的 rust 代码；

>   **关于：`quote!`：**
>
>   **`quote!` 中可以定义我们想要返回的 Rust 代码；**
>
>   **由于编译器需要的内容和 `quote!` 直接返回的不一样，因此还需要使用 `.into` 方法其转换为 `TokenStream`；**

在代码中，我们将函数声明重新拼好，同时在 `#func_block` 前后增加了我们的逻辑：

```rust
#func_vis fn #func_name #func_generics(#func_inputs) #func_output {
  use std::time;

  let start = time::Instant::now();
  #func_block
  println!("time cost {:?}", start.elapsed());
}
```

这样，我们的过程宏就已经开发完成了！

怎么样，是不是非常的暴力！

<font color="#f00">**[syn](https://docs.rs/syn/1.0.1/syn/) 和 [quote](https://docs.rs/quote/1.0.0/quote/) 库让我们有了操纵整个 Rust 代码 AST 的能力，使得在编译期我们无所不能，有无限的可能！**</font>

<br/>

#### **测试过程宏**

前面我们开发了一个过程宏，当然最后需要测试一下；

首先，引入我们的过程宏 crate：

Cargo.toml

```toml
[dependencies]
my-macro = { path = "./my-macro" }
```

随后，修改 `main.rs`，使用我们定义的宏：

src/main.rs

```rust
use my_macro::elapsed;
use std::thread;
use std::time::Duration;

#[elapsed]
fn demo(t: u64) {
    let secs = Duration::from_secs(t);
    thread::sleep(secs);
}

fn main() {
    demo(4);
    demo(2);
}
```

代码中，我们为函数 `demo` 增加了 `#[elapsed]` 过程宏声明；

因此，在编译时这个函数会被我们替换，我们可以通过 `cargo-expand` 来查看：

```rust
$ cargo expand       

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use my_macro::elapsed;
use std::thread;
use std::time::Duration;
fn demo(t: u64) {
    use std::time;
    let start = time::Instant::now();
    {
        let secs = Duration::from_secs(t);
        thread::sleep(secs);
    }
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["time cost ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&start.elapsed())],
            ),
        );
    };
}
fn main() {
    demo(4);
    demo(2);
}
```

可以看到，在 demo 中增加了我们定义的代码！

执行代码，结果如下：

```bash
$ cargo run   

time cost 4.00297825s
time cost 2.000378291s
```

<br/>

## **总结**

通过上面的例子可以看到，虽然我们不能在运行时对函数、结构体等定义进行解析，但是 Rust 为我们提供了更强大的方式：**直接在编译期操作代码的 AST，从而提供了无限的可能！**

**而 `#[elapsed]` 过程宏的使用方式也像极了 Java 中的注解 `@elapsed`！**

但是过程宏的整个实现方式都在编译期完成，没有任何运行时消耗！

<br/>

# **附录**

源代码：

-   https://github.com/JasonkayZK/rust-learn/tree/reflection

参考文章：

-   https://github.com/dtolnay/reflect
-   https://course.rs/advance/macro.html#%E7%B1%BB%E5%B1%9E%E6%80%A7%E5%AE%8Fattribute-like-macros
-   https://dengjianping.github.io/2019/02/28/%E5%A6%82%E4%BD%95%E7%BC%96%E5%86%99%E4%B8%80%E4%B8%AA%E8%BF%87%E7%A8%8B%E5%AE%8F(proc-macro).html

