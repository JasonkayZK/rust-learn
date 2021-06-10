fn main() {
    dangling_refer_demo();

    lifetime_in_function_demo();
    lifetime_in_function_demo2();
    lifetime_in_function_demo3();

    another_lifetime_in_function_demo();
    yet_another_lifetime_in_function_demo();
}

fn dangling_refer_demo() {
    let _r;
    {
        let x = 5;
        _r = &x;
        // x's value goes away, so r is a dangling refer
    }
    // Cause error here:
    // println!("r: {}", _r);
}

fn lifetime_in_function_demo() {
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let res = longest(str1.as_str(), str2);
    println!("longest: {}", res);
}

fn lifetime_in_function_demo2() {
    let str1 = String::from("long string is long");
    {
        let str2 = String::from("xyz");
        let res = longest(str1.as_str(), str2.as_str());
        println!("longest: {}", res);
    }
}

fn lifetime_in_function_demo3() {
    let str1 = String::from("long string is long");
    let _res;
    {
        let str2 = String::from("xyz");
        _res = longest(str1.as_str(), str2.as_str());
    }
    // Fail to compile!
    // println!("longest: {}", _res);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn another_lifetime_in_function_demo() {
    let str1 = String::from("long string is long");
    let res;
    {
        let str2 = String::from("xyz");
        res = another_longest(str1.as_str(), str2.as_str());
    }
    println!("longest: {}", res);
}

fn another_longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("{}", y);
    x
}

fn yet_another_lifetime_in_function_demo() {
    let str1 = String::from("long string is long");
    let str2;
    let res;
    {
        str2 = String::from("xyz");
        res = yet_another_longest(str1.as_str(), str2.as_str());
    }
    println!("longest: {}", res);
}

fn yet_another_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    println!("{}", y);
    x
}
