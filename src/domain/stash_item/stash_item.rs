use chrono::NaiveDate;

use crate::domain::product::ProductId;

use super::StashItemId;

/// A stash item is an instance of a product in the stash
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StashItem {
    /// ID of the stash item
    id: StashItemId,

    /// ID of the product this is an instance of
    product_id: ProductId,

    /// Quantity of the product in this stash item
    quantity: i64,

    /// Date when this stash item expires
    expiry_date: NaiveDate,
}

impl StashItem {
    pub fn new(
        id: StashItemId,
        product_id: ProductId,
        quantity: i64,
        expiry_date: NaiveDate,
    ) -> Self {
        Self {
            id,
            product_id,
            quantity,
            expiry_date,
        }
    }

    /// The ID of this stash item
    pub fn id(&self) -> &StashItemId {
        &self.id
    }

    /// The product ID of this stash item
    pub fn product_id(&self) -> &ProductId {
        &self.product_id
    }

    /// How many of this item is in the stash
    pub fn quantity(&self) -> i64 {
        self.quantity
    }

    /// The date when this stash item expires
    pub fn expiry_date(&self) -> &NaiveDate {
        &self.expiry_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_product_id() {
        let product_id = ProductId::new("ID").unwrap();

        let item = StashItem::new(
            StashItemId::new("ID").unwrap(),
            product_id.clone(),
            1,
            NaiveDate::from_ymd_opt(2023, 10, 28).unwrap(),
        );

        assert_eq!(item.product_id(), &product_id);
    }

    #[test]
    fn test_quantity() {
        let quantity = 5;

        let item = StashItem::new(
            StashItemId::new("ID").unwrap(),
            ProductId::new("ID").unwrap(),
            quantity,
            NaiveDate::from_ymd_opt(2023, 10, 28).unwrap(),
        );

        assert_eq!(item.quantity(), quantity);
    }

    #[test]
    fn test_expiry_date() {
        let expires = NaiveDate::from_ymd_opt(2023, 10, 28).unwrap();

        let item = StashItem::new(
            StashItemId::new("ID").unwrap(),
            ProductId::new("ID").unwrap(),
            1,
            expires,
        );

        assert_eq!(item.expiry_date(), &expires);
    }
}
