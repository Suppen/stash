use crate::domain::{brand::Brand, entity::Entity};

use super::ProductId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Product {
    id: ProductId,
    brand: Brand,
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

    pub fn brand(&self) -> &Brand {
        &self.brand
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Entity<ProductId> for Product {
    fn id(&self) -> &ProductId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_id() {
        let product_id = ProductId::new(String::from("ID")).unwrap();

        let product = Product::new(
            product_id.clone(),
            Brand::new(String::from("Brand")).unwrap(),
            "Name",
        );

        assert_eq!(product.id(), &product_id);
    }

    #[test]
    fn test_brand() {
        let brand = Brand::new(String::from("Brand")).unwrap();

        let product = Product::new(
            ProductId::new(String::from("ID")).unwrap(),
            brand.clone(),
            "Name",
        );

        assert_eq!(product.brand(), &brand);
    }

    #[test]
    fn test_name() {
        let name = "Name";

        let product = Product::new(
            ProductId::new(String::from("ID")).unwrap(),
            Brand::new(String::from("Brand")).unwrap(),
            name,
        );

        assert_eq!(product.name(), name);
    }
}
