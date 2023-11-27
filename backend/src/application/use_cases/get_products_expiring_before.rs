use chrono::NaiveDate;

use crate::domain::{entities::Product, errors::ProductRepositoryError};

pub trait GetProductsExpiringBefore {
    /// Gets all products with at least one stash item expiring before the given date
    ///
    /// # Parameters
    /// - `before` - The end of the date range, exclusive
    ///
    /// # Returns
    /// A list of products with at least one stash item expiring before the given date
    fn products_expiring_before(
        &self,
        before: NaiveDate,
    ) -> Result<Vec<Product>, ProductRepositoryError>;
}
