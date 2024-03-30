use std::collections::HashMap;

use serde::Deserialize;

use super::module_path::ModulePath;

#[derive(Debug, Deserialize)]
pub enum RawSchema {
    Class(RawClassSchema),
    Enum(RawEnumSchema),
}

pub type RawClassSchema = HashMap<String, RawSchemaMemberType>;

#[derive(Debug, Deserialize)]
pub enum RawSchemaMemberType {
    Num,
    Str,
    Bool,
    Arr(Box<RawSchemaMemberType>),
    Map(RawMapKeyType, Box<RawSchemaMemberType>),
    Opt(RawOptType),
    DefClass(ModulePath),
    DefEnum(ModulePath),
}

#[derive(Debug, Deserialize)]
pub enum RawMapKeyType {
    Num,
    Str,
    DefEnum(ModulePath),
}

impl From<RawMapKeyType> for RawSchemaMemberType {
    fn from(value: RawMapKeyType) -> Self {
        match value {
            RawMapKeyType::Num => Self::Num,
            RawMapKeyType::Str => Self::Str,
            RawMapKeyType::DefEnum(mod_path) => Self::DefEnum(mod_path),
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum RawOptType {
    Num,
    Str,
    Bool,
    Arr(Box<RawSchemaMemberType>),
    Map(RawMapKeyType, Box<RawSchemaMemberType>),
    DefClass(ModulePath),
    DefEnum(ModulePath),
}

impl From<RawOptType> for RawSchemaMemberType {
    fn from(value: RawOptType) -> Self {
        match value {
            RawOptType::Num => Self::Num,
            RawOptType::Str => Self::Str,
            RawOptType::Bool => Self::Bool,
            RawOptType::Arr(arr_ty) => Self::Arr(arr_ty),
            RawOptType::Map(map_key_ty, map_val_ty) => Self::Map(map_key_ty, map_val_ty),
            RawOptType::DefClass(mod_path) => Self::DefClass(mod_path),
            RawOptType::DefEnum(mod_path) => Self::DefEnum(mod_path),
        }
    }
}

pub type RawEnumSchema = Vec<String>;
