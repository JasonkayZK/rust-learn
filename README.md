# **Rust中的比较**

在Rust的 `core::cmp.rs` 模块中定义了许多用于两值之间比较的Trait，分别是：

-   PartialEq；
-   Eq；
-   PartialOrd；
-   Ord；

这四个 Trait 之间有这样一个关系：

-   **Eq 基于 PartialEq，即 `pub Trait Eq：PartialEq`；**
-   **PartialOrd 基于 PartialEq，即 `pub Trait PartialOrd：PartialEq`；**
-   **Ord 基于 Eq 和 PartialOrd，`pub Trait PartialOrd：Eq + PartialOrd<Self>`；**

同时还定义了比较结果，为 `Ordering` 枚举类型：

```rust
pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```

下面我们分别来看；

<br/>

## **部分等价关系 PartialEq**

先说最基础的 `PartialEq`，在这个 Trait 中定义了两个方法：

```rust
pub Trait PartialEq<Rhs: ?Sized = Self> {
    /// This method tests for `self` and `other` values to be equal, and is used
    /// by `==`.
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn eq(&self, other: &Rhs) -> bool;

    /// This method tests for `!=`. The default implementation is almost always
    /// sufficient, and should not be overridden without very good reason.
    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
```

-   eq：两个值相等的话就返回 `true`，需要使用者自行定义该方法；
-   ne：两个值不相等的话就返回 `true`，默认为 `!eq(&self)`；

