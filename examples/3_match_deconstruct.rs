struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    // No data
    Quit,
    // Have data
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn deconstruct_struct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn deconstruct_struct_2() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x: 0, y: 0 } => {
            println!("at origin: (0, 0)")
        }
        Point { x, y: 0 } => {
            println!("on the x axis at {}", x)
        }
        Point { x: 0, y } => {
            println!("on the y axis at {}", y)
        }
        Point { x, y } => {
            println!("on neither axis: ({}, {})", x, y)
        }
    }
}

fn deconstruct_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("quit has no data")
        }
        Message::Move { x, y } => {
            println!("move to ({}, {})", x, y)
        }
        Message::Write(text) => {
            println!("text msg: {}", text)
        }
        Message::ChangeColor(r, g, b) => {
            println!("change (r,g,b): ({}, {}, {})", r, g, b)
        }
    }
}

fn deconstruct_refer() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    println!("sum_of_squares: {}", sum_of_squares);
}

fn main() {
    deconstruct_struct();

    deconstruct_struct_2();

    deconstruct_enum();

    deconstruct_refer();
}
