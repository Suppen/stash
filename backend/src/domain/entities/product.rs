use std::collections::{HashMap, HashSet};

use getset::{Getters, Setters};
use uuid::Uuid;

use crate::domain::value_objects::{Brand, ProductId};

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

    /// Adds a stash item to the product. Will replace an existing stash item with the same ID.
    ///
    /// # Arguments
    ///
    /// * `stash_item` - Stash item to add
    ///
    /// # Returns
    ///
    /// * None if the item was added, Some(stash_item) if an item with the same ID was replaced
    pub fn add_or_replace_stash_item(&mut self, stash_item: StashItem) -> Option<StashItem> {
        // TODO Two stash items on a product cannot have the same expiry date
        self.stash_items.insert(stash_item.id().clone(), stash_item)
    }

    /// Removes a stash item from the product
    ///
    /// # Arguments
    ///
    /// * `stash_item` - Stash item to remove
    ///
    /// # Returns
    ///
    /// * The removed stash item, if one was removed
    pub fn remove_stash_item_by_id(&mut self, stash_item_id: &Uuid) -> Option<StashItem> {
        self.stash_items.remove(stash_item_id)
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

        let result = product.add_or_replace_stash_item(stash_item.clone());

        assert!(result.is_none());
        assert!(product.stash_items().contains(&&stash_item));
    }

    #[test]
    fn test_add_stash_item_to_non_empty_list() {
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

        let result = product.add_or_replace_stash_item(stash_item_2.clone());

        assert!(result.is_none());
        for stash_item in &[&stash_item_1, &stash_item_2] {
            assert!(product.stash_items().contains(stash_item));
        }
    }

    #[test]
    fn test_replace_stash_item() {
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

        let stash_item2 = StashItem::new(
            stash_item_1.id().clone(),
            2.try_into().unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
        );

        let result = product.add_or_replace_stash_item(stash_item2.clone());

        assert_eq!(result, Some(stash_item_1));
        assert!(product.stash_items().contains(&&stash_item2));
    }

    #[test]
    fn test_remove_stash_item() {
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

        product.add_or_replace_stash_item(stash_item.clone());

        product.remove_stash_item_by_id(stash_item.id());

        assert_eq!(product.stash_items().len(), 0);
    }
}
