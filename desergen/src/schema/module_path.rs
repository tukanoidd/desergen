use std::{fmt::Display, path::PathBuf};

use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct ModulePath(Vec<String>);

impl Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("::"))
    }
}

impl From<ModulePath> for PathBuf {
    fn from(value: ModulePath) -> Self {
        PathBuf::from(value.0.join("/"))
    }
}

pub struct ModulePathVisitor;

impl<'de> Visitor<'de> for ModulePathVisitor {
    type Value = ModulePath;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A string in format as such: 'path::to::module'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ModulePath(v.split("::").map(Into::into).collect()))
    }
}

impl<'de> Deserialize<'de> for ModulePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ModulePathVisitor)
    }
}
