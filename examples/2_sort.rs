use std::cmp::Ordering;

#[derive(Debug)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Eq for Person {}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn main() {
    let mut v = vec![
        Person {
            id: 3,
            name: "".to_string(),
            height: 3.0,
        },
        Person {
            id: 2,
            name: "".to_string(),
            height: 4.0,
        },
        Person {
            id: 1,
            name: "".to_string(),
            height: 5.0,
        },
    ];

    v.sort();

    println!("{:?}", v);
}
