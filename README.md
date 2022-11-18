# **Rust中的错误处理**

## **Result枚举**

Rust 中没有提供类似于 Java、C++ 中的 Exception 机制，而是使用 `Result` 枚举的方式来实现：

```rust
pub enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}
```

在使用时：

-   **如果无错误则使用 `Ok(T)` 返回；**
-   **如果存在错误，则使用 `Err(E)` 包装错误类型返回；**

例如：

examples/0_result.rs

```rust
#[derive(Debug)]
pub enum MyError {
    Internal(String),
    InvalidId(String),
}

fn add(num: i64) -> Result<i64, MyError> {
    if num < 0 {
        Err(MyError::InvalidId(String::from("Invalid num!")))
    } else {
        Ok(num + 100000)
    }
}

fn main() -> Result<(), MyError> {
    // fetch_id(-1)?;

    let res = add(1)?;
    println!("{}", res);
    Ok(())
}
```

上面的代码首先通过 MyError 枚举定义了多个可能会出现的错误；

随后，在 `add` 函数中：

-   当 num 小于 0 时返回错误；
-   否则给 num 增加 100000 并返回；

在上面的 `let res = add(1)?;` 中使用了 `?` 操作符，他相当于是一个语法糖：

-   **如果被调函数正常返回则调用 `unwrap` 获取其值；**
-   **反之，则将被调函数的错误直接向上返回（相当于直接 return Err）；**

即上面的语法糖相当于：

```rust
let res = match add() {
  Ok(id) => id,
  Err(err) => {
    return Err(err);
  }
};
```

<br/>

## **错误类型转换**

上面简单展示了 Rust 中错误的使用；

由于 Rust 是强类型的语言，因此如果在一个函数中使用 `?` 返回了多个错误，并且他们的类型是不同的，**还需要对返回的错误类型进行转换，转为相同的类型！**

例如下面的例子：

```rust
#[derive(Debug)]
pub enum MyError {
    ReadError(String),
    ParseError(String),
}

fn read_file() -> Result<i64, MyError> {
    // Error: Could not get compiled!
    let content = fs::read_to_string("/tmp/id")?;
    let id = content.parse::<i64>()?;
}

fn main() -> Result<(), MyError> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
```

上面的例子无法编译通过，原因在于： **`read_to_string` 和 `parse` 返回的是不同类型的错误！**

因此，如果要能返回，我们需要对每一个错误进行转换，转为我们所定义的 Error 类型；

例如：

examples/1_error_convert.rs

```rust
fn read_file() -> Result<i64, MyError> {
    // Error: Could not get compiled!
    // let content = fs::read_to_string("/tmp/id")?;
    // let id = content.parse::<i64>()?;

    // Method 1: Handling error explicitly!
    let content = match std::fs::read_to_string("/tmp/id") {
        Ok(content) => content,
        Err(err) => {
            return Err(MyError::ReadError(format!("read /tmp/id failed: {}", err)));
        }
    };
    let content = content.trim();
    println!("read content: {}", content);

    // Method 2: Use map_err to transform error type
    let id = content
        .parse::<i64>()
        .map_err(|err| MyError::ParseError(format!("parse error: {}", err)))?;

    Ok(id)
}
```

上面展示了两种不同的转换 Error 的方法：

方法一通过 match 匹配手动的对 `read_to_string` 函数的返回值进行处理，如果发生了 Error，则将错误转为我们指定类型的错误；

方法二通过 `map_err` 的方式，如果返回的是错误，则将其转为我们指定的类型，这时就可以使用 `?` 返回了；

相比之下，使用 map_err 的方式，代码会清爽很多！

<br/>

### **From Trait**

上面处理错误的方法，每次都要对错误的类型进行转换，比较麻烦；

Rust 中提供了 From Trait，在进行类型匹配时，如果提供了从一个类型转换为另一个类型的方法（实现了某个类型的 From Trait），则在编译阶段，编译器会调用响应的函数，直接将其转为相应的类型！

例如：

examples/2_from_trait.rs

```rust
#[derive(Debug)]
pub enum MyError {
    ReadError(String),
    ParseError(String),
}

impl From<std::io::Error> for MyError {
    fn from(source: std::io::Error) -> Self {
        MyError::ReadError(source.to_string())
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(source: std::num::ParseIntError) -> Self {
        MyError::ParseError(source.to_string())
    }
}

fn read_file() -> Result<i64, MyError> {
    let _content = fs::read_to_string("/tmp/id")?;
    let content = _content.trim();
    let id = content.parse::<i64>()?;
    Ok(id)
}

fn main() -> Result<(), MyError> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
```

在上面的代码中，我们为 MyError 类型的错误分别实现了转换为 `std::io::Error` 和 `std::num::ParseIntError` 类型的 From Trait；

