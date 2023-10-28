use rusqlite::params;

pub fn setup_db(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            brand TEXT NOT NULL,
            name TEXT NOT NULL
        )",
        params![],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS stash_items (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            expiry_date TEXT NOT NULL,
            FOREIGN KEY (product_id) REFERENCES products(id)
        )",
        params![],
    )?;

    Ok(())
}
