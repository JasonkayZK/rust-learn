fn match_const() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything but not 1,2"),
    }
}

fn match_var() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => {
            println!("got 50")
        }
        Some(y) => println!("matched, y = {}", y), // override the outer `y`!
        _ => println!("default case, x={:?}", x),
    }
}

fn match_multi() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything but not 1,2,3"),
    }
}

fn match_range() {
    let x = 5;
    match x {
        1...5 => println!("one through five"),
        _ => println!("anything but not 1-5"),
    }
}

fn match_range_2() {
    let x = 'c';
    match x {
        'a'...'j' => println!("early alphabet"),
        'k'...'z' => println!("late alphabet"),
        _ => println!("something else"),
    }
}

fn main() {
    match_const();

    match_var();

    match_multi();

    match_range();

    match_range_2();
}
