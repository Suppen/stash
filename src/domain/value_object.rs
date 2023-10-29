pub trait ValueObject<T> {
    /// The error type that is returned when the value object cannot be created
    type Error;

    /// Creates a new value object with the given value
    fn new(value: T) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Returns the value of the value object
    fn value(&self) -> &T;
}
