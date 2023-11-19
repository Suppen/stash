/// An entity in the domain. An entity is uniquely identified by its id.
pub trait Entity<T> {
    /// Returns the id of the entity
    fn id(&self) -> &T;
}
