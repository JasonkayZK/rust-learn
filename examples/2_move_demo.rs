fn main() {
    move_demo1();
    move_demo2();
}

fn move_demo1() {
    let x = 5;
    let y = x;

    // Valid: Integer's size is known, so it will go to stack!
    println!("x={}, y={}", x, y);
}

fn move_demo2() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Invalid: s1's value has been moved to s2!
    // println!("{}, world", s1);

    // Valid:
    println!("{}, world", s2);
}
