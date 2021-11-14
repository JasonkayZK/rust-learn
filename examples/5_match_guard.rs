fn match_guard_demo() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => {
            println!("less than five: {}", x)
        }
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn match_guard_test_outer() {
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => {
            println!("got 50")
        }
        Some(n) if n == y => println!("matched, y = {}", y), // override the outer `y`!
        _ => println!("default case, x={:?}", x),
    }
}

fn match_guard_with_or() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => {
            println!("yes")
        }
        _ => println!("no"),
    }
}

fn main() {
    match_guard_demo();

    match_guard_test_outer();

    match_guard_with_or();
}
