use std::cmp::Ordering;

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
        self.height.partial_cmp(&other.height)
    }
}

fn main() {
    let p1 = Person {
        id: 0,
        name: "John".to_string(),
        height: 1.2,
    };

    let p2 = Person {
        id: 0,
        name: "Jack".to_string(),
        height: 1.4,
    };

    println!("p1 < p2 = {}", p1 < p2);
    println!("p1 <= p2 = {}", p1 <= p2);
    println!("p1 > p2 = {}", p1 > p2);
    println!("p1 >= p2 = {}", p1 >= p2);
}
