use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::{Product, StashItem},
    interfaces::web::errors::ProductParseError,
};

use super::StashItemDTO;

/// DTO for a domain Product entity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductDTO {
    pub id: String,
    pub brand: String,
    pub name: String,
    pub stash_items: Vec<StashItemDTO>,
}

impl From<Product> for ProductDTO {
    fn from(product: Product) -> Self {
        Self {
            id: product.id().to_string(),
            brand: product.brand().to_string(),
            name: product.name().to_string(),
            stash_items: product
                .stash_items()
                .into_iter()
                .map(|item| StashItemDTO::from(item.clone()))
                .collect(),
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
            dto.stash_items
                .into_iter()
                .map(|item| StashItem::try_from(item))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_dto_from_product() {
        let expected_dto = ProductDTO {
            id: "1".to_string(),
            brand: "brand".to_string(),
            name: "name".to_string(),
            stash_items: vec![
                StashItemDTO {
                    id: Uuid::new_v4().to_string(),
                    quantity: 3,
                    expiry_date: "2021-01-01".to_string(),
                },
                StashItemDTO {
                    id: Uuid::new_v4().to_string(),
                    quantity: 5,
                    expiry_date: "2021-01-01".to_string(),
                },
            ],
        };

        let product = Product::new(
            expected_dto.id.parse().unwrap(),
            expected_dto.brand.parse().unwrap(),
            expected_dto.name.as_str(),
            expected_dto
                .stash_items
                .iter()
                .map(|item| {
                    StashItem::new(
                        item.id.parse().unwrap(),
                        item.quantity.try_into().unwrap(),
                        item.expiry_date.parse().unwrap(),
                    )
                })
                .collect(),
        );

        let dto = ProductDTO::from(product);

        assert_eq!(dto.id, expected_dto.id);
        assert_eq!(dto.brand, expected_dto.brand);
        assert_eq!(dto.name, expected_dto.name);
        for stash_item in dto.stash_items.iter() {
            assert!(expected_dto.stash_items.contains(stash_item));
        }
    }

    #[test]
    fn test_product_try_from_dto() {
        let expected_product = Product::new(
            "1".parse().unwrap(),
            "brand".parse().unwrap(),
            "name",
            vec![
                StashItem::new(
                    Uuid::new_v4(),
                    3.try_into().unwrap(),
                    "2021-01-01".parse().unwrap(),
                ),
                StashItem::new(
                    Uuid::new_v4(),
                    5.try_into().unwrap(),
                    "2021-01-01".parse().unwrap(),
                ),
            ],
        );

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
            stash_items: vec![],
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
            stash_items: vec![],
        };

        let product = Product::try_from(dto);

        assert!(product.is_err());
    }
}
