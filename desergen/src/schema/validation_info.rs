use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SchemaValidationInfo {
    #[serde(default)]
    pub allow_undefined: Vec<String>,
    #[serde(default)]
    pub aliases: HashMap<String, Vec<String>>,
    pub defaults: Option<SchemaValidationDefaults>,
}

#[derive(Debug, Deserialize)]
pub enum SchemaValidationDefaults {
    Class(HashMap<String, String>),
    Enum(String),
}

pub struct ClassSchemaValidationInfo<'a> {
    pub allow_undefined: &'a Vec<String>,
    pub aliases: &'a HashMap<String, Vec<String>>,
    pub defaults: &'a Option<HashMap<String, String>>,
}

pub struct EnumSchemaValidationInfo<'a> {
    pub aliases: &'a HashMap<String, Vec<String>>,
    pub default: Option<&'a String>,
}
