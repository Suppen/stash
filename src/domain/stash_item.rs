use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{entity::Entity, product::ProductId, quantity::Quantity};

/// A stash item is an instance of a product in the stash
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StashItem {
    /// ID of the stash item
    id: Uuid,

    /// ID of the product this is an instance of
    product_id: ProductId,

    /// Quantity of the product in this stash item
    quantity: Quantity,

    /// Date when this stash item expires
    expiry_date: NaiveDate,
}

impl StashItem {
    pub fn new(
        id: Uuid,
        product_id: ProductId,
        quantity: Quantity,
        expiry_date: NaiveDate,
    ) -> Self {
        Self {
            id,
            product_id,
            quantity,
            expiry_date,
        }
    }

    /// The product ID of this stash item
    pub fn product_id(&self) -> &ProductId {
        &self.product_id
    }

    /// How many of this item is in the stash
    pub fn quantity(&self) -> &Quantity {
        &self.quantity
    }

    /// The date when this stash item expires
    pub fn expiry_date(&self) -> &NaiveDate {
        &self.expiry_date
    }
}

impl Entity<Uuid> for StashItem {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_id() {
        let id = Uuid::new_v4();

        let item = StashItem::new(
            id.clone(),
            ProductId::new(String::from("ID")).unwrap(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 10, 28).unwrap(),
        );

        assert_eq!(item.id(), &id);
    }

    #[test]
    fn test_product_id() {
        let product_id = ProductId::new(String::from("ID")).unwrap();

        let item = StashItem::new(
            Uuid::new_v4(),
            product_id.clone(),
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 10, 28).unwrap(),
        );

        assert_eq!(item.product_id(), &product_id);
    }

    #[test]
    fn test_quantity() {
        let quantity = Quantity::new(5).unwrap();

        let item = StashItem::new(
            Uuid::new_v4(),
            ProductId::new(String::from("ID")).unwrap(),
            quantity,
            NaiveDate::from_ymd_opt(2023, 10, 28).unwrap(),
        );

        assert_eq!(item.quantity(), &quantity);
    }

    #[test]
    fn test_expiry_date() {
        let expires = NaiveDate::from_ymd_opt(2023, 10, 28).unwrap();

        let item = StashItem::new(
            Uuid::new_v4(),
            ProductId::new(String::from("ID")).unwrap(),
            Quantity::new(1).unwrap(),
            expires,
        );

        assert_eq!(item.expiry_date(), &expires);
    }
}
