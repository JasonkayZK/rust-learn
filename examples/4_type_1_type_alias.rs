type Kilometer = i32;

fn type_alias_demo() {
    let x: i32 = 5;
    let y: Kilometer = 6;
    println!("{}", x + y);
}

// Use type-alias to save code
type Thunk = Box<dyn Fn() + Send + 'static>;

fn type_alias_demo2() {
    let f: Thunk = Box::new(|| println!("hi"));
    takes_long_type(f);

    takes_long_type(returns_long_type());
}

fn takes_long_type(f: Thunk) {
    f()
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("haha"))
}

fn main() {
    type_alias_demo();

    type_alias_demo2();
}
