use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::value_objects::Quantity;

use super::StashItem;

/// A fake stash item builder
#[derive(Debug)]
pub struct FakeStashItem {
    id: Option<Uuid>,
    quantity: Option<Quantity>,
    expiry_date: Option<NaiveDate>,
}

impl FakeStashItem {
    pub fn new() -> Self {
        Self {
            id: None,
            quantity: None,
            expiry_date: None,
        }
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn with_expiry_date(mut self, expiry_date: NaiveDate) -> Self {
        self.expiry_date = Some(expiry_date);
        self
    }

    fn random_date() -> NaiveDate {
        use rand::distributions::Uniform;
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let range = Uniform::new_inclusive(2020, 2025);
        let year = rng.sample(range);
        let range = Uniform::new_inclusive(1, 12);
        let month = rng.sample(range);
        let range = Uniform::new_inclusive(1, 28);
        let day = rng.sample(range);
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    pub fn build(self) -> StashItem {
        StashItem::new(
            self.id.unwrap_or_else(Uuid::new_v4),
            self.quantity.unwrap_or_else(Quantity::random),
            self.expiry_date.unwrap_or_else(FakeStashItem::random_date),
        )
    }
}
