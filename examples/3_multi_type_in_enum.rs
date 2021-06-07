#[derive(Debug)]
enum Message {
    // No data
    Quit,
    // Have data
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("self: {:?}", self)
    }
}

fn main() {
    call_fn();
}

fn call_fn() {
    Message::Quit.call();
    Message::Move { x: 12, y: 2 }.call();
    Message::Write(String::from("hello")).call();
    Message::ChangeColor(1,2,3).call();
}
