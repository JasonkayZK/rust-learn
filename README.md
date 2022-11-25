# **Rust反射之Any**

## **前言**

关于Rust为何不引入 Runtime Reflection 可以参考这个 RFC：

-   https://internals.rust-lang.org/t/pre-rfc-runtime-reflection/11039

大致总结如下：

DI 不一定非要使用反射来实现， Rust中可以有更好的实现：

派生宏和Trait之间的配合，可以将实现从运行时转移到编译时；

例如，利用过程宏实现了编译时反射功能，以实现依赖注入等反射功能：

-   https://github.com/dtolnay/reflect

**Rust 中提供了 `Any` Trait：所有类型（含自定义类型）都自动实现了该特征；**

因此，我们可以通过它进行一些类似反射的功能；

<br/>

## **Any解析**

下面是 `std::any` 模块的说明：

该模块实现了 `Any` trait，它可以通过运行时反射来动态键入任何 `'static` 类型；`Any` 本身可以用来获取 `TypeId`，并用作 trait 对象时具有更多功能；

作为 `&dyn Any` (借用的 trait 对象)，它具有 `is` 和 `downcast_ref` 方法，以测试所包含的值是否为给定类型，并对该类型的内部值进行引用；作为 `&mut dyn Any`，还有 `downcast_mut` 方法，用于获取内部值的变量引用；

`Box<dyn Any>` 添加了 `downcast` 方法，该方法尝试转换为 `Box<T>`；**注意，`&dyn Any` 仅限于测试值是否为指定的具体类型，而不能用于测试某个类型是否实现 Trait；**

总结如下，`std::any`起到的作用有4个：

-   **获得变量的类型TypeId；**
-   **判断变量是否是指定类型；**
-   **把any转换成指定类型；**
-   **获取类型的名字；**

下面是 Any Trait 的源码，以及对应的 TypeId 类型：

```rust
pub trait Any: 'static {
    fn type_id(&self) -> TypeId;
}

// 获得变量的类型TypeId
// 为所有的T实现了Any
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: 'static + ?Sized > Any for T {
    fn type_id(&self) -> TypeId { TypeId::of::<T>() }
}
 
// 判断变量是否是指定类型
#[stable(feature = "rust1", since = "1.0.0")]
#[inline]
pub fn is<T: Any>(&self) -> bool {
    // Get `TypeId` of the type this function is instantiated with.
    let t = TypeId::of::<T>();

    // Get `TypeId` of the type in the trait object.
    let concrete = self.type_id();

    // Compare both `TypeId`s on equality.
    t == concrete
}
 

// 把any转换成指定类型
#[stable(feature = "rust1", since = "1.0.0")]
#[inline]
pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
    if self.is::<T>() {
        // SAFETY: just checked whether we are pointing to the correct type
        unsafe {
            Some(&*(self as *const dyn Any as *const T))
        }
    } else {
        None
    }
}

// 获取类型名字
pub const fn type_name<T: ?Sized>() -> &'static str {
    intrinsics::type_name::<T>()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TypeId {
    t: u64,
}
```

<font color="#f00">**注意：所有拥有静态生命周期的类型都会实现Any，未来可能会考虑加入生命周期是非‘static的情况**</font>

在 Rust 中，每个类型都存在一个全局唯一的标识（A `TypeId` represents a globally unique identifier for a type）；

这些 TypeId 通过调用 intrinsic 模块中定义的函数来完成创建；

>   **关于intrinsic 模块：**
>
>   intrinsic 库函数是指：**由编译器内置实现的函数**，一般是具有如下特点的函数：
>
>   -   与CPU架构相关性很大，必须利用汇编实现或者利用汇编才能具备最高性能的函数；
>   -   和编译器密切相关的函数，由编译器来实现最为合适；

**因此，type_id 的生成是由编译器的实现来决定的！**

具体实现见：

-   https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_llvm/src/intrinsic.rs

<br/>

## **Any基本使用**

上一小节提到了 Any 可以实现：

-   **获得变量的类型TypeId；**
-   **判断变量是否是指定类型；**
-   **把any转换成指定类型；**
-   **获取类型的名字；**

下面我们通过具体代码来看：

examples/0_any.rs

