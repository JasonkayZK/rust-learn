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
