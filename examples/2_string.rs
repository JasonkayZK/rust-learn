fn main() {
    init();
    append();

    concat1();
    concat2();

    index();
    utf8_demo();
    index2();

    traverse();
}


fn init() {
    // init 1
    let mut str1 = String::new();
    str1.push('c');

    // init 2
    let str2 = "haha"; // &str type
    let str2 = str2.to_string(); // String type

    // init 3
    let str3 = String::from("hello");

    println!("s1: {}, s2: {}, s3: {}", str1, str2, str3);
}

fn append() {
    let mut s1 = String::from("foo");

    let s2 = "bar";
    s1.push_str(&s2);
    // Is ok to print
    println!("s2: {}", s2);

    s1.push('s');
    println!("s1: {}", s1);
}

fn concat1() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");

    // s1 value removed here!
    let s3 = s1 + &s2;

    // Error: s1 has no value!
    // println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
    println!("s2: {}, s3: {}", s2, s3);
}

fn concat2() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s1: {}, s2: {}, s3: {}, s: {}", s1, s2, s3, s);
}

fn index() {
    let s1 = String::from("hello");

    // Cause err!
    // let h = s1[0];

    let h = s1.chars().nth(0);

    println!("h: {:?}", h);
}

fn utf8_demo() {
    println!("{}", String::from("Hello").len());
    println!("{}", String::from("你好").len());
}

fn index2() {
    let hello = "你好";

    let s1 = &hello[0..3];
    println!("s1: {}", s1);

    // Cause panic: Han use 3 bytes to store!
    // let s1 = &hello[0..2];
    // println!("s1: {}", s1);
}

fn traverse() {
    let hello = "你好";

    for x in hello.chars() {
        println!("{}", x);
    }

    for x in hello.bytes() {
        println!("{}", x);
    }
}
