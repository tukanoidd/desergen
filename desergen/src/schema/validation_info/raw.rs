use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawSchemaValidationInfo {
    allow_undefined: Option<Vec<String>>,
    aliases: HashMap<String, Vec<String>>,
    defaults: HashMap<String, String>,
}
