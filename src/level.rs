/// *Level*
///
/// Description.
pub struct Level {
    /// *name*
    ///
    /// Level name
    pub name: String,
}

impl Level {
    /// *Level*
    ///
    /// Create a new Level
    ///
    /// # Arguments:
    ///
    /// * `name` - Level name
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}
