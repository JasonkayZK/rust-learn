use std::sync::Mutex;

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use log::info;
use rusqlite::LoadExtensionGuard;

lazy_static! {
    pub static ref SQLITE_CLIENT: Mutex<SqliteDB> = Mutex::new({
        SqliteDB::init();
        SqliteDB::new()
    });
    static ref NOW: i32 = chrono::Local::now().timestamp_millis() as i32;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct SqliteDB {
    pub conn: rusqlite::Connection,
}

impl SqliteDB {
    pub fn new() -> Self {
        let db_uri = Self::db_filename();
        let c = rusqlite::Connection::open(&db_uri)
            .unwrap_or_else(|_| panic!("Error connecting to {:?}", db_uri));

        unsafe {
            let _guard = LoadExtensionGuard::new(&c).unwrap();
            c.load_extension(
                "./src/lib/crsqlite-darwin-aarch64.dylib",
                Some("sqlite3_crsqlite_init"),
            )
            .unwrap_or_else(|e| panic!("Error loading extension: {:?}", e));
        }

        c.query_row("SELECT crsql_as_crr('t_person');", [], |_| Ok(()))
            .unwrap_or_else(|e| panic!("Error loading extension: {:?}", e.to_string()));

        SqliteDB { conn: c }
    }

    pub fn init() {
        let db_uri = Self::db_filename();
        let mut c = SqliteConnection::establish(&db_uri)
            .unwrap_or_else(|_| panic!("Error connecting to {:?}", db_uri));

        // This will run the necessary migrations.
        c.run_pending_migrations(MIGRATIONS)
            .unwrap_or_else(|_| panic!("Error running migration"));

        info!("Database migration success!");
    }

    fn db_filename() -> String {
        format!("cr-sqlite-{}.sqlite", *NOW)
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::SQLITE_CLIENT;

    #[test]
    fn test_build() {
        let c = &mut SQLITE_CLIENT.lock().unwrap().conn;

        let cnt = c
            .query_row("select count(*) from t_person", [], |row| {
                row.get(0) as rusqlite::Result<i32>
            })
            .unwrap();
        println!("{}", cnt);
    }
}
