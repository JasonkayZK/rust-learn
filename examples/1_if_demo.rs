fn main() {
    if_demo(4);
    println!("5 is even? {}", let_if(5));
}

fn if_demo(n: i32) {
    if n % 4 == 0 {
        println!("n % 4");
    } else if n % 3 == 0 {
        println!("n % 3");
    } else if n % 2 == 0 {
        println!("n % 2");
    } else {
        println!("none");
    }
}

fn let_if(x: i32) -> bool {
    let res = if x % 2 == 0 {
        true
    } else {
        false
    };
    return res;
}
