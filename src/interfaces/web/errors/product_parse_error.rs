use crate::domain::errors::{BrandError, ProductIdError};

/// Errors that can occur when parsing a Product from a ProuctDTO
#[derive(Debug, PartialEq, Eq)]
pub enum ProductParseError {
    ProductIdError(ProductIdError),
    BrandError(BrandError),
}

impl std::fmt::Display for ProductParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProductIdError(error) => error.fmt(f),
            Self::BrandError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ProductParseError {}

impl From<ProductIdError> for ProductParseError {
    fn from(error: ProductIdError) -> Self {
        Self::ProductIdError(error)
    }
}

impl From<BrandError> for ProductParseError {
    fn from(error: BrandError) -> Self {
        Self::BrandError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{
            entities::Product,
            value_objects::{Brand, ProductId},
        },
        interfaces::web::dtos::ProductDTO,
    };

    #[test]
    fn test_try_from_product_dto() {
        let id = "1".parse::<ProductId>().unwrap();
        let brand = "brand".parse::<Brand>().unwrap();
        let name = "name";

        let dto = ProductDTO {
            id: id.value().to_string(),
            brand: brand.value().to_string(),
            name: name.to_string(),
        };

        let product = Product::try_from(dto).unwrap();

        assert_eq!(product.id(), &id);
        assert_eq!(product.brand(), &brand);
        assert_eq!(product.name(), name);
    }

    #[test]
    fn test_try_from_product_dto_invalid_id() {
        let dto = ProductDTO {
            id: "".to_string(),
            brand: "brand".to_string(),
            name: "name".to_string(),
        };

        let result = Product::try_from(dto);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ProductParseError::ProductIdError(ProductIdError::EmptyStringError)
        );
    }

    #[test]
    fn test_try_from_product_dto_invalid_brand() {
        let dto = ProductDTO {
            id: "1".to_string(),
            brand: "".to_string(),
            name: "name".to_string(),
        };

        let product = Product::try_from(dto);

        assert!(product.is_err());
        assert_eq!(
            product.unwrap_err(),
            ProductParseError::BrandError(BrandError::EmptyStringError)
        );
    }
}
