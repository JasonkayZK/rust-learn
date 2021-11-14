use std::option::Option::Some;

fn match_branch(x: isize) {
    match x {
        1 => {
            println!("1");
        }
        2 => {
            println!("2");
        }
        _ => {
            println!("not 1 & 2");
        }
    }
}

fn if_let_demo() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("using your favorite color: {}, as background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple");
        } else {
            println!("using orange");
        }
    } else {
        // Compiler will NOT check this!
        println!("using blue");
    }
}

fn while_let_demo() {
    let mut stack = vec![1, 2, 3];

    while let Some(v) = stack.pop() {
        println!("{}", v);
    }
}

fn for_pattern_demo() {
    let v = vec![1, 2, 3];

    for (idx, v) in v.iter().enumerate() {
        println!("{} is at {}", v, idx);
    }
}

fn let_pattern_demo() {
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Compiling err:
    // let (x, y) = (1, 2, 3);

    let (x, ..) = (4, 3, 2, 1);
    println!("x: {}", x);

    let (x, _, y, z) = (5, 6, 7, 8);
    println!("x: {}, y: {}, z: {}", x, y, z);

    let (x, .., y) = (9, 10, 11, 12);
    println!("x: {}, y: {}", x, y);
}

fn func_pattern_demo(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}

fn main() {
    match_branch(2);

    if_let_demo();

    while_let_demo();

    for_pattern_demo();

    let_pattern_demo();

    let point = (3, 5);
    func_pattern_demo(&point);
}
