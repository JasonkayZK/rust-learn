#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    print_rect();
}

fn print_rect() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    // Invalid: `Rectangle` doesn't implement std::fmt::`Display` (required by {})
    // println!("rect1 is {}", rect1);

    println!("rect1 is {:#?}", rect1);
}


