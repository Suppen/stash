use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use getset::{Getters, Setters};
use uuid::Uuid;

use crate::domain::{
    errors::{ProductRepositoryError, StashItemDoesntExistError, StashItemExistsError},
    value_objects::{Brand, ProductId},
};

use super::{Entity, StashItem};

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Product {
    /// ID of the product
    #[getset(get = "pub")]
    id: ProductId,

    /// Brand of the product
    #[getset(get = "pub", set = "pub")]
    brand: Brand,

    /// Name of the product
    #[getset(get = "pub", set = "pub")]
    name: String,

    /// Stash items of this product
    stash_items: HashMap<Uuid, StashItem>,
}

impl Product {
    /// Create a new Product
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the product
    /// * `brand` - Brand of the product
    /// * `name` - Name of the product
    ///
    /// # Returns
    ///
    /// * A new Product with the given data
    pub fn new(id: ProductId, brand: Brand, name: String, stash_items: Vec<StashItem>) -> Self {
        let mut product = Self {
            id,
            brand,
            name,
            stash_items: HashMap::new(),
        };

        for stash_item in stash_items {
            product.add_stash_item(stash_item).unwrap();
        }

        product
    }

    /// Gets an item with the given expiry date, if one exists
    ///
    /// # Arguments
    /// * `expiry_date` - Expiry date of the item to get
    ///
    /// # Returns
    /// * The item with the given expiry date, if one exists
    fn stash_item_with_expiry_date(&self, expiry_date: &NaiveDate) -> Option<&StashItem> {
        self.stash_items
            .values()
            .find(|item| item.expiry_date() == expiry_date)
    }

    /// Gets the list of stash items associated with the product. Note: No order is guaranteed.
    ///
    /// # Returns
    ///
    /// * The list of stash items associated with the product
    pub fn stash_items(&self) -> HashSet<&StashItem> {
        self.stash_items.values().collect()
    }

    /// Gets the stash item with the given ID, if it exists
    ///
    /// # Arguments
    /// * `stash_item_id` - ID of the stash item to get
    ///
    /// # Returns
    /// * The stash item with the given ID, if one exists
    pub fn stash_item(&self, stash_item_id: &Uuid) -> Option<&StashItem> {
        self.stash_items.get(stash_item_id)
    }

    /// Checks if the product has a stash item with the given ID
    ///
    /// # Arguments
    /// * `stash_item_id` - ID of the stash item to check for
    ///
    /// # Returns
    /// * True if the product has a stash item with the given ID, false otherwise
    pub fn has_stash_item(&self, stash_item_id: &Uuid) -> bool {
        self.stash_items.contains_key(stash_item_id)
    }

    /// Adds a stash item to the product
    ///
    /// # Arguments
    /// * `stash_item` - Stash item to add
    ///
    /// # Returns
    /// * Ok(()) if the item was added
    /// Err(StashItemExistsError) if an item with the same ID already exists
    pub fn add_stash_item(&mut self, stash_item: StashItem) -> Result<(), StashItemExistsError> {
        if self.has_stash_item(stash_item.id()) {
            return Err(StashItemExistsError);
        }

        if let Some(_) = self.stash_item_with_expiry_date(stash_item.expiry_date()) {
            // TODO Other error type
            return Err(StashItemExistsError);
        }

        self.stash_items.insert(stash_item.id().clone(), stash_item);

        Ok(())
    }

    /// Removes a stash item from the product
    ///
    /// # Arguments
    /// * `stash_item_id` - ID of the stash item to remove
    ///
    /// # Returns
    /// * Ok(StashItem) The removed stash item, if one was removed
    /// * Err(StashItemDoesntExistError) if no stash item with the given ID exists
    pub fn remove_stash_item(
        &mut self,
        stash_item_id: &Uuid,
    ) -> Result<StashItem, StashItemDoesntExistError> {
        match self.stash_items.remove(stash_item_id) {
            Some(stash_item) => Ok(stash_item),
            None => Err(StashItemDoesntExistError),
        }
    }

    /// Updates a stash item in the product
    ///
    /// # Arguments
    /// * `stash_item` - Stash item to update
    ///
    /// # Returns
    /// * Ok(()) if the item was updated
    /// * Err(StashItemDoesntExistError) if no stash item with the given ID exists
    pub fn update_stash_item(
        &mut self,
        stash_item: StashItem,
    ) -> Result<(), ProductRepositoryError> {
        // Check if the stash item exists
        if !self.has_stash_item(stash_item.id()) {
            return Err(ProductRepositoryError::StashItemNotFound);
        }

        // Check if a stash item on the product has the same expiry date
        match self.stash_item_with_expiry_date(stash_item.expiry_date()) {
            Some(si) => {
                // ...but not the same ID
                if si.id() != stash_item.id() {
                    return Err(ProductRepositoryError::DuplicateExpiryDateError);
                }
            }
            None => (),
        }

        self.remove_stash_item(stash_item.id())?;
        self.add_stash_item(stash_item)
            .expect("If it already exists now, the deletion didn't work");

        Ok(())
    }
}

