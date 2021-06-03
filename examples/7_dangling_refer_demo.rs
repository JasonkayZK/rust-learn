fn main() {
    // let ref_to_nothing = dangle();
    // println!("{}", ref_to_nothing);

    let ref_to_str = no_dangle();
    println!("{}", ref_to_str);
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    // Invalid:
    // String::from created a string on heap, but the space will release when func dangle return;
    // So the return refer `&s` refer to a released space(invalid & danger!)
    // Which cause a error!
    &s
}
 */

fn no_dangle() -> String {
    String::from("hello")
}
