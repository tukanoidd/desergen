pub mod class;
pub mod enum_;
pub mod str;

use miette::Diagnostic;
use thiserror::Error;

use crate::schema::{
    info::SchemaInfo,
    module_path::ModulePath,
    registry::Registry,
    validation_info::{
        ClassSchemaValidationInfo, EnumSchemaValidationInfo, SchemaValidationDefaults,
        SchemaValidationInfo,
    },
    ClassSchema, EnumSchema,
};

use self::enum_::{generate_enum, EnumGeneratorError};

pub struct ClassSchemaGenInfo<'a> {
    name: &'a String,
    file_name: &'a String,
    mod_path: &'a String,
    class_schema: &'a ClassSchema,
    validation: &'a Option<ClassSchemaValidationInfo<'a>>,
}

pub struct EnumSchemaGenInfo<'a> {
    name: &'a String,
    file_name: &'a String,
    mod_path: &'a ModulePath,
    enum_schema: &'a EnumSchema,
    validation: &'a Option<EnumSchemaValidationInfo<'a>>,
}

pub fn generate(registry: &Registry) -> GeneratorResult<()> {
    for SchemaInfo {
        name,
        file_name,
        mod_path,
        schema,
        validation,
        ..
    } in registry.schemas()
    {
        match schema {
            crate::schema::Schema::Class(_) => {
                tracing::debug!("Still working on enums");
            }
            crate::schema::Schema::Enum(enum_schema) => {
                generate_enum(EnumSchemaGenInfo {
                    name,
                    file_name,
                    mod_path,
                    enum_schema,
                    validation: &validation.as_ref().map(
                        |SchemaValidationInfo {
                             aliases, defaults, ..
                         }| EnumSchemaValidationInfo {
                            aliases,
                            default: defaults.as_ref().and_then(|defaults| match defaults {
                                SchemaValidationDefaults::Enum(default) => Some(default),
                                _ => None,
                            }),
                        },
                    ),
                })?;
            }
        }
    }

    Ok(())
}

pub type GeneratorResult<T> = Result<T, GeneratorError>;

#[derive(Debug, Error, Diagnostic)]
pub enum GeneratorError {
    #[error("[Generator] {0}")]
    Enum(#[from] EnumGeneratorError),
}
