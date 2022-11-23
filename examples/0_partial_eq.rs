pub struct Person {
    pub id: u32,
    pub name: String,
    pub height: f64,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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

    println!("p1 == p2 = {}", p1 == p2);
}
