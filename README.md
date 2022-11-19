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
> ```bash
> cargo run
> ```

<br/>

## **Now Finished**

| Project                                                      | Date                                      | Note                                                         | Linked Blog                                                  |
| ------------------------------------------------------------ | ----------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| [chapter2-guessing-game](https://github.com/JasonkayZK/rust-learn/tree/chapter2-guessing-game) | 2021-06-01                                | A guessing game                                              |                                                              |
| [chapter3-variables](https://github.com/JasonkayZK/rust-learn/tree/chapter3-variables) | 2021-06-02                                | Variable & Type in Rust                                      |                                                              |
| [multiple-main-demo](https://github.com/JasonkayZK/rust-learn/tree/multiple-main-demo) | 2021-06-02                                | A demo to show how to run multiple main in Rust              |                                                              |
| [chapter4-function](https://github.com/JasonkayZK/rust-learn/tree/chapter4-function) | 2021-06-02                                | Function in Rust                                             |                                                              |
| [chapter5-control-flow](https://github.com/JasonkayZK/rust-learn/tree/chapter5-control-flow) | 2021-06-02                                | Control flow(if/loop/while/for) in Rust                      |                                                              |
| [chapter6-ownership](https://github.com/JasonkayZK/rust-learn/tree/chapter6-ownership) | 2021-06-03                                | Ownership(also string/slice) in Rust                         |                                                              |
| [chapter7-struct](https://github.com/JasonkayZK/rust-learn/tree/chapter7-struct) | 2021-06-04                                | Struct in Rust                                               |                                                              |
| [chapter8-enum-and-match](https://github.com/JasonkayZK/rust-learn/tree/chapter8-enum-and-match) | 2021-06-07                                | Enum & Match in Rust                                         |                                                              |
| [chapter9-modules](https://github.com/JasonkayZK/rust-learn/tree/chapter9-modules) | 2021-06-07                                | Modules(mod/pub/use/super) in Rust                           |                                                              |
| [chapter10-collections](https://github.com/JasonkayZK/rust-learn/tree/chapter10-collections) | 2021-06-09                                | Vector & String & Map in Rust                                |                                                              |
| [chapter11-error-handling](https://github.com/JasonkayZK/rust-learn/tree/chapter11-error-handling) | 2021-06-09                                | Error handling in Rust (Panic! & Result)                     |                                                              |
| [chapter12-generic-trait-lifetime](https://github.com/JasonkayZK/rust-learn/tree/chapter12-generic-trait-lifetime) | 2021-06-10                                | Generic & Trait & Lifetime in Rust                           |                                                              |
| [chapter13-testing](https://github.com/JasonkayZK/rust-learn/tree/chapter13-testing) | 2021-06-12                                | Testing(Write, Run & Organize) in Rust                       |                                                              |
| [chapter14-io-project-grep](https://github.com/JasonkayZK/rust-learn/tree/chapter14-io-project-grep) | 2021-06-13                                | A io project: `mini-grep` written in rust.                   |                                                              |
| [chapter15-functional-features](https://github.com/JasonkayZK/rust-learn/tree/chapter15-functional-features) | 2021-06-14                                | Functional features(Closure & Iterator) in rust.             |                                                              |
| [chapter16-cargo](https://github.com/JasonkayZK/rust-learn/tree/chapter16-cargo) | 2021-06-15                                | Cargo(Config, Publish, Install & Extend) & Workspace in rust. |                                                              |
| [chapter17-smart-pointer](https://github.com/JasonkayZK/rust-learn/tree/chapter17-smart-pointer) | 2021-09-29                                | Smart Pointer in Rust(Within double-linked-list accomplishment). |                                                              |
| [chapter18-concurrency](https://github.com/JasonkayZK/rust-learn/tree/chapter18-concurrency) | 2021-10-03                                | Concurrency in Rust.                                         |                                                              |
| [chapter19-oop](https://github.com/JasonkayZK/rust-learn/tree/chapter19-oop) | 2021-11-14                                | Object-Oriented-Programming in Rust.                         |                                                              |
| [chapter20-match-patterns](https://github.com/JasonkayZK/rust-learn/tree/chapter20-match-patterns) | 2021-11-14                                | The Match Patterns in Rust.                                  |                                                              |
| [chapter21-advanced-features](https://github.com/JasonkayZK/rust-learn/tree/chapter21-advanced-features) | 2021-11-14                                | The advanced features in Rust:<br />Unsafe、Lifetime、Trait、Type、Function & Closure |                                                              |
| [actix-web-demo](https://github.com/JasonkayZK/rust-learn/tree/actix-web-demo) | 2021-10-04                                | RESTful API accomplished by [actix-web](https://github.com/actix/actix-web) framework. |                                                              |
| [rbatis-demo](https://github.com/JasonkayZK/rust-learn/tree/rbatis-demo) | 2021-10-07                                | A demo to show how to use ORM framework: [rbatis](https://github.com/rbatis/rbatis) |                                                              |
| [wasm-hello](https://github.com/JasonkayZK/rust-learn/tree/wasm-hello) | 2021-10-09                                | A simple rust-wasm demo.[Use template: [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template)] | [《Rust实现WebAssembly初窥》](https://jasonkayzk.github.io/2021/10/10/Rust实现WebAssembly初窥/) |
| [feature-phantom](https://github.com/JasonkayZK/rust-learn/tree/feature-phantom) | 2021-10-19                                | A demo to show how to use `PhantomData` beautify your code   | [《Rust中的PhantomType》](https://jasonkayzk.github.io/2021/10/20/Rust中的PhantomType/) |
| [url-mapper-rs](https://github.com/JasonkayZK/rust-learn/tree/url-mapper-rs) | 2021-12-04<br />(2021-12-21 Last Updated) | A simple URL Mapper service built in Rust                    | [《Building a Web Application with Rust》](https://www.youtube.com/playlist?list=PLz51_WNhdOqv7S5pnycKySU_4PpCagU4Q) |
| [algorithm](https://github.com/JasonkayZK/rust-learn/tree/algorithm) | 2021-12-22                                | Collect lots of algorithm & data structures in Rust(Such as: LinkedList, …) |                                                              |
| [too-many-lists](https://github.com/JasonkayZK/rust-learn/tree/algorithm/too-many-lists) | 2022-01-05                                | A accomplishment for [Learn Rust With Entirely Too Many Linked Lists](https://github.com/rust-unofficial/too-many-lists) |                                                              |
| [ffi-demo](https://github.com/JasonkayZK/rust-learn/tree/ffi-demo) | 2022-01-17                                | A FFI(Foreign Function Interface) demo according to:<br />https://nomicon.purewhite.io/ffi.html |                                                              |
| [hot-reload](https://github.com/JasonkayZK/rust-learn/tree/hot-reload) | 2022-08-10                                | A demo to show hot-reload in Rust.<br />Reference: https://robert.kra.hn/posts/hot-reloading-rust/ |                                                              |
| [tokio](https://github.com/JasonkayZK/rust-learn/tree/tokio) | 2022-11-01                                | A branch to learn [tokio](https://github.com/tokio-rs/tokio) |                                                              |
| [recover](https://github.com/JasonkayZK/rust-learn/tree/recover) | 2022-11-17                                | A branch to show how rust recovered from panic               | [《Rust从panic中恢复》](https://jasonkayzk.github.io/2022/11/17/Rust从panic中恢复/) |
| [build-version](https://github.com/JasonkayZK/rust-learn/tree/build-version) | 2022-11-17                                | A branch to use `build.rs` add commit version for binary executable | [《为Cargo编译的可执行文件增加commit版本号》](https://jasonkayzk.github.io/2022/11/17/为Cargo编译的可执行文件增加commit版本号/) |
| [error](https://github.com/JasonkayZK/rust-learn/tree/error) | 2022-11-18                                | A branch to show error handle in Rust                        | [《Rust中的错误处理》](https://jasonkayzk.github.io/2022/11/18/Rust中的错误处理/) |
| [project-structure](https://github.com/JasonkayZK/rust-learn/tree/project-structure) | 2022-11-19                                | A branch to show how rust project structure organized        | [《Rust模块组织结构》](https://jasonkayzk.github.io/2022/11/19/Rust模块组织结构/) |
|                                                              |                                           |                                                              |                                                              |
|                                                              |                                           |                                                              |                                                              |

<br/>

## **Serial Project**

### **url-mapper-rs**

Project Space:

- [url-mapper-rs](https://github.com/JasonkayZK/rust-learn/tree/url-mapper-rs)

Learning Step:

- [Part I : Configuration](https://github.com/JasonkayZK/rust-learn/commit/12b88b1b5f5e02141ff90716feefea834817c34b)
- [Part II : Database Setup](https://github.com/JasonkayZK/rust-learn/commit/89327a61a4afda4e2fb9f55171889ee7fa205de5)
- [Part III - Database Manager: add mapper & tokio-async](https://github.com/JasonkayZK/rust-learn/commit/51120a38865911aa19a5fd4b093d077a40e95cd0)
- [Part IV: Basic Server & log tracing](https://github.com/JasonkayZK/rust-learn/commit/75267288ec824cd9b65f84245e14b37a9b4d5b4c)
- [Part V: Server and Database Manager communication](https://github.com/JasonkayZK/rust-learn/commit/cefc2ad7639c8359719cb639b9351c16db9e19d1)
- [Part VI - UrlMap CRUD API](https://github.com/JasonkayZK/rust-learn/commit/d77521b4c39ca953ef51cc75065f23a487ba6b12)
- [Part VII - Auth Middleware](https://github.com/JasonkayZK/rust-learn/commit/2da0d7d7ef20cf54bf4d01f4cc927e29ca5a58ea)
- [Part VIII - Containerization](https://github.com/JasonkayZK/rust-learn/commit/5d5cebcf69dccb809afb46b74dd6479991e511ae)
- [Part IX - Handling Signals & Deploying to Kubernetes](https://github.com/JasonkayZK/rust-learn/commit/03d3a5c76ad168da2ac3bd850e18bde6780d747f)
- [Part X - Frontend using Tera](https://github.com/JasonkayZK/rust-learn/commit/ad3828f69af89ea25092d8319bb6099cc357966f)
- [Part XI - React Front-End](https://github.com/JasonkayZK/rust-learn/commit/bdb21c2bff6ead55ba55554a51e0223e76453c60)

### algorithm

Project Space:

- [algorithm](https://github.com/JasonkayZK/rust-learn/tree/algorithm)
    - [sorting](https://github.com/JasonkayZK/rust-learn/tree/algorithm/algorithms/src/sorting)
        - [bubble_sort.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/algorithms/src/sorting/bubble_sort.rs)
        - [insertion_sort.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/algorithms/src/sorting/insertion_sort.rs)
        - [merge_sort.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/algorithms/src/sorting/merge_sort.rs)
        - [quick_sort.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/algorithms/src/sorting/quick_sort.rs)
        - [selection_sort.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/algorithms/src/sorting/selection_sort.rs)
- [collection](https://github.com/JasonkayZK/rust-learn/tree/algorithm/collection)
    - [list](https://github.com/JasonkayZK/rust-learn/tree/algorithm/collection/src/list)
        - [vector.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/collection/src/list/vector.rs)
        - [linked_list.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/collection/src/list/linked_list.rs)
    - [tree](https://github.com/JasonkayZK/rust-learn/tree/algorithm/collection/src/tree)
        - [binary_search_tree.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/collection/src/tree/binary_search_tree.rs)
- [concurrency](https://github.com/JasonkayZK/rust-learn/tree/algorithm/concurrency)
    - [my_arc.rs](https://github.com/JasonkayZK/rust-learn/blob/algorithm/concurrency/src/my_arc.rs)

Learning Step:

Not Yet!

<br/>

## **More Info**

- https://rust.cc/
- https://wiki.rust-china.org/
