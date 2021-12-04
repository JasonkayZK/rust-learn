fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn closure_demo_1() {
    println!("do_twice: {}", do_twice(add_one, 5));
}

fn closure_demo_2() {
    let list = vec![1, 2, 3];

    // Method 1: raw closure
    let list_of_string: Vec<String> = list.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_string);

    // Method 2: Fully constraint
    let list_of_string: Vec<String> = list.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_string);
}

fn main() {
    closure_demo_1();

    closure_demo_2();
}
