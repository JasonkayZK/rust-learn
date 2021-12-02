#[derive(Debug)]
struct MyRef<'a, T>(&'a T);
// struct MyRef<'a, T: 'a>(&'a T);

fn main() {
    let x = "12345";

    println!("{:?}", MyRef(&x));

    println!("{}", x);
}
