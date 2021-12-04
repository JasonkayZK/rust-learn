// Compiling err:
// fn return_closure() -> Fn(i32) -> i32 {
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let add_func = return_closure();
    let res: i32 = add_func(3);
    println!("call add func: {}", res);
}
