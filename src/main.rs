use crate::dao::person::insert_person;
use crate::models::person::Person;

mod dao;
mod models;
mod storage;

fn main() {
    insert_person(Person {
        uid: uuid::Uuid::new_v4().to_string(),
        name: "abc".to_string(),
        ..Default::default()
    })
    .unwrap();

    insert_person(Person {
        uid: uuid::Uuid::new_v4().to_string(),
        name: "def".to_string(),
        ..Default::default()
    })
    .unwrap();
}
