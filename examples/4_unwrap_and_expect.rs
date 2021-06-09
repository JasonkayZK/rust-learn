use std::fs::File;

fn main() {
    // unwrap_demo();
    expect_demo();
}

fn unwrap_demo() {
    let _f = File::open("not_found.txt").unwrap();
}

fn expect_demo() {
    let _f = File::open("not_found.txt").expect("Failed to open not_found.txt");
}
