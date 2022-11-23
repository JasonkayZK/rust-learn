#[derive(Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

fn main() {
    let mut v = vec![
        Person {
            id: 3,
            name: "".to_string(),
            height: 3.0,
        },
        Person {
            id: 1,
            name: "".to_string(),
            height: 5.0,
        },
        Person {
            id: 2,
            name: "".to_string(),
            height: 4.0,
        },
    ];

    v.sort_by(|a, b| a.id.cmp(&b.id));

    println!("{:?}", v);
}
