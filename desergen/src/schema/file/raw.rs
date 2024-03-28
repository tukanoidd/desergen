use std::path::{Path, PathBuf};

use miette::Diagnostic;
use serde::Deserialize;
use thiserror::Error;

use crate::schema::{
    module_path::ModulePath, raw::RawSchema, validation_info::raw::RawSchemaValidationInfo,
};

#[derive(Debug, Deserialize)]
pub struct RawSchemaFile {
    name: Option<String>,
    file_name: Option<String>,
    mod_path: Option<ModulePath>,
    schema: RawSchema,
    validation: Option<RawSchemaValidationInfo>,
}

impl RawSchemaFile {
    pub fn open(schemas_root: impl AsRef<Path>, mod_path: ModulePath) -> RawSchemaFileResult<Self> {
        let path = {
            let mut path = schemas_root.as_ref().join(PathBuf::from(mod_path));

            if !path.set_extension("ron") {
                return Err(RawSchemaFileError::FailedToSetRonExtension(path));
            }

            if !path.exists() {
                return Err(RawSchemaFileError::DoesNotExist(path));
            }

            path
        };

        let schema_file_str = std::fs::read_to_string(&path)
            .map_err(|err| RawSchemaFileError::IO(err, path.clone()))?;

        ron::from_str(&schema_file_str).map_err(|err| RawSchemaFileError::RON(err, path))
    }
}

pub type RawSchemaFileResult<T> = Result<T, RawSchemaFileError>;

#[derive(Debug, Error, Diagnostic)]
pub enum RawSchemaFileError {
    #[error("[RawSchemaFile] Failed to set a '.ron' extension for {0:?}")]
    FailedToSetRonExtension(PathBuf),
    #[error("[RawSchemaFile] {0:?} does not exist")]
    DoesNotExist(PathBuf),
    #[error("[RawSchemaFile] [IO ({1:?})] {0}")]
    IO(std::io::Error, PathBuf),
    #[error("[RawSchemaFile] [RON ({1:?})] {0}")]
    RON(ron::de::SpannedError, PathBuf),
}
