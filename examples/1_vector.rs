#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    init1();
    init2();

    destroy();

    access_elem();
    out_of_range();
    modify_while_access();

    traverse();
    enum_in_vec();
}

fn init1() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    println!("{:?}", v);
}

fn init2() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

fn destroy() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // v goes out of scope and is freed here! (Also, its elem!)
}

fn access_elem() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("third: {}", third);
    let third: Option<&i32> = v.get(2);
    if let Some(e) = third {
        println!("third: {}", e);
    }
}

fn out_of_range() {
    let v = vec![1, 2, 3, 4, 5];

    // Cause Panic!
    // let do_not_exist = &v[100];
    // println!("do_not_exist: {:?}", do_not_exist);

    // Return None!
    let do_not_exist = v.get(100);
    println!("do_not_exist: {:?}", do_not_exist);
}

fn modify_while_access() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6);

    // Cause err here: Use both mutable & immutable var!
    println!("first: {}", first);
}

fn traverse() {
    let mut v = vec![100, 32, 22];
    for x in &mut v {
        *x += 100;
    };
    for x in &v {
        println!("{}", x);
    }
}

fn enum_in_vec() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}",row);
}
