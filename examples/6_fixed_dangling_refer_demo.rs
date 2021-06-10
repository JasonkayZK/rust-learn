fn main() {
    fixed_dangling_refer_demo();
}

fn fixed_dangling_refer_demo() {
    let r: &i32;
    {
        // x is a refer to `5`
        let x: &i32 = &5;

        // move(copy) x's val to r
        r = x;

        println!("x: {}", x);
    };

    println!("r: {}", r);
}
