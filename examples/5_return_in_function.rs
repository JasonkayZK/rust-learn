fn main() {
    return_demo1();
    return_demo2();
}

fn return_demo1() {
    let s1 = gives_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("hello");
    println!("s2: {}", s2);

    let s3 = takes_and_gives_back(s2);

    // Invalid: s2's value has been move into s3 in function takes_and_gives_back
    // println!("s2: {}", s2);

    println!("s3: {}", s3);
}

fn gives_ownership() -> String {
    let some_str = String::from("hello");
    some_str
}

fn takes_and_gives_back(a_str: String) -> String {
    a_str
}

fn return_demo2() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_len(s1);
    println!("len of {} is: {}", s2, len);
}

// move into & out s
// use tuple to return
fn calculate_len(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}
