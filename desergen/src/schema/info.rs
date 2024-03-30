pub mod raw;

use super::{module_path::ModulePath, validation_info::SchemaValidationInfo, Schema};

#[derive(Debug)]
pub struct SchemaInfo {
    pub name: String,
    pub file_name: String,
    pub mod_path: ModulePath,
    pub schema: Schema,
    pub validation: Option<SchemaValidationInfo>,
    pub last_updated: u128,
}
