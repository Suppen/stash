use serde::{Deserialize, Serialize};

use crate::{domain::entities::Product, interfaces::web::errors::ProductParseError};

/// DTO for a domain Product entity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dto_from_product() {
        let expected_dto = ProductDTO {
            id: "1".to_string(),
            brand: "brand".to_string(),
            name: "name".to_string(),
        };

        let product = Product::new(
            expected_dto.id.parse().unwrap(),
            expected_dto.brand.parse().unwrap(),
            expected_dto.name.as_str(),
        );

        let dto = ProductDTO::from(product);

        assert_eq!(dto, expected_dto);
    }

    #[test]
    fn test_product_try_from_dto() {
        let expected_product = Product::new("1".parse().unwrap(), "brand".parse().unwrap(), "name");

        let dto = ProductDTO::from(expected_product.clone());

        let product = Product::try_from(dto).unwrap();

        assert_eq!(product, expected_product);
    }

    #[test]
    fn test_product_try_from_dto_with_invalid_id() {
        let dto = ProductDTO {
            id: "".to_string(),
            brand: "brand".to_string(),
            name: "name".to_string(),
        };

        let product = Product::try_from(dto);

        assert!(product.is_err());
    }

    #[test]
    fn test_product_try_from_dto_with_invalid_brand() {
        let dto = ProductDTO {
            id: "1".to_string(),
            brand: "".to_string(),
            name: "name".to_string(),
        };

        let product = Product::try_from(dto);

        assert!(product.is_err());
    }
}
