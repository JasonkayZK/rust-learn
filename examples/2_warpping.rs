use std::num::Wrapping;

fn main() {
    let mut x = Wrapping(125_u8);

    println!("{}", x + Wrapping(200)); // 69
    println!("{}", x - Wrapping(200)); // 181

    // 如果我们同时更改变量 x, 那么可以直接使用基本数据类型, 不用再套一层
    // x 现在为 113
    x *= 5;
    println!("{}", x);

    // 错误! 注意 - 我们只可以在有赋值操作时使用基本数据类型
    // (如在使用 += -= 等操作符时)
    // x / 5;
}
