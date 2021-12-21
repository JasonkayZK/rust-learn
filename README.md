# **Rust Learn**

<a href="https://github.com/JasonkayZK/rust-learn/actions/workflows/ci.yaml">
  <img src="https://github.com/JasonkayZK/rust-learn/actions/workflows/ci.yaml/badge.svg"/>
</a>

A repo to learn rust.

This main branch is a standard template for new rust project!

<br/>

## **Learning Resource**

Learn Rust with book：

- [《Rust 程序设计语言（第二版） 简体中文版》](https://www.bookstack.cn/books/trpl-zh-cn)；

Gitbook Url：

- https://www.gitbook.com/book/kaisery/trpl-zh-cn/details

<br/>

## **Jupyter**

**The Jupyter branch is shown below (Which helps you run Rust as script!)：**

- https://github.com/JasonkayZK/rust-learn/tree/jupyter

Which depend on jupyter kernel：

- https://github.com/google/evcxr/tree/main/evcxr_jupyter

<br/>

## **Create Project**

Use Cargo to create a project:

```bash
cargo new hello_rust --bin
```

build:

```bash
cd hello_rust && cargo build --release
```

run:

```bash
./target/release/hello_rust
Hello, world!
```

> Or just run program with:
>
>   ```bash
>   cargo run
>   ```

<br/>

## **Now Finished**

| Project                                                                                                            | Date                                      | Note                                                                                                                                                                                                               |
|--------------------------------------------------------------------------------------------------------------------|-------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [chapter2-guessing-game](https://github.com/JasonkayZK/rust-learn/tree/chapter2-guessing-game)                     | 2021-06-01                                | A guessing game                                                                                                                                                                                                    |
| [chapter3-variables](https://github.com/JasonkayZK/rust-learn/tree/chapter3-variables)                             | 2021-06-02                                | Variable & Type in Rust                                                                                                                                                                                            |
| [multiple-main-demo](https://github.com/JasonkayZK/rust-learn/tree/multiple-main-demo)                             | 2021-06-02                                | A demo to show how to run multiple main in Rust                                                                                                                                                                    |
| [chapter4-function](https://github.com/JasonkayZK/rust-learn/tree/chapter4-function)                               | 2021-06-02                                | Function in Rust                                                                                                                                                                                                   |
| [chapter5-control-flow](https://github.com/JasonkayZK/rust-learn/tree/chapter5-control-flow)                       | 2021-06-02                                | Control flow(if/loop/while/for) in Rust                                                                                                                                                                            |
| [chapter6-ownership](https://github.com/JasonkayZK/rust-learn/tree/chapter6-ownership)                             | 2021-06-03                                | Ownership(also string/slice) in Rust                                                                                                                                                                               |
| [chapter7-struct](https://github.com/JasonkayZK/rust-learn/tree/chapter7-struct)                                   | 2021-06-04                                | Struct in Rust                                                                                                                                                                                                     |
| [chapter8-enum-and-match](https://github.com/JasonkayZK/rust-learn/tree/chapter8-enum-and-match)                   | 2021-06-07                                | Enum & Match in Rust                                                                                                                                                                                               |
| [chapter9-modules](https://github.com/JasonkayZK/rust-learn/tree/chapter9-modules)                                 | 2021-06-07                                | Modules(mod/pub/use/super) in Rust                                                                                                                                                                                 |
| [chapter10-collections](https://github.com/JasonkayZK/rust-learn/tree/chapter10-collections)                       | 2021-06-09                                | Vector & String & Map in Rust                                                                                                                                                                                      |
| [chapter11-error-handling](https://github.com/JasonkayZK/rust-learn/tree/chapter11-error-handling)                 | 2021-06-09                                | Error handling in Rust (Panic! & Result)                                                                                                                                                                           |
| [chapter12-generic-trait-lifetime](https://github.com/JasonkayZK/rust-learn/tree/chapter12-generic-trait-lifetime) | 2021-06-10                                | Generic & Trait & Lifetime in Rust                                                                                                                                                                                 |
| [chapter13-testing](https://github.com/JasonkayZK/rust-learn/tree/chapter13-testing)                               | 2021-06-12                                | Testing(Write, Run & Organize) in Rust                                                                                                                                                                             |
| [chapter14-io-project-grep](https://github.com/JasonkayZK/rust-learn/tree/chapter14-io-project-grep)               | 2021-06-13                                | A io project: `mini-grep` written in rust.                                                                                                                                                                         |
| [chapter15-functional-features](https://github.com/JasonkayZK/rust-learn/tree/chapter15-functional-features)       | 2021-06-14                                | Functional features(Closure & Iterator) in rust.                                                                                                                                                                   |
| [chapter16-cargo](https://github.com/JasonkayZK/rust-learn/tree/chapter16-cargo)                                   | 2021-06-15                                | Cargo(Config, Publish, Install & Extend) & Workspace in rust.                                                                                                                                                      |
| [chapter17-smart-pointer](https://github.com/JasonkayZK/rust-learn/tree/chapter17-smart-pointer)                   | 2021-09-29                                | Smart Pointer in Rust(Within double-linked-list accomplishment).                                                                                                                                                   |
| [chapter18-concurrency](https://github.com/JasonkayZK/rust-learn/tree/chapter18-concurrency)                       | 2021-10-03                                | Concurrency in Rust.                                                                                                                                                                                               |
| [chapter19-oop](https://github.com/JasonkayZK/rust-learn/tree/chapter19-oop)                                       | 2021-11-14                                | Object-Oriented-Programming in Rust.                                                                                                                                                                               |
| [chapter20-match-patterns](https://github.com/JasonkayZK/rust-learn/tree/chapter20-match-patterns)                 | 2021-11-14                                | The Match Patterns in Rust.                                                                                                                                                                                        |
| [chapter21-advanced-features](https://github.com/JasonkayZK/rust-learn/tree/chapter21-advanced-features)           | 2021-11-14                                | The advanced features in Rust:<br />Unsafe、Lifetime、Trait、Type、Function & Closure                                                                                                                                  |
| [actix-web-demo](https://github.com/JasonkayZK/rust-learn/tree/actix-web-demo)                                     | 2021-10-04                                | RESTful API accomplished by [actix-web](https://github.com/actix/actix-web) framework.                                                                                                                             |
| [rbatis-demo](https://github.com/JasonkayZK/rust-learn/tree/rbatis-demo)                                           | 2021-10-07                                | A demo to show how to use ORM framework: [rbatis](https://github.com/rbatis/rbatis)                                                                                                                                |
| [wasm-hello](https://github.com/JasonkayZK/rust-learn/tree/wasm-hello)                                             | 2021-10-09                                | A simple rust-wasm demo.[Use template: [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template)]<br />Linked Blog: [Rust实现WebAssembly初窥](https://jasonkayzk.github.io/2021/10/10/Rust实现WebAssembly初窥/) |
| [feature-phantom](https://github.com/JasonkayZK/rust-learn/tree/feature-phantom)                                   | 2021-10-19                                | A demo to show how to use `PhantomData` beautify your code.<br />Linked Blog: [Rust中的PhantomType](https://jasonkayzk.github.io/2021/10/20/Rust中的PhantomType/)                                                      |
| [url-mapper-rs](https://github.com/JasonkayZK/rust-learn/tree/url-mapper-rs)                                       | 2021-12-04<br />(2021-12-21 Last Updated) | A simple URL Mapper service built in Rust.<br />Linked Youtube: https://www.youtube.com/playlist?list=PLz51_WNhdOqv7S5pnycKySU_4PpCagU4Q                                                                           |
|                                                                                                                    |                                           |                                                                                                                                                                                                                    |

<br/>

## **More Info**

- https://rust.cc/
- https://wiki.rust-china.org/
