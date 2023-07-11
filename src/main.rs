mod entity;
mod storage;

use crate::entity::person::Person;
use crate::storage::migration::init_db;
use crate::storage::person::{add_person, get_persons};
use boost_rs::logger;
use boost_rs::logger::log::info;

fn main() {
    // Step 1: Init logger
    logger::init(Some(logger::LogLevel::Trace));

    // Step 2: Init database
    let conn = init_db().unwrap();

    // Use the db 🥳
    add_person(
        &conn,
        &Person {
            id: 0,
            name: "John".to_string(),
            age: 18,
            data: Some(Vec::from("test")),
        },
    )
    .unwrap();

    let persons = get_persons(&conn);
    info!("{:?}", persons);
}
