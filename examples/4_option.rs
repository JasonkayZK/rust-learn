fn main() {
    init_option();

    use_option();
}

fn init_option() {
    let some_num = Some(5);
    let some_str = Some(String::from("hello"));

    // Explicit type: compiler can't infer
    let absent_num : Option<i32> = None;

    println!("some_num: {:?}", some_num);
    println!("some_str: {:?}", some_str);
    println!("absent_num: {:?}", absent_num);
}

fn use_option() {
    let x: i8 = 5;
    let y: Option<i8> = Some(8);

    // Invalid: x & y are different type
    // let sum = x + y;

    if let Some(y) = y {
        println!("x + y: {}", y +x );
    }
}
