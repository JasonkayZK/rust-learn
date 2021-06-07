fn main() {
    match_option_demo1();
}

fn match_option_demo1() {
    println!("add five: {:?}", plus_one(Some(5)));
    println!("none: {:?}", plus_one(None));

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

