use crate::domain::errors::QuantityError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quantity(u64);

/// A quantity of a product
impl Quantity {
    /// Create a new quantity from a positive integer
    ///
    /// # Parameters
    /// - `value` - The value of the quantity
    ///
    /// # Errors
    /// - `QuantityError::Zero` - The value is zero
    pub fn new(value: u64) -> Result<Self, QuantityError> {
        Quantity::try_from(value)
    }

    /// Get the value of the quantity
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)
    }
}

impl std::ops::Deref for Quantity {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<u64> for Quantity {
    type Error = QuantityError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(QuantityError::ZeroError)
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_quantity() {
        assert_eq!(Quantity::new(1), Ok(Quantity(1)));
        assert_eq!(Quantity::new(2), Ok(Quantity(2)));
        assert_eq!(Quantity::new(3), Ok(Quantity(3)));
    }

    #[test]
    fn test_new_quantity_zero() {
        assert_eq!(Quantity::new(0), Err(QuantityError::ZeroError));
    }

    #[test]
    fn test_value() {
        let quantity = Quantity::new(1).unwrap();

        assert_eq!(quantity.value(), 1);
    }

    #[test]
    fn test_deref() {
        let quantity = Quantity::new(1).unwrap();

        assert_eq!(*quantity, 1);
    }
}
