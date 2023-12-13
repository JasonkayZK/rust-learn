fn wrapping_demo() {
    println!("{}", (250_u8).wrapping_add(10)); // 4
    println!("{}", (120_i8).wrapping_add(10)); // -126
    println!("{}", (300_u16).wrapping_mul(800)); // 43392
    println!("{}", (-100_i8).wrapping_sub(100)); // 56
    println!("{}", (8000_i32).wrapping_pow(5000)); // 0
}

fn overflowing_demo() {
    // 4, true
    let (result, overflowed) = (250_u8).overflowing_add(10);

    println!(
        "sum is {} where overflow {} occur",
        result,
        if overflowed { "did" } else { "did not" },
    );
}

fn checked_demo() {
    // match (100_u8).checked_add(200) {
    match (100_u8).checked_add(20) {
        Some(result) => println!("{result}"),
        None => panic!("overflowed!"),
    }
}

fn saturating_demo() {
    println!("{}", (-32768_i16).saturating_sub(10)); // -32768
    println!("{}", (200_u8).saturating_add(100)); // 255
}

fn main() {
    wrapping_demo();
    println!();
    overflowing_demo();
    println!();
    checked_demo();
    println!();
    saturating_demo()
}
