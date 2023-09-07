use anyhow::Result;
use rusqlite::params;

use crate::models::person::Person;
use crate::storage::SQLITE_CLIENT;

pub fn insert_person(p: Person) -> Result<()> {
    let sql: &str = "INSERT INTO `t_person`(`uid`, `name`) VALUES (?, ?)";
    let c = &mut SQLITE_CLIENT.lock().unwrap().conn;
    c.execute(sql, params![p.uid, p.name])?;

    Ok(())
}
