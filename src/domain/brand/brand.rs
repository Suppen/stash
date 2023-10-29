/// Brand of a product

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Brand(String);

impl Brand {
    pub fn new(brand: &str) -> Result<Self, String> {
        if brand.is_empty() {
            Err("Brand cannot be empty".to_string())
        } else {
            Ok(Self(brand.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
