use super::ProductId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Product {
    id: ProductId,
    brand: String,
    name: String,
}

impl Product {
    pub fn new(id: ProductId, brand: &str, name: &str) -> Self {
        Self {
            id,
            brand: brand.to_string(),
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn brand(&self) -> &str {
        &self.brand
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_id() {
        let product_id = ProductId::new("ID").unwrap();
        let brand = "Brand";

        let product = Product::new(product_id.clone(), brand, "Name");

        assert_eq!(product.id(), &product_id);
    }

    #[test]
    fn test_brand() {
        let product_id = ProductId::new("ID").unwrap();
        let brand = "Brand";

        let product = Product::new(product_id, brand, "Name");

        assert_eq!(&product.brand(), &brand);
    }

    #[test]
    fn test_name() {
        let product_id = ProductId::new("ID").unwrap();
        let brand = "Brand";

        let product = Product::new(product_id, brand, "Name");

        assert_eq!(product.name(), "Name");
    }
}
