#[derive(Debug, Default)]
pub struct Foo {
    bar: String,
    baz: i32,
    abc: bool,
}

// impl Default for Foo {
//     fn default() -> Self {
//         Foo {
//             bar: "".to_string(),
//             baz: 0,
//             abc: false,
//         }
//     }
// }

fn main() {
    let x = Foo::default();

    let y = Foo { baz: 2, ..Default::default() };

    println!("{:?}", x);
    println!("{:?}", y);
}