**因此，在 read_file 函数中就可以直接使用 `?` 向上返回错误了！**

但是上面的方法需要为每个错误实现 From Trait 还是有些麻烦，因此出现了 [thiserror](https://docs.rs/thiserror/latest/thiserror/) 以及 [anyhow](https://docs.rs/anyhow/latest/anyhow/) 库来解决这些问题；

<br/>

## **其他第三方库**

### **thiserror**

上面提到了我们可以为每个错误实现 From Trait 来直接转换错误类型，`thiserror` 库就是使用这个逻辑；

我们可以使用 thiserror 库提供的宏来帮助我们生成到对应类型的 Trait；

例如：

examples/3_thiserror.rs

```rust
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("io error.")]
    IoError(#[from] std::io::Error),
    #[error("parse error.")]
    ParseError(#[from] std::num::ParseIntError),
}

fn read_file() -> Result<i64, MyError> {
    // Could get compiled!
    let content = fs::read_to_string("/tmp/id")?;
    let id = content.parse::<i64>()?;
    Ok(id)
}

fn main() -> Result<(), MyError> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
```

我们只需要对我们定义的类型进行宏标注，在编译时这些宏会自动展开并实现对应的 Trait；

展开后的代码如下：

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::fs;
pub enum MyError {
    #[error("io error.")]
    IoError(#[from] std::io::Error),
    #[error("parse error.")]
    ParseError(#[from] std::num::ParseIntError),
}
#[allow(unused_qualifications)]
impl std::error::Error for MyError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        use thiserror::__private::AsDynError;
        #[allow(deprecated)]
        match self {
            MyError::IoError { 0: source, .. } => std::option::Option::Some(source.as_dyn_error()),
            MyError::ParseError { 0: source, .. } => {
                std::option::Option::Some(source.as_dyn_error())
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::fmt::Display for MyError {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            MyError::IoError(_0) => {
                let result =
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["io error."], &[]));
                result
            }
            MyError::ParseError(_0) => {
                let result =
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(&["parse error."], &[]));
                result
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<std::io::Error> for MyError {
    #[allow(deprecated)]
    fn from(source: std::io::Error) -> Self {
        MyError::IoError { 0: source }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<std::num::ParseIntError> for MyError {
    #[allow(deprecated)]
    fn from(source: std::num::ParseIntError) -> Self {
        MyError::ParseError { 0: source }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&MyError::IoError(ref __self_0),) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "IoError", &&*__self_0)
            }
            (&MyError::ParseError(ref __self_0),) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ParseError", &&*__self_0)
            }
        }
    }
}
fn read_file() -> Result<i64, MyError> {
    let content = fs::read_to_string("/tmp/id")?;
    let id = content.parse::<i64>()?;
    Ok(id)
}
#[allow(dead_code)]
fn main() -> Result<(), MyError> {
    let id = read_file()?;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["id: ", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&id)],
        ));
    };
    Ok(())
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
```

可以看到实际上就是为 MyError 实现了对应错误类型的 From Trait；

thiserror 库的这种实现方式，还需要为类型指定要转换的错误类型；

而下面看到的 anyhow 库，可以将错误类型统一为同一种形式；

<br/>

### **anyhow**

如果你对 Go 中的错误类型不陌生，那么你就可以直接上手 anyhow 了！

来看下面的例子：

examples/4_anyhow.rs

```rust
use anyhow::Result;
use std::fs;

fn read_file() -> Result<i64> {
    // Could get compiled!
    let content = fs::read_to_string("/tmp/id")?;
    let id = content.parse::<i64>()?;
    Ok(id)
}

fn main() -> Result<()> {
    let id = read_file()?;
    println!("id: {}", id);
    Ok(())
}
```

**注意到，上面的 Result 类型为 `anyhow::Result`，而非标准库中的 Result 类型！**

`anyhow` 为 `Result<T, E>` 实现了 `Context` Trait：

```rust
impl<T, E> Context<T, E> for Result<T, E> where
    E: ext::StdError + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        // Not using map_err to save 2 useless frames off the captured backtrace
        // in ext_context.
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context)),
        }
    }

    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context())),
        }
    }
}
```

在 `Context` 中提供了 `context` 函数，并且将原来的 `Result<T, E>` 转成了 `Result<T, anyhow::Error>`；

因此，最终将错误类型统一为了 `anyhow::Error` 类型；

<br/>

# **附录**

文档：

-   https://doc.rust-lang.org/book/ch09-00-error-handling.html
-   https://nick.groenen.me/posts/rust-error-handling/
-   https://docs.rs/thiserror/latest/thiserror/
-   https://docs.rs/anyhow/latest/anyhow/

<br/>
