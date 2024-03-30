use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SchemaValidationInfo {
    pub allow_undefined: Option<Vec<String>>,
    pub aliases: HashMap<String, Vec<String>>,
    pub defaults: SchemaValidationDefaults,
}

#[derive(Debug, Deserialize)]
pub enum SchemaValidationDefaults {
    Class(HashMap<String, String>),
    Enum(String),
}
