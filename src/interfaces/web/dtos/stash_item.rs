use serde::{Deserialize, Serialize};

use crate::{domain::entities::StashItem, interfaces::web::errors::StashItemParseError};

/// DTO for a stash item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashItemDTO {
    pub id: String,
    pub product_id: String,
    pub quantity: u64,
    pub expiry_date: String,
}

impl From<StashItem> for StashItemDTO {
    fn from(item: StashItem) -> Self {
        Self {
            id: item.id().to_string(),
            product_id: item.product_id().to_string(),
            quantity: item.quantity().value(),
            expiry_date: item.expiry_date().to_string(),
        }
    }
}

impl TryFrom<StashItemDTO> for StashItem {
    type Error = StashItemParseError;

    fn try_from(dto: StashItemDTO) -> Result<Self, Self::Error> {
        Ok(Self::new(
            dto.id.parse()?,
            dto.product_id.parse()?,
            dto.quantity.try_into()?,
            dto.expiry_date.parse()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_dto_from_stash_item() {
        let expected_dto = StashItemDTO {
            id: Uuid::new_v4().to_string(),
            product_id: "1".to_string(),
            quantity: 3,
            expiry_date: "2021-01-01".to_string(),
        };

        let item = StashItem::new(
            expected_dto.id.parse().unwrap(),
            expected_dto.product_id.parse().unwrap(),
            expected_dto.quantity.try_into().unwrap(),
            expected_dto.expiry_date.parse().unwrap(),
        );

        let dto = StashItemDTO::from(item);

        assert_eq!(expected_dto, dto);
    }

    #[test]
    fn test_stash_item_from_dto() {
        let expected_item = StashItem::new(
            Uuid::new_v4(),
            "1".parse().unwrap(),
            3.try_into().unwrap(),
            "2021-01-01".parse().unwrap(),
        );

        let dto = StashItemDTO::from(expected_item.clone());

        let item = StashItem::try_from(dto.clone()).unwrap();

        assert_eq!(expected_item, item);
    }

    #[test]
    fn test_stash_item_from_dto_invalid_id() {
        let dto = StashItemDTO {
            id: "".to_string(),
            product_id: "1".to_string(),
            quantity: 3,
            expiry_date: "2021-01-01".to_string(),
        };

        let result = StashItem::try_from(dto);

        assert!(result.is_err());
    }

    #[test]
    fn test_stash_item_from_dto_invalid_product_id() {
        let dto = StashItemDTO {
            id: Uuid::new_v4().to_string(),
            product_id: "".to_string(),
            quantity: 3,
            expiry_date: "2021-01-01".to_string(),
        };

        let result = StashItem::try_from(dto);

        assert!(result.is_err());
    }

    #[test]
    fn test_stash_item_from_dto_invalid_quantity() {
        let dto = StashItemDTO {
            id: Uuid::new_v4().to_string(),
            product_id: "1".to_string(),
            quantity: 0,
            expiry_date: "2021-01-01".to_string(),
        };

        let result = StashItem::try_from(dto);

        assert!(result.is_err());
    }

    #[test]
    fn test_stash_item_from_dto_invalid_expiry_date() {
        let dto = StashItemDTO {
            id: Uuid::new_v4().to_string(),
            product_id: "1".to_string(),
            quantity: 3,
            expiry_date: "".to_string(),
        };

        let result = StashItem::try_from(dto);

        assert!(result.is_err());
    }
}