```rust
use std::any::{Any, TypeId};

struct Person {
    pub name: String,
}

/// 获取TypeId
fn is_string(s: &dyn Any) -> bool {
    TypeId::of::<String>() == s.type_id()
}

/// 判断是否是指定类型
fn check_string(s: &dyn Any) {
    if s.is::<String>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}

/// 转换Any为特定类型
fn print_if_string(s: &dyn Any) {
    if let Some(ss) = s.downcast_ref::<String>() {
        println!("It's a string({}): '{}'", ss.len(), ss);
    } else {
        println!("Not a string...");
    }
}

/// 获取类型的名字
/// 通过此函数获得的名字不唯一！
/// 比如type_name::<Option<String>>()可能返回"Option<String>"或"std::option::Option<std::string::String>"
/// 同时编译器版本不同返回值可能不同
fn get_type_name<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    let p = Person { name: "John".to_string() };
    assert!(!is_string(&p));
    assert!(is_string(&p.name));

    check_string(&p);
    check_string(&p.name);

    print_if_string(&p);
    print_if_string(&p.name);

    println!("Type name of p: {}", get_type_name(&p));
    println!("Type name of p.name: {}", get_type_name(&p.name));
}
```

输出如下：

```
Not a string...
It's a string!
Not a string...
It's a string(4): 'John'
Type name of p: 0_any::Person
Type name of p.name: alloc::string::String
```

总结如下：

```rust
/// 获取TypeId，并比较: type_id
TypeId::of::<String>() == s.type_id()

/// 判断是否是指定类型: s.is
s.is::<String>()

/// 转换Any为特定类型: s.downcast_ref
s.downcast_ref::<String>()

/// 获取类型的名字: type_name::<T>()
/// 通过此函数获得的名字不唯一！
/// 比如type_name::<Option<String>>()可能返回"Option<String>"或"std::option::Option<std::string::String>"
/// 同时编译器版本不同返回值可能不同
std::any::type_name::<T>().to_string()
```

<br/>

## **Any的使用场景**

Rust 中的 Any 类似于 Java 中的 Object，可以传入任何拥有静态生命周期的类型；

因此在有些入参类型复杂的场景，我们可以简化入参；

例如，打印任何类型对应的值：

examples/1_print_any.rs

```rust
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
struct MyType {
    name: String,
    age: u32,
}

fn print_any<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;

    if let Some(string) = value_any.downcast_ref::<String>() {
        println!("String ({}): {}", string.len(), string);
    } else if let Some(MyType { name, age }) = value_any.downcast_ref::<MyType>() {
        println!("MyType ({}, {})", name, age)
    } else {
        println!("{:?}", value)
    }
}

fn main() {
    let ty = MyType {
        name: "Rust".to_string(),
        age: 30,
    };
    let name = String::from("Rust");

    print_any(&ty);
    print_any(&name);
    print_any(&30);
}
```

如上所示，不论是 String 类型、MyType 自定义类型，还是内置的i32类型，都可以被打印，只要他们实现了 Debug Trait；

可以认为这是Rust中一种函数重载的方式，在读取一些结构复杂的配置时，也可以直接使用Any；

<br/>

## **总结**

any特性并非实际意义上的 Reflection，最多是编译时反射；同时Rust只是启用类型检查和类型转换，而不是检查任意结构的内容；

any符合零成本抽象，因为Rust只会针对调用相关函数的类型生成代码，并且判断类型时返回的是编译器内部的类型ID，没有额外的开销；甚至可以直接使用 `TypeId::of::<String>`，从而没有了dyn any的动态绑定的开销；

虽然Rust没有提供 Reflection，但是过程宏可以实现大部分反射能够实现的功能！

实际上，在Rust的早期版本中提供了 Reflection功能，但是在[14年移除了相关代码](https://links.jianshu.com/go?to=https%3A%2F%2Fgithub.com%2Frust-lang%2Frfcs%2Fpull%2F379)，原因是：

-   **反射打破了原有的封装原则，可以任意访问结构体的内容，不安全；**
-   **反射的存在使得代码过于臃肿，移除后编译器会简化很多；**
-   **反射功能设计的比较弱，开发者对于是否在未来的版本中还拥有反射功能存疑；**

至于保留any的原因：

-   **在调试范型类型相关的代码的时候，有TypeId会更方便，更容易给出正确的错误提示；**
-   **有利于编译器作出代码的优化；**

<br/>

# **附录**

文章参考：

-   https://www.coder.rs/index.php/archives/517.html
-   https://internals.rust-lang.org/t/pre-rfc-runtime-reflection/11039/8
-   https://rust.ffactory.org/std/any/index.html
-   https://www.coder.rs/index.php/archives/517.html
-   https://www.jianshu.com/p/c4ef17bb1ca3

