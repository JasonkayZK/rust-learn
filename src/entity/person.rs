#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: u8,
    pub data: Option<Vec<u8>>,
}