impl Entity<ProductId> for Product {
    fn id(&self) -> &ProductId {
        &self.id()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        entities::{FakeProduct, FakeStashItem},
        value_objects::Quantity,
    };

    use super::*;

    #[test]
    fn test_product_id() {
        let product_id: ProductId = "ID".parse().unwrap();
        let product = FakeProduct::new().with_id(product_id.clone()).build();
        assert_eq!(product.id(), &product_id);
    }

    #[test]
    fn test_brand() {
        let brand: Brand = "Brand".parse().unwrap();
        let product = FakeProduct::new().with_brand(brand.clone()).build();
        assert_eq!(product.brand(), &brand);
    }

    #[test]
    fn test_name() {
        let name = "Name";
        let product = FakeProduct::new().with_name(name.to_string()).build();
        assert_eq!(product.name(), name);
    }

    #[test]
    fn test_stash_items() {
        let stash_items = vec![FakeStashItem::new().build(), FakeStashItem::new().build()];

        let product = FakeProduct::new()
            .with_stash_items(stash_items.clone())
            .build();

        for stash_item in &stash_items {
            assert!(product.stash_items().contains(&stash_item));
        }
    }

    #[test]
    fn test_stash_item() {
        let stash_item = FakeStashItem::new().build();

        let product = FakeProduct::new()
            .with_stash_items(vec![stash_item.clone()])
            .build();

        assert_eq!(product.stash_item(stash_item.id()), Some(&stash_item));
    }

    #[test]
    fn test_stash_item_doesnt_exist() {
        let product = FakeProduct::new().build();
        assert_eq!(product.stash_item(&Uuid::new_v4()), None);
    }

    #[test]
    fn test_add_stash_item_to_empty_list() {
        let mut product = FakeProduct::new().build();
        let stash_item = FakeStashItem::new().build();
        let result = product.add_stash_item(stash_item.clone());

        assert!(result.is_ok());
        assert!(product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_add_stash_item_to_nonempty_list() {
        let stash_item_1 = FakeStashItem::new().build();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![stash_item_1.clone()])
            .build();

        let stash_item_2 = FakeStashItem::new().build();
        let result = product.add_stash_item(stash_item_2.clone());

        assert!(result.is_ok());
        assert!(product.stash_items().contains(&&stash_item_1));
        assert!(product.stash_items().contains(&&stash_item_2));
    }

    #[test]
    fn test_add_stash_item_exists_error() {
        let stash_item = FakeStashItem::new().build();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![stash_item.clone()])
            .build();
        let result = product.add_stash_item(stash_item.clone());

        assert!(result.is_err());
        assert!(product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_add_stash_item_existing_expiry_date() {
        let expiry_date = NaiveDate::from_ymd_opt(2023, 11, 26).unwrap();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_expiry_date(expiry_date.clone())
                .build()])
            .build();

        let result = product.add_stash_item(
            FakeStashItem::new()
                .with_expiry_date(expiry_date.clone())
                .build(),
        );

        // TODO Check error type
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_stash_item() {
        let stash_item = FakeStashItem::new().build();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![stash_item.clone()])
            .build();
        let result = product.remove_stash_item(stash_item.id());

        assert!(result.is_ok());
        assert!(!product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_remove_stash_item_doesnt_exist() {
        let mut product = FakeProduct::new().build();
        let result = product.remove_stash_item(&Uuid::new_v4());

        assert!(result.is_err());
    }

    #[test]
    fn test_update_stash_item() {
        let stash_item_1 = FakeStashItem::new().build();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![stash_item_1.clone()])
            .build();
        let stash_item_2 = FakeStashItem::new()
            .with_id(stash_item_1.id().clone())
            .build();
        let result = product.update_stash_item(stash_item_2.clone());

        assert!(result.is_ok());
        assert!(product.stash_items().contains(&&stash_item_2));
        assert!(!product.stash_items().contains(&&stash_item_1));
    }

    #[test]
    fn test_update_stash_item_doesnt_exist() {
        let mut product = FakeProduct::new().build();
        let stash_item = FakeStashItem::new().build();
        let result = product.update_stash_item(stash_item.clone());

        assert!(result.is_err());
        assert!(!product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_update_stash_item_existing_expiry_date() {
        let stash_item_id = Uuid::new_v4();
        let expiry_date = NaiveDate::from_ymd_opt(2023, 11, 26).unwrap();
        let mut product = FakeProduct::new()
            .with_stash_items(vec![FakeStashItem::new()
                .with_id(stash_item_id.clone())
                .with_expiry_date(expiry_date.clone())
                .with_quantity(Quantity::new(2).unwrap())
                .build()])
            .build();

        let result = product.update_stash_item(
            FakeStashItem::new()
                .with_id(stash_item_id.clone())
                .with_expiry_date(expiry_date.clone())
                .with_quantity(Quantity::new(3).unwrap())
                .build(),
        );

        // TODO Check error type
        assert!(result.is_err());
    }
}
