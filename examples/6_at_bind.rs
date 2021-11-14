fn at_bind_demo() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("Found an id in range {}", id_var);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Id in another range");
        }
        Message::Hello { id } => {
            println!("Found other id: {}", id);
        }
    }
}

fn main() {
    at_bind_demo();
}
