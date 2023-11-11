use rusqlite::{params, types::FromSql, ToSql};

use crate::domain::{brand::Brand, product::ProductId};

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

impl ToSql for Brand {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>, rusqlite::Error> {
        Ok(self.value().to_sql()?)
    }
}

impl FromSql for Brand {
    fn column_result(
        value: rusqlite::types::ValueRef<'_>,
    ) -> Result<Self, rusqlite::types::FromSqlError> {
        let str = value.as_str()?;

        str.parse()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl ToSql for ProductId {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>, rusqlite::Error> {
        Ok(self.value().to_sql()?)
    }
}

impl FromSql for ProductId {
    fn column_result(
        value: rusqlite::types::ValueRef<'_>,
    ) -> Result<Self, rusqlite::types::FromSqlError> {
        let str = value.as_str()?;

        str.parse()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brand_to_from_sql() {
        let brand_str = "BRAND";
        let brand: Brand = brand_str.parse().unwrap();

        let connection = rusqlite::Connection::open_in_memory().unwrap();
        connection
            .execute("CREATE TABLE test (brand TEXT NOT NULL)", params![])
            .unwrap();

        connection
            .execute("INSERT INTO test (brand) VALUES (?)", params![brand])
            .unwrap();

        let mut statement = connection
            .prepare("SELECT brand FROM test LIMIT 1")
            .unwrap();
        let mut rows = statement.query(params![]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let brand_from_sql: Brand = row.get(0).unwrap();

        assert_eq!(brand, brand_from_sql);
    }

    #[test]
    fn test_product_id_to_from_sql() {
        let id_str = "id";
        let id: ProductId = id_str.parse().unwrap();

        let connection = rusqlite::Connection::open_in_memory().unwrap();
        connection
            .execute("CREATE TABLE test (id TEXT NOT NULL)", params![])
            .unwrap();

        connection
            .execute("INSERT INTO test (id) VALUES (?)", params![id])
            .unwrap();

        let mut statement = connection.prepare("SELECT id FROM test LIMIT 1").unwrap();
        let mut rows = statement.query(params![]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let id_from_sql: ProductId = row.get(0).unwrap();

        assert_eq!(id, id_from_sql);
    }
}
