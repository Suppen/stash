pub trait ValueObject<T> {
    /// Creates a new value object with the given value
    fn new(value: T) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    /// Returns the value of the value object
    fn value(&self) -> &T;
}
