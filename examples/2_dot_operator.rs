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
