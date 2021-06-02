const MAX_POINTS: u32 = 100_000;

fn main() {
    mut_demo();

    const_demo();

    shadowing_demo();
}

fn mut_demo() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn const_demo() {
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn shadowing_demo() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
}
