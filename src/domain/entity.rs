pub trait Entity<T> {
    /// Returns the id of the entity
    fn id(&self) -> &T;
}
