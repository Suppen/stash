use chrono::NaiveDate;
use getset::{Getters, Setters};
use uuid::Uuid;

use crate::domain::value_objects::Quantity;

use super::Entity;

/// A stash item is an instance of a product in the stash
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, Setters)]
pub struct StashItem {
    /// ID of the stash item
    #[getset(get = "pub")]
    id: Uuid,

    /// Quantity of the product in this stash item
    #[getset(get = "pub", set = "pub")]
    quantity: Quantity,

    /// Date when this stash item expires
    #[getset(get = "pub", set = "pub")]
    expiry_date: NaiveDate,
}

impl StashItem {
    pub fn new(id: Uuid, quantity: Quantity, expiry_date: NaiveDate) -> Self {
        Self {
            id,
            quantity,
            expiry_date,
        }
    }
}

impl Entity<Uuid> for StashItem {
    fn id(&self) -> &Uuid {
        &self.id()
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
            Quantity::new(1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 10, 28).unwrap(),
        );

        assert_eq!(item.id(), &id);
    }

    #[test]
    fn test_quantity() {
        let quantity = Quantity::new(5).unwrap();

        let item = StashItem::new(
            Uuid::new_v4(),
            quantity,
            NaiveDate::from_ymd_opt(2023, 10, 28).unwrap(),
        );

        assert_eq!(item.quantity(), &quantity);
    }

    #[test]
    fn test_expiry_date() {
        let expires = NaiveDate::from_ymd_opt(2023, 10, 28).unwrap();

        let item = StashItem::new(Uuid::new_v4(), Quantity::new(1).unwrap(), expires);

        assert_eq!(item.expiry_date(), &expires);
    }
}
