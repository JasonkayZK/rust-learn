use anyhow::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

const DATABASE_FILE: &str = "lifeline.db";

// Define migrations. These are applied atomically.
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> = Migrations::new(
        MIGRATIONS_DIR
            .dirs()
            .map(|dir| {
                dir.files()
                    .find(|f| f.path().ends_with("up.sql"))
                    .map(|up_file| M::up(up_file.contents_utf8().unwrap()))
                    .unwrap()
            })
            .collect()
    );
}

pub fn init_db() -> Result<Connection> {
    let mut conn = Connection::open(DATABASE_FILE)?;

    // Update the database schema, atomically
    MIGRATIONS.to_latest(&mut conn)?;

    // Apply some PRAGMA. These are often better applied outside of migrations, as some needs to be
    // executed for each connection (like `foreign_keys`) or to be executed outside transactions
    // (`journal_mode` is a noop in a transaction).
    conn.pragma_update(None, "journal_mode", "WAL").unwrap();
    conn.pragma_update(None, "foreign_keys", "ON").unwrap();

    Ok(conn)
}

// Test that migrations are working
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_test() {
        assert!(MIGRATIONS.validate().is_ok());
    }
}
