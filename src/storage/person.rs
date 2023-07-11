use crate::entity::person::Person;
use anyhow::Result;
use rusqlite::{params, Connection};

pub fn add_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "INSERT INTO person (name, age, data) VALUES (?1, ?2, ?3)",
        params![person.name, person.age, person.data],
    )
    .unwrap();

    Ok(())
}

pub fn get_persons(conn: &Connection) -> Result<Vec<Person>> {
    let mut stmt = conn.prepare("SELECT * FROM person")?;
    let persons = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    let mut ret_persons = Vec::new();
    for p in persons {
        ret_persons.push(p?);
    }
    Ok(ret_persons)
}
