fn main() {
    while_demo();
    println!();
    for_in_demo();
    println!();
    count_down();
    println!();
    traverse();
}

fn loop_demo() {
    loop {
        println!("again");
    }
}

fn while_demo() {
    let mut n = 3;
    while n >= 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!");
}

fn for_in_demo() {
    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("the value is: {}", elem);
    }
}

fn count_down() {
    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!");
}

fn traverse() {
    let a = [10, 20, 30, 40, 50];

    for n in 0..a.len() {
        println!("the value is: a[{}]={}", n, a[n]);
    }
}
