use crate::domain::{
    entities::FakeStashItem,
    value_objects::{Brand, ProductId},
};

use super::{Product, StashItem};

pub struct FakeProduct {
    id: Option<ProductId>,
    brand: Option<Brand>,
    name: Option<String>,
    stash_items: Option<Vec<StashItem>>,
}

impl FakeProduct {
    pub fn new() -> Self {
        Self {
            id: None,
            brand: None,
            name: None,
            stash_items: None,
        }
    }

    pub fn with_id(mut self, id: ProductId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_brand(mut self, brand: Brand) -> Self {
        self.brand = Some(brand);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_stash_items(mut self, stash_items: Vec<StashItem>) -> Self {
        self.stash_items = Some(stash_items);
        self
    }

    fn random_name() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let length = rng.gen_range(5..10);
        rng.sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    fn random_stash_items() -> Vec<StashItem> {
        use rand::distributions::Uniform;
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let range = Uniform::new_inclusive(1, 3);
        let length = rng.sample(range);
        let mut stash_items = Vec::with_capacity(length);
        for _ in 0..length {
            let stash_item = FakeStashItem::new().build();
            stash_items.push(stash_item);
        }
        stash_items
    }

    pub fn build(self) -> Product {
        Product::new(
            self.id.unwrap_or_else(ProductId::random),
            self.brand.unwrap_or_else(Brand::random),
            self.name.unwrap_or_else(FakeProduct::random_name).as_str(),
            self.stash_items
                .unwrap_or_else(FakeProduct::random_stash_items),
        )
    }
}
