use rusqlite::params;

use crate::domain::errors::{ProductRepositoryError, StashItemRepositoryError};

pub fn setup_db(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            brand TEXT NOT NULL,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT
        )",
        params![],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS stash_items (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            expiry_date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT,
            FOREIGN KEY (product_id) REFERENCES products(id)
            UNIQUE (product_id, expiry_date)
        )",
        params![],
    )?;

    Ok(())
}

impl From<rusqlite::Error> for ProductRepositoryError {
    fn from(error: rusqlite::Error) -> Self {
        Self::PersisteneError(error.to_string())
    }
}

impl From<rusqlite::Error> for StashItemRepositoryError {
    fn from(error: rusqlite::Error) -> Self {
        Self::PersistenceError(error.to_string())
    }
}
