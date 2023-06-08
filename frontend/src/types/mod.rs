pub mod auth;

use std::collections::HashMap;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
