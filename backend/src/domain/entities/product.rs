use std::collections::{HashMap, HashSet};

use getset::{Getters, Setters};
use uuid::Uuid;

use crate::domain::{
    errors::{StashItemDoesntExistError, StashItemExistsError},
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
    pub fn new(id: ProductId, brand: Brand, name: &str, stash_items: Vec<StashItem>) -> Self {
        Self {
            id,
            brand,
            name: name.to_string(),
            stash_items: stash_items
                .into_iter()
                .map(|item| (item.id().clone(), item))
                .collect(),
        }
    }

    /// Gets the list of stash items associated with the product. Note: No order is guaranteed.
    ///
    /// # Returns
    ///
    /// * The list of stash items associated with the product
    pub fn stash_items(&self) -> HashSet<&StashItem> {
        self.stash_items.values().collect()
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

        // TODO Check if a stash item with the same expiry date already exists

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
    ) -> Result<(), StashItemDoesntExistError> {
        // TODO Check if a stash item with the same expiry date already exists

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
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_product_id() {
        let product_id: ProductId = "ID".parse().unwrap();

        let product = Product::new(product_id.clone(), "Brand".parse().unwrap(), "Name", vec![]);

        assert_eq!(product.id(), &product_id);
    }

    #[test]
    fn test_brand() {
        let brand: Brand = "Brand".parse().unwrap();

        let product = Product::new("ID".parse().unwrap(), brand.clone(), "Name", vec![]);

        assert_eq!(product.brand(), &brand);
    }

    #[test]
    fn test_name() {
        let name = "Name";

        let product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            name,
            vec![],
        );

        assert_eq!(product.name(), name);
    }

    #[test]
    fn test_stash_items() {
        let stash_items = vec![
            StashItem::new(
                Uuid::new_v4(),
                1.try_into().unwrap(),
                NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            ),
            StashItem::new(
                Uuid::new_v4(),
                2.try_into().unwrap(),
                NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            ),
        ];

        let product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            stash_items.clone(),
        );

        for stash_item in &stash_items {
            assert!(product.stash_items().contains(&stash_item));
        }
    }

    #[test]
    fn test_stash_item_exists() {
        let stash_item = StashItem::new(
            Uuid::new_v4(),
            1.try_into().unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        let product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![stash_item.clone()],
        );

        assert!(product.has_stash_item(stash_item.id()));
        assert!(!product.has_stash_item(&Uuid::new_v4()));
    }

    #[test]
    fn test_add_stash_item_to_empty_list() {
        let mut product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![],
        );

        let stash_item = StashItem::new(
            Uuid::new_v4(),
            1.try_into().unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        let result = product.add_stash_item(stash_item.clone());

        assert!(result.is_ok());
        assert!(product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_add_stash_item_to_nonempty_list() {
        let stash_item_1 = StashItem::new(
            Uuid::new_v4(),
            1.try_into().unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        let mut product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![stash_item_1.clone()],
        );

        let stash_item_2 = StashItem::new(
            Uuid::new_v4(),
            2.try_into().unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        let result = product.add_stash_item(stash_item_2.clone());

        assert!(result.is_ok());
        assert!(product.stash_items().contains(&&stash_item_1));
        assert!(product.stash_items().contains(&&stash_item_2));
    }

    #[test]
    fn test_add_stash_item_exists_error() {
        let stash_item = StashItem::new(
            Uuid::new_v4(),
            1.try_into().unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        let mut product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![stash_item.clone()],
        );

        let result = product.add_stash_item(stash_item.clone());

        assert!(result.is_err());
        assert!(product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_remove_stash_item() {
        let stash_item = StashItem::new(
            Uuid::new_v4(),
            1.try_into().unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        let mut product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![stash_item.clone()],
        );

        let result = product.remove_stash_item(stash_item.id());

        assert!(result.is_ok());
        assert!(!product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_remove_stash_item_doesnt_exist() {
        let mut product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![],
        );

        let result = product.remove_stash_item(&Uuid::new_v4());

        assert!(result.is_err());
    }

    #[test]
    fn test_replace_stash_item() {
        let stash_item_1 = StashItem::new(
            Uuid::new_v4(),
            1.try_into().unwrap(),
            "2021-01-01".parse().unwrap(),
        );

        let mut product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![stash_item_1.clone()],
        );

        let stash_item_2 = StashItem::new(
            stash_item_1.id().clone(),
            2.try_into().unwrap(),
            "2021-01-01".parse().unwrap(),
        );

        let result = product.update_stash_item(stash_item_2.clone());

        assert!(result.is_ok());
        assert!(product.stash_items().contains(&&stash_item_2));
        assert!(!product.stash_items().contains(&&stash_item_1));
    }

    #[test]
    fn test_replace_stash_item_doesnt_exist() {
        let mut product = Product::new(
            "ID".parse().unwrap(),
            "Brand".parse().unwrap(),
            "Name",
            vec![],
        );

        let stash_item = StashItem::new(
            Uuid::new_v4(),
            2.try_into().unwrap(),
            "2021-01-01".parse().unwrap(),
        );

        let result = product.update_stash_item(stash_item.clone());

        assert!(result.is_err());
        assert!(!product.stash_items().contains(&&stash_item));
    }
}
