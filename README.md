# **在Rust中处理整数溢出**

## **默认行为**

默认情况下，当出现整型溢出时，Debug 模式会发生 panic，Release 模式下会在溢出后取舍归零；

src/main.rs

```rust
#[allow(arithmetic_overflow)]
fn main() {
    let x: u8 = 255;
    println!("{}", x + 1);
}
```

Rust 的编译器是非常智能的，会在编译期就检测出上面的代码存在溢出；

此时可以通过添加：`#[allow(arithmetic_overflow)]` 来关闭该检查；

debug模式：

```shel
cargo run

thread 'main' panicked at 'attempt to add with overflow', src/main.rs:4:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

release模式：


```shel
cargo run --release

0
```

该差异的来源在于：`dev` 和 `release` 配置的定义不同（参见：[Profiles - The Cargo Book](https://doc.rust-lang.org/cargo/reference/profiles.html)）；

造成这一行为的选项是 `overflow-checks`，它在 `dev` 中开启，`release` 中关闭；

如果你的应用程序依赖于整数溢出行为，可以直接更改 `dev` 配置，以关闭 `overflow-checks`：

```toml
[profile.dev]
overflow-checks = false
```

<br/>

## **显式运算**

对于所有的有符号和无符号整数，Rust 提供了四组不同的运算函数，这提供了显式处理整数溢出的方式；

### **`wrapping_` 系列函数：**

examples/1_opt.rs

```rust
fn wrapping_demo() {
    println!("{}", (250_u8).wrapping_add(10));     // 4
    println!("{}", (120_i8).wrapping_add(10));     // -126
    println!("{}", (300_u16).wrapping_mul(800));   // 43392
    println!("{}", (-100_i8).wrapping_sub(100));   // 56
    println!("{}", (8000_i32).wrapping_pow(5000)); // 0
}
```

`wrapping_` 系列函数处理整数溢出的方法是回绕，即从整数类型的最大值回绕到最小值（也是我们期望发生的默认情况）；

这种方法确保了在使用这些函数时，无论构建配置文件如何，都不会造成意外的运行时错误；

<br/>

### **`overflowing_`系列函数：**

examples/1_opt.rs

```rust
fn overflowing_demo() {
    // 4, true
    let (result, overflowed) = (250_u8).overflowing_add(10);

    println!(
        "sum is {} where overflow {} occur",
        result,
        if overflowed { "did" } else { "did not" },
    );
}
```

这些函数等同于 `wrapping_`，除了返回值会多一个 `bool` 以指明是否有溢出产生；

例如：在实现模拟器时可能特别有用，因为许多 CPU 有一个标志，且必须在指令导致溢出时设置；

<br/>

### **`checked_`系列函数：**

examples/1_opt.rs

有时我们不想回绕值，而是将溢出作为一种特殊情况处理。可以通过 **`checked_`** 达到这一效果：

```rust
fn checked_demo() {
    match (100_u8).checked_add(200) {
        Some(result) => println!("{result}"),
        None => panic!("overflowed!"),
    }
}
```

<br/>

### **`saturating_`系列函数：**

examples/1_opt.rs

另一种选择是在**溢出时饱合（saturating）**，而非回绕（即到达最大值或最小时，保持该值）：

```rust
fn saturating_demo() {
    println!("{}", (-32768_i16).saturating_sub(10)); // -32768
    println!("{}", (200_u8).saturating_add(100));    // 255
}
```

<br/>

### **执行额外开销**

你可能会担心，每当想执行基本的运算时，多余的函数调用会减慢代码的执行速度；实际上 Rust 可以优化掉多余的函数调用；

我们可以通过使用 [`cargo-show-asm`](https://crates.io/crates/cargo-show-asm) 来查看某个函数编译后的汇编指令；

先看看普通的加法，和编译后的汇编：

```
pub fn addition(x: u8, y: u8) -> u8 {
    x + y
}

$ cargo asm overflow_example::addition --simplify

    Finished release [optimized] target(s) in 0.00s

overflow_example::addition:

    lea eax, [rsi + rdi]
    ret
```

编译的结果是单条 `lea` 指令。再来看看 `wrapping_add`：

```
pub fn addition(x: u8, y: u8) -> u8 {
    x.wrapping_add(y)
}

$ cargo asm overflow_example::addition --simplify

    Finished release [optimized] target(s) in 0.00s

overflow_example::addition:

    lea eax, [rsi + rdi]
    ret
```

正如我们期望的，编译的结果相同：`wrapping_add` 的调用已经被优化！

<br/>

## **包装类型**

在某些场景中，有许多地方都可能发生整数溢出，那么上述方法就会显得有些冗长，很多时候还容易忘记处理整数溢出；

Rust 也提供了 `Wrapping<T>` 包装类型，这种类型允许使用正常的算术操作符，同时确保在整数溢出时自动回绕！

例如：

examples/2_warpping.rs

```rust
use std::num::Wrapping;

fn main() {
    let mut x = Wrapping(125_u8);

    println!("{}", x + Wrapping(200)); // 69
    println!("{}", x - Wrapping(200)); // 181

    // 如果我们同时更改变量 x, 那么可以直接使用基本数据类型, 不用再套一层
    // x 现在为 113
    x *= 5;
    println!("{}", x);

    // 错误! 注意 - 我们只可以在有赋值操作时使用基本数据类型
    // (如在使用 += -= 等操作符时)
    // x / 5;
}
```

这比在每个运算时都使用 `wrapping_` 函数显得更清晰！

也有一个类似的 `Saturating<T>`，和 `Wrapping<T>` 类似，但在溢出时饱合而非回绕；

>   **`Saturating<T>` 已于 2023 年 11 月发布的 Rust 1.74.0 中稳定：[#115477](https://github.com/rust-lang/rust/pull/115477/)**
