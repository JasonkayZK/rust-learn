use std::any::{Any, TypeId};

struct Person {
    pub name: String,
}

/// 获取TypeId
fn is_string(s: &dyn Any) -> bool {
    TypeId::of::<String>() == s.type_id()
}

/// 判断是否是指定类型
fn check_string(s: &dyn Any) {
    if s.is::<String>() {
        println!("It's a string!");
    } else {
        println!("Not a string...");
    }
}

/// 转换Any为特定类型
fn print_if_string(s: &dyn Any) {
    if let Some(ss) = s.downcast_ref::<String>() {
        println!("It's a string({}): '{}'", ss.len(), ss);
    } else {
        println!("Not a string...");
    }
}

/// 获取类型的名字
/// 通过此函数获得的名字不唯一！
/// 比如type_name::<Option<String>>()可能返回"Option<String>"或"std::option::Option<std::string::String>"
/// 同时编译器版本不同返回值可能不同
fn get_type_name<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    let p = Person {
        name: "John".to_string(),
    };
    assert!(!is_string(&p));
    assert!(is_string(&p.name));

    check_string(&p);
    check_string(&p.name);

    print_if_string(&p);
    print_if_string(&p.name);

    println!("Type name of p: {}", get_type_name(&p));
    println!("Type name of p.name: {}", get_type_name(&p.name));
}
