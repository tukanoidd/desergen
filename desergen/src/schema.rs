pub mod info;
pub mod module_path;
pub mod raw;
pub mod registry;
pub mod validation_info;

use std::collections::HashMap;

use miette::Diagnostic;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug)]
pub enum Schema {
    Class(ClassSchema),
    Enum(EnumSchema),
}

pub type ClassSchema = HashMap<String, SchemaMemberType>;

#[derive(Debug)]
pub enum SchemaMemberType {
    Num,
    Str,
    Bool,
    Arr(Box<SchemaMemberType>),
    Map(MapKeyType, Box<SchemaMemberType>),
    Opt(OptType),
    DefClass(Uuid),
    DefEnum(Uuid),
}

#[derive(Debug)]
pub enum MapKeyType {
    Num,
    Str,
    DefEnum(Uuid),
}

impl TryFrom<SchemaMemberType> for MapKeyType {
    type Error = SchemaError;

    fn try_from(value: SchemaMemberType) -> Result<Self, Self::Error> {
        match value {
            SchemaMemberType::Num => Ok(Self::Num),
            SchemaMemberType::Str => Ok(Self::Str),
            SchemaMemberType::DefEnum(mod_path) => Ok(Self::DefEnum(mod_path)),
            schema_member_type => Err(SchemaError::SMTMapKey(schema_member_type)),
        }
    }
}

#[derive(Debug)]
pub enum OptType {
    Num,
    Str,
    Bool,
    Arr(Box<SchemaMemberType>),
    Map(MapKeyType, Box<SchemaMemberType>),
    DefClass(Uuid),
    DefEnum(Uuid),
}

impl TryFrom<SchemaMemberType> for OptType {
    type Error = SchemaError;

    fn try_from(value: SchemaMemberType) -> Result<Self, Self::Error> {
        match value {
            SchemaMemberType::Num => Ok(Self::Num),
            SchemaMemberType::Str => Ok(Self::Str),
            SchemaMemberType::Bool => Ok(Self::Bool),
            SchemaMemberType::Arr(arr_ty) => Ok(Self::Arr(arr_ty)),
            SchemaMemberType::Map(map_key_ty, map_val_ty) => Ok(Self::Map(map_key_ty, map_val_ty)),
            SchemaMemberType::DefClass(mod_path) => Ok(Self::DefClass(mod_path)),
            SchemaMemberType::DefEnum(mod_path) => Ok(Self::DefEnum(mod_path)),
            schema_member_type => Err(SchemaError::SMTOpt(schema_member_type)),
        }
    }
}

pub type EnumSchema = Vec<String>;

#[derive(Debug, Error, Diagnostic)]
pub enum SchemaError {
    #[error("[Schema] {0:?} is either Num, Str or DefEnum")]
    SMTMapKey(SchemaMemberType),
    #[error("[Schema] {0:?} is either Num, Str, Bool, Arr, Map, DefClass or DefEnum")]
    SMTOpt(SchemaMemberType),
}
