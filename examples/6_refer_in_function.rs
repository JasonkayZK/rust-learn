fn main() {
    refer_borrow_demo();

    change_demo();
}

fn refer_borrow_demo() {
    let s = String::from("hello");
    let len = calculate_len(&s);
    println!("len of {} is: {}", s, len);
}

// Use refer
fn calculate_len(s: &String) -> usize {
    // Invalid: can not modify reference's value in DEFAULT!
    // s.push_str(", world");

    s.len()
}

fn change_demo() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("before change r1: {}", r1);

    // Invalid: mut borrow can be occurred ONLY ONCE!
    // let r2 = &mut s;

    // Valid: r2 goes into different scope than r1 did
    {
        let r2 = &mut s;
        println!("before change r2: {}", r2);
    }

    // Invalid: can not use refer to borrow in both mutable and immutable at the same time!
    {
        let ref1 = &s;
        let ref2 = &s;
        // let ref3 = &mut s;

        println!("before change ref1: {}", ref1);
        println!("before change ref2: {}", ref2);
        // println!("before change ref3: {}", ref3);
    }

    change(&mut s);
    println!("after change: {}", s);
}

fn change(str: &mut String) {
    str.push_str(", world");
}
