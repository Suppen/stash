use serde::{Deserialize, Serialize};

use crate::domain::entities::Product;

/// DTO for a domain Product entity
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    pub id: String,
    pub brand: String,
    pub name: String,
}

impl From<Product> for ProductDTO {
    fn from(product: Product) -> Self {
        Self {
            id: product.id().to_string(),
            brand: product.brand().to_string(),
            name: product.name().to_string(),
        }
    }
}

impl From<ProductDTO> for Product {
    fn from(product_dto: ProductDTO) -> Self {
        Self::new(
            product_dto.id.parse().unwrap(),
            product_dto.brand.parse().unwrap(),
            product_dto.name.as_str(),
        )
    }
}
