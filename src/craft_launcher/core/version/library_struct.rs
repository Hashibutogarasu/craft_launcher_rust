use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library<T> {
    /// Maven artifact name
    pub name: String,

    /// Download information for this library
    pub downloads: Option<T>,
}
