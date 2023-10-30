use getset::{Getters, Setters};

use crate::domain::{brand::Brand, entity::Entity};

use super::ProductId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, Setters)]
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
}

impl Product {
    pub fn new(id: ProductId, brand: Brand, name: &str) -> Self {
        Self {
            id,
            brand,
            name: name.to_string(),
        }
    }
}

impl Entity<ProductId> for Product {
    fn id(&self) -> &ProductId {
        &self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_id() {
        let product_id: ProductId = "ID".parse().unwrap();

        let product = Product::new(product_id.clone(), "Brand".parse().unwrap(), "Name");

        assert_eq!(product.id(), &product_id);
    }

    #[test]
    fn test_brand() {
        let brand: Brand = "Brand".parse().unwrap();

        let product = Product::new("ID".parse().unwrap(), brand.clone(), "Name");

        assert_eq!(product.brand(), &brand);
    }

    #[test]
    fn test_name() {
        let name = "Name";

        let product = Product::new("ID".parse().unwrap(), "Brand".parse().unwrap(), name);

        assert_eq!(product.name(), name);
    }
}
