pub mod class;
pub mod enum_;
pub mod raw;

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SchemaValidationInfo {
    required: Option<Vec<String>>,
    aliases: HashMap<String, Vec<String>>,
    defaults: HashMap<String, String>,
}
