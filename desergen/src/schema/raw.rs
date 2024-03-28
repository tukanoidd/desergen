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
    Map(Box<RawSchemaMemberType>, Box<RawSchemaMemberType>),
    Opt(Box<RawSchemaMemberType>),
    DefEnum(ModulePath),
    DefClass(ModulePath),
}

pub type RawEnumSchema = Vec<String>;
