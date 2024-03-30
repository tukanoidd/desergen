use std::path::{Path, PathBuf};

use miette::Diagnostic;
use serde::Deserialize;
use thiserror::Error;

use crate::schema::{
    module_path::ModulePath, raw::RawSchema, validation_info::SchemaValidationInfo,
};

#[derive(Debug, Deserialize)]
pub struct RawSchemaInfo {
    pub name: Option<String>,
    pub file_name: Option<String>,
    pub mod_path: Option<ModulePath>,
    pub schema: RawSchema,
    pub validation: Option<SchemaValidationInfo>,
}

impl RawSchemaInfo {
    pub fn open(schemas_root: impl AsRef<Path>, mod_path: ModulePath) -> RawSchemaInfoResult<Self> {
        let path = {
            let mut path = schemas_root.as_ref().join(PathBuf::from(mod_path));

            if !path.set_extension("ron") {
                return Err(RawSchemaInfoError::FailedToSetRonExtension(path));
            }

            if !path.exists() {
                return Err(RawSchemaInfoError::DoesNotExist(path));
            }

            path
        };

        let schema_file_str = std::fs::read_to_string(&path)
            .map_err(|err| RawSchemaInfoError::IO(err, path.clone()))?;

        ron::from_str(&schema_file_str).map_err(|err| RawSchemaInfoError::RON(err, path))
    }
}

pub type RawSchemaInfoResult<T> = Result<T, RawSchemaInfoError>;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Error, Diagnostic)]
pub enum RawSchemaInfoError {
    #[error("[RawSchemaInfo] Failed to set a '.ron' extension for {0:?}")]
    FailedToSetRonExtension(PathBuf),
    #[error("[RawSchemaInfo] {0:?} does not exist")]
    DoesNotExist(PathBuf),
    #[error("[RawSchemaInfo] [IO ({1:?})] {0}")]
    IO(std::io::Error, PathBuf),
    #[error("[RawSchemaInfo] [RON ({1:?})] {0}")]
    RON(ron::de::SpannedError, PathBuf),
}
