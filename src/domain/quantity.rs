use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quantity(u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityError {
    /// A quantity can not be zero
    Zero,
    /// An operation caused the quantity to overflow
    Overflow,
}

impl Quantity {
    /// Create a new quantity from a positive integer
    ///
    /// # Parameters
    /// - `value` - The value of the quantity
    ///
    /// # Errors
    /// - `QuantityError::Zero` - The value is zero
    pub fn new(value: u64) -> Result<Self, QuantityError> {
        if value == 0 {
            Err(QuantityError::Zero)
        } else {
            Ok(Self(value))
        }
    }

    /// Get the value of the quantity
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Add for Quantity {
    type Output = Result<Quantity, QuantityError>;

    fn add(self, rhs: Quantity) -> Self::Output {
        let new_value = self
            .value()
            .checked_add(rhs.value())
            .ok_or(QuantityError::Overflow)?;

        Quantity::new(new_value)
    }
}

impl Sub for Quantity {
    type Output = Result<Quantity, QuantityError>;

    fn sub(self, rhs: Quantity) -> Self::Output {
        let new_value = self
            .value()
            .checked_sub(rhs.value())
            .ok_or(QuantityError::Overflow)?;

        Quantity::new(new_value)
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
        assert_eq!(Quantity::new(0), Err(QuantityError::Zero));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Quantity::new(1).unwrap() + Quantity::new(2).unwrap(),
            Ok(Quantity::new(3).unwrap())
        );
    }

    #[test]
    fn test_add_overflow() {
        assert_eq!(
            Quantity::new(u64::MAX).unwrap() + Quantity::new(1).unwrap(),
            Err(QuantityError::Overflow)
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Quantity::new(3).unwrap() - Quantity::new(2).unwrap(),
            Ok(Quantity::new(1).unwrap())
        );
    }

    #[test]
    fn test_sub_overflow() {
        assert_eq!(
            Quantity::new(1).unwrap() - Quantity::new(2).unwrap(),
            Err(QuantityError::Overflow)
        );
    }

    #[test]
    fn test_sub_zero() {
        assert_eq!(
            Quantity::new(1).unwrap() - Quantity::new(1).unwrap(),
            Err(QuantityError::Zero)
        );
    }
}