`PartialEq Trait` 实现了[部分等价关系 Partial_equivalence_relation](https://en.wikipedia.org/wiki/Partial_equivalence_relation)，这种数值关系有以下特性：

-   **对称性 (symmetric)：如果 `a == b`，那么 `b == a`；**
-   **可传递性 (transitive)：如果 `a == b` 且 `b == c`，那么 `a == c`；**

**所有的基本数据类型都实现了 `PartialEq Trait`，它们都定义在 cmp.rs 源码文件里；**

并且，通常情况下只需要用 `#[derive]` 的方法实现即可，例如：

```rust
#[derive(PartialEq)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}
```

编译器会生成类似下面的代码：

```rust
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.name == other.name &&
            self.height == other.height
    }
}
```

如果在比较两个 `Person` 时，只想通过 `id` 属性来确定是不是同一个人，可以手动定义 `PartialEq Trait` 的实现：

```rust
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
```

<br/>

### **PartialEq和运算符重载**

对于实现了 PartialEq Trait 的类型，相应的也会重载 `==` 运算符；

例如：

examples/0_partial_eq.rs

```rust
pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let p1 = Person {
        id: 0,
        name: "John".to_string(),
        height: 1.2,
    };

    let p2 = Person {
        id: 0,
        name: "Jack".to_string(),
        height: 1.4,
    };

    println!("p1 == p2 = {}", p1 == p2); // p1 == p2 = true
}
```

<br/>

## **等价关系 Eq**

`Eq Trait` 实现了 [等价关系 Equivalence_relation](https://en.wikipedia.org/wiki/Equivalence_relation)，该数值关系具有以下特性：

-   **对称性 (symmetric)：如果 `a == b`，那么 `b == a`；**
-   **可传递性 (transitive)：如果 `a == b` 且 `b == c`，那么 `a == c`；**
-   **自反性 (reflexive)：`a == a`；**

**`Eq Trait` 基于 `PartialEq Trait`，并且在此之上并没有添加新的方法定义；**

>   **这个 Trait 只是用于告诉编译器，这是个 `等价关系` 而非 `部分等价关系`；**
>
>   **因为编译器并不能检测 `自反性 (reflexive)`；**

在标准库中，只有 f32 和 f64 没有实现 `Eq Trait`，因为浮点值有两个特殊的值：

-   **NAN：非数值不可比较，`NAN != NAN`；**
-   **INFINITY：无穷大；**

例如：

```rust
println!("NAN == NAN ? {}", std::f64::NAN == std::f64::NAN);
// NAN == NAN ? false

println!("INFINITY == INFINITY ? {}", std::f64::INFINITY == std::f64::INFINITY);
// INFINITY == INFINITY ? true
```

所以，上面的示例中定义的 `struct Person` 是无法用 `#[derive(Eq)]` 的方法定义的：

```rust
#[derive(Eq)]
struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}
```

编译器会报出以下错误:

```bash
188 |     height: f64,
    |     ^^^^^^^^^^^ the Trait `std::cmp::Eq` is not implemented for `f64`
    |
    = note: required by `std::cmp::AssertParamIsEq`
```

但我们可以手动实现该 Trait:

```rust
struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

impl Eq for Person {
    fn eq(&self, other: &Self) -> bool {
        ...
    }
}
```

**`Eq Trait` 基于 `PartialEq Trait`，因此实现了 `Eq Trait` 的类型自然也相应的重载了 `==` 运算符；**

<br/>

## **偏序关系 PartialOrd**

`PartialOrd Trait` 基于 `PartialEq Trait` 实现，它新定义了几个方法：

```rust
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}
```

-   **`partial_cmp`，需要使用者实现本方法，返回两值的比较结果；**
-   **lt，le，gt，ge 已经定义好；**

偏序关系有以下特性：

-   **不对称性 antisymmetry：如果 `a < b` 那么 `!(a > b)`；**
-   **可传递性 transitive：如果 `a < b` 且 `b < c` 那么 `a < c`；**

**标准库里的所有基本类型都已实现该 Trait；**

可直接使用 `#[derive]` 的方法实现该 Trait，也可像下面这样手动实现，这里是以身高来排序的：

```rust
impl PartialOrd for Person {
    fn partial_cmp(&self，other：&Self) -> Option<std::cmp::Ordering> {
        self.height.partial_cmp(&other.height)
    }
}
```

<br/>

### **PartialOrd和运算符重载**

和上面类似，对于实现了 PartialOrd Trait 的类型，相应的也会重载  `<`、`<=`、`>` 和 `>=` 运算符；

例如：

examples/1_parital_ord.rs

```rust
use std::cmp::Ordering;

pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.height.partial_cmp(&other.height)
    }
}

fn main() {
    let p1 = Person {
        id: 0,
        name: "John".to_string(),
        height: 1.2,
    };

    let p2 = Person {
        id: 0,
        name: "Jack".to_string(),
        height: 1.4,
    };

    println!("p1 < p2 = {}", p1 < p2);
    println!("p1 <= p2 = {}", p1 <= p2);
    println!("p1 > p2 = {}", p1 > p2);
    println!("p1 >= p2 = {}", p1 >= p2);
}
```

<br/>

## **全序关系 Ord**

`Ord Trait` 是基于 `PartialOrd Trait` 和 `Eq Trait` 实现，它新定义了几个方法：

```rust
pub trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
        Self: ~const Destruct,
    {
        // HACK(fee1-dead): go back to using `self.max_by(other, Ord::cmp)`
        // when trait methods are allowed to be used when a const closure is
        // expected.
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            Ordering::Greater => self,
        }
    }

    #[stable(feature = "ord_max_min", since = "1.21.0")]
    #[inline]
    #[must_use]
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
        Self: ~const Destruct,
    {
        // HACK(fee1-dead): go back to using `self.min_by(other, Ord::cmp)`
        // when trait methods are allowed to be used when a const closure is
        // expected.
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            Ordering::Greater => other,
        }
    }

    /// Restrict a value to a certain interval.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise this returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!((-3).clamp(-2, 1) == -2);
    /// assert!(0.clamp(-2, 1) == 0);
    /// assert!(2.clamp(-2, 1) == 1);
    /// ```
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: ~const Destruct,
        Self: ~const PartialOrd,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
```

-   **cmp：需要使用者实现本方法，返回两值的比较结果；**
-   **max，min，clamp：已经定义好；**

>   **注：clamp函数用于将数值限制在一个给定的区间[min, max]内；**

全序关系有以下特性：

-   **完整的不对称性 total antisymmetry：`a < b`，`a == b`，`a > b` 这三种结果只有一个是真；**
-   **可传递性 transitive：如果 `a < b` 且 `b < c` 那么 `a < c`；**

<font color="#f00">**在标准库中，f32 和 f64 没有实现 `Ord Trait`，同样是因为 `NAN` 和 `INFINITY` 的 不确定性，`NAN` 和 `INFINITY` 无法跟其它浮点值比较大小；**</font>

<br/>

## **PartialOrd和Ord的区别**

PartialOrd和Ord的区别在于，**PartialOrd 是部分有序的**（说了又好像没说。。。）；

简单来说：

<font color="#f00">**如果我们的类型只在部分情况下具有相等性，那你就只能实现 `PartialEq`，否则可以实现 `PartialEq` 然后再默认实现 `Eq`；**

<font color="#f00">**同时，从代码的角度来说，PartialOrd Trait 返回值类型为 `Option<Ordering>`，而 Ord Trait 的返回值为 `Ordering`；**</font>

<font color="#f00">**即对于 PartialOrd，存在我们无法确定的比较结果！**</font>

<br/>

### **部分相等性**

首先我们需要找到一个类型，它实现了 `PartialEq` 但是没有实现 `Eq`；

>   **由于部分相等肯定是全部相等的子集，所以不存在反过来的情况；**

Rust 中 `HashMap` 的 key 要求实现 `Eq` 特征，也就是要能完全相等，而浮点数由于没有实现 `Eq` ，因此不能用于 `HashMap` 的 key；

那么，让我们考虑浮点数既然没有实现 `Eq` 为何还能进行比较呢？

```rust
fn main() {
   let f1 = 3.14;
   let f2 = 3.14;

   if f1 == f2 {
       println!("hello, world!");
   }
}
```

以上代码是可以看到输出内容的，既然浮点数没有实现 `Eq` 那说明它实现了 `PartialEq`！

可以写个简单代码验证下：

```rust
fn main() {
    let f1 = 3.14;
    is_eq(f1);
    is_partial_eq(f1)
}

fn is_eq<T: Eq>(f: T) {}
fn is_partial_eq<T: PartialEq>(f: T) {}
```

上面的代码通过特征约束的方式验证了我们的结论：

```shell
3 |     is_eq(f1);
  |     ----- ^^ the trait `Eq` is not implemented for `{float}`
```

我们成功找到了一个类型实现了 `PartialEq` 但没有实现 `Eq`，那就通过它来看看何为部分相等性；

其实**答案很简单：浮点数有一个特殊的值 `NaN`，它是无法进行相等性比较的！**

```rust
fn main() {
    let f1 = f32::NAN;
    let f2 = f32::NAN;

    if f1 == f2 {
        println!("NaN 竟然可以比较，这很不数学啊！")
    } else {
        println!("果然，虽然两个都是 NaN ，但是它们其实并不相等")
    }
}
```

因此，既然浮点数有一个值不可以比较相等性，那它自然只能实现 `PartialEq` 而不能实现 `Eq` 了！

简而言之：

<font color="#f00">**全序规则要求该类型包括的所有元素对都是可比较的，而 NaN 不可以！**</font>

以此类推，如果我们的类型也有这种特殊要求，那也应该这么做！

>   **注：Ord 意味着一个类型的所有值都可以进行排序，而 PartialOrd 则不然！**

>   **详细说明：**
>
>   -   https://course.rs/difficulties/eq.html
>   -   https://zh.wikipedia.org/wiki/%E5%81%8F%E5%BA%8F%E5%85%B3%E7%B3%BB
>   -   https://zh.m.wikipedia.org/zh-hans/NaN

<br/>

## **指定排序规则**

### **为类型实现Ord Trait**

vector 中的 sort 方法要求类型实现了 Ord Trait；

例如：

examples/2_sort.rs

```rust
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Eq for Person {}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn main() {
    let mut v = vec![
        Person {
            id: 3,
            name: "".to_string(),
            height: 3.0,
        },
        Person {
            id: 2,
            name: "".to_string(),
            height: 4.0,
        },
        Person {
            id: 1,
            name: "".to_string(),
            height: 5.0,
        },
    ];

    v.sort();

    println!("{:?}", v);
    // [Person { id: 1, name: "", height: 5.0 }, Person { id: 2, name: "", height: 4.0 }, Person { id: 3, name: "", height: 3.0 }]
}
```

<br/>

### **使用sort_by**

上面为 Person 实现了 Ord Trait，因此可以使用 `v.sort` 进行排序；

但是有的时候不想为这个类型实现一大堆的 Trait，此时可以使用 sort_by，并传入 lambda 表达式：

```rust
pub fn sort_by<F>(&mut self, mut compare: F)
where
F: FnMut(&T, &T) -> Ordering,
{
  merge_sort(self, |a, b| compare(a, b) == Less);
}
```

我们只需要传入一个返回 Ordering 枚举的比较函数即可！

例如：

examples/2_sort2.rs

```rust
#[derive(Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

fn main() {
    let mut v = vec![
        Person {
            id: 3,
            name: "".to_string(),
            height: 3.0,
        },
        Person {
            id: 1,
            name: "".to_string(),
            height: 5.0,
        },
        Person {
            id: 2,
            name: "".to_string(),
            height: 4.0,
        },
    ];

    v.sort_by(|a, b| a.id.cmp(&b.id));

    println!("{:?}", v);
    // [Person { id: 1, name: "", height: 5.0 }, Person { id: 2, name: "", height: 4.0 }, Person { id: 3, name: "", height: 3.0 }]
}
```

代码是不是清爽了许多？

<br/>

# **附录**

文章参考：

-   https://blog.biofan.org/2019/08/rust-cmp/
-   [Equivalence_relation](https://en.wikipedia.org/wiki/Equivalence_relation)
-   [Partial_equivalence_relation](https://en.wikipedia.org/wiki/Partial_equivalence_relation)
-   [Total_order](https://en.wikipedia.org/wiki/Total_order)
-   [nightly 版的 cmp.rs 源代码](https://blog.biofan.org/2019/08/rust-cmp/cmp.rs)
-   [Part 3 Equivalence relations 等价关系与偏序关系](https://wenku.baidu.com/view/58f21acdba4cf7ec4afe04a1b0717fd5360cb29a.html)
-   https://stackoverflow.com/questions/70588237/why-sort-need-t-to-be-ord#comment124782439_70588237
-   https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html

