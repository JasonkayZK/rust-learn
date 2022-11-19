# **Rust中的默认初始化和初始化重载**

## **引言**

与 Go 中默认给出默认值不同，Rust 要求在创建对象的时候就对各个字段进行初始化；

例如，下面的代码没有对字段进行初始化，无法编译：

```rust
#[derive(Debug)]
pub struct Foo {
    bar: String,
    baz: i32,
    abc: bool,
}

fn main() {
    let x = Foo {};

    println!("{:?}", x);
}
```

报错：

```
error[E0063]: missing fields `abc`, `bar` and `baz` in initializer of `Foo`
 --> examples/0_default.rs:8:13
  |
8 |     let x = Foo {};
  |             ^^^ missing `abc`, `bar` and `baz`
```

因此，我们要手动赋值：

```rust
let x = Foo {
  bar: "".to_string(),
  baz: 0,
  abc: false
};
```

但是每次都需要手动的指定空值，非常麻烦；

我们可以使用 Default Trait 来简化；

<br/>

## **Default Trait**

我们可以为 Foo 类型实现 Default Trait：

examples/0_default.rs

```rust
impl Default for Foo {
    fn default() -> Self {
        Foo {
            bar: "".to_string(),
            baz: 0,
            abc: false,
        }
    }
}

fn main() {
    let x = Foo::default();

    println!("{:?}", x);
}
```

随后即可使用 `Foo::default()` 初始化；

同时，也可以初始化部分字段，例如：

```rust
let y = Foo { baz: 2, ..Default::default() };
```

实际上，对于 Rust 中的常见类型，他们默认都实现了 Default Trait；

因此我们可以直接使用 `#[derive(Default)]` 来生成 Default Trait，而无需手动实现；

例如：

```rust
#[derive(Debug, Default)]
pub struct Foo {
    bar: String,
    baz: i32,
    abc: bool,
}

fn main() {
    let x = Foo::default();

    let y = Foo { baz: 2, ..Default::default() };

    println!("{:?}", x);
    println!("{:?}", y);
}
```

<br/>

## **With Trait**

在面向对象的语言中，可以通过单个参数或多个参数构造一个新的对象；

除了上面 Default 的方式外，还可以通过 With Trait 实现类似的功能；

例如：

examples/1_with.rs

```rust
pub trait With<T> {
    fn with(value: T) -> Self;
}

#[derive(Debug, Default)]
pub struct Foo {
    bar: String,
    baz: i32,
    abc: bool,
}

impl With<String> for Foo {
    fn with(x: String) -> Self {
        Foo {
            bar: x,
            ..Default::default()
        }
    }
}

impl With<i32> for Foo {
    fn with(x: i32) -> Self {
        Foo {
            baz: x,
            ..Default::default()
        }
    }
}

impl With<bool> for Foo {
    fn with(x: bool) -> Self {
        Foo {
            abc: x,
            ..Default::default()
        }
    }
}

impl With<(String, bool)> for Foo {
    fn with(x: (String, bool)) -> Self {
        Foo {
            bar: x.0,
            abc: x.1,
            ..Default::default()
        }
    }
}
```

在上面的代码中，我们定义的 With Trait：

```rust
pub trait With<T> {
    fn with(value: T) -> Self;
}
```

我们分别为 Foo 类型实现了不同范型类型的 With：String、i32、bool 甚至 `(String, bool)` 类型；

因此，我们可以使用 with 函数：

examples/1_with.rs

```rust
fn main() {
    let a = Foo::with("test".to_string());
    let b = Foo::with(1);
    let c = Foo::with(true);
    let d = Foo::with(("multi".to_string(), true));

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);
}
```

注意到，**上面调用的都是 `Foo::with` 方法，但是实际上是不同的 Trait 范型实现！**

**虽然 Rust 中没有范型，但是我们可以通过 Trait + Generic 的方式实现相同的功能！**

<br/>

## **`..`运算符**

最后，再补充一点，在上面的 `..Default::default()` 会将对象各个字段解构，随后赋给对应字段名相同的属性；

除了 default 构建的对象，正常的对象也可以使用这个运算符结构，例如：

examples/2_dot_operator.rs

```rust
#[derive(Debug)]
pub struct Foo {
    bar: String,
    baz: i32,
    abc: bool,
}

fn main() {
    let x = Foo {
        bar: "hello".to_string(),
        baz: 0,
        abc: false,
    };

    let y = Foo { abc: true, ..x };

    // println!("{:?}", x);
    println!("{:?}", y);
}
```

我们可以使用 `..x` 来构造 y；

但是需要注意的是，**`..` 也是 Move 语义，因此上面的代码如果在后面使用了x，则会报错！**

<br/>

# **附录**

参考文章：

-   https://www.cnblogs.com/cutepig/p/12685126.html
