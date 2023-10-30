#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quantity(i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityError {
    /// A quantity can not be zero
    Zero,
}

impl Quantity {
    /// Create a new quantity from a positive integer
    ///
    /// # Parameters
    /// - `value` - The value of the quantity
    ///
    /// # Errors
    /// - `QuantityError::Zero` - The value is zero
    pub fn new(value: i64) -> Result<Self, QuantityError> {
        if value == 0 {
            Err(QuantityError::Zero)
        } else {
            Ok(Self(value))
        }
    }

    /// Get the value of the quantity
    pub fn value(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_quantity() {
        assert_eq!(Quantity::new(1), Ok(Quantity(1)));
        assert_eq!(Quantity::new(2), Ok(Quantity(2)));
        assert_eq!(Quantity::new(3), Ok(Quantity(3)));
    }

    #[test]
    fn new_quantity_zero() {
        assert_eq!(Quantity::new(0), Err(QuantityError::Zero));
    }
}
