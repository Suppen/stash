use rusqlite::{
    types::{FromSql, ToSqlOutput},
    ToSql,
};

use crate::domain::value_objects::{Brand, ProductId, Quantity};

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

impl ToSql for Quantity {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>, rusqlite::Error> {
        let val = self.value();

        if val > i64::MAX as u64 {
            return Err(rusqlite::Error::ToSqlConversionFailure(
                format!("Quantity {} is too large to fit in an i64", val).into(),
            ));
        }

        Ok(ToSqlOutput::from(val as i64))
    }
}

impl FromSql for Quantity {
    fn column_result(
        value: rusqlite::types::ValueRef<'_>,
    ) -> Result<Self, rusqlite::types::FromSqlError> {
        let val = value.as_i64()?;

        if val < 0 {
            return Err(rusqlite::types::FromSqlError::InvalidType);
        }

        Ok(Quantity::new(val as u64).map_err(|_| rusqlite::types::FromSqlError::InvalidType)?)
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::params;

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

    #[test]
    fn test_quantity_to_from_sql() {
        let quantity = Quantity::new(1).unwrap();

        let connection = rusqlite::Connection::open_in_memory().unwrap();
        connection
            .execute("CREATE TABLE test (quantity INTEGER NOT NULL)", params![])
            .unwrap();

        connection
            .execute("INSERT INTO test (quantity) VALUES (?)", params![quantity])
            .unwrap();

        let mut statement = connection
            .prepare("SELECT quantity FROM test LIMIT 1")
            .unwrap();
        let mut rows = statement.query(params![]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let quantity_from_sql: Quantity = row.get(0).unwrap();

        assert_eq!(quantity, quantity_from_sql);
    }

    #[test]
    fn test_quantity_to_sql_overflow() {
        let quantity = Quantity::new(u64::MAX).unwrap();

        let connection = rusqlite::Connection::open_in_memory().unwrap();
        connection
            .execute("CREATE TABLE test (quantity INTEGER NOT NULL)", params![])
            .unwrap();

        let result =
            connection.execute("INSERT INTO test (quantity) VALUES (?)", params![quantity]);

        assert!(result.is_err());
    }

    #[test]
    fn test_quantity_from_sql_overflow() {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        connection
            .execute("CREATE TABLE test (quantity INTEGER NOT NULL)", params![])
            .unwrap();

        connection
            .execute("INSERT INTO test (quantity) VALUES (-1)", params![])
            .unwrap();

        let mut statement = connection
            .prepare("SELECT quantity FROM test LIMIT 1")
            .unwrap();
        let mut rows = statement.query(params![]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let quantity_from_sql = row.get::<_, Quantity>(0);

        assert!(quantity_from_sql.is_err());
    }
}
