use serde::{Deserialize, Serialize};

use crate::{domain::entities::Product, interfaces::web::errors::ProductParseError};

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

impl TryFrom<ProductDTO> for Product {
    type Error = ProductParseError;

    fn try_from(dto: ProductDTO) -> Result<Self, Self::Error> {
        Ok(Self::new(
            dto.id.parse()?,
            dto.brand.parse()?,
            dto.name.as_str(),
        ))
    }
}
