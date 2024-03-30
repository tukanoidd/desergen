use std::{fmt::Display, path::PathBuf};

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePath(Vec<String>);

impl ModulePath {
    pub fn last(&self) -> &String {
        self.0.last().unwrap()
    }
}

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
        let components = v.split("::").map(Into::into).collect::<Vec<String>>();

        match components.len() {
            0 => return Err(Error::custom("Module path is empty!")),
            1 => match components[0].is_empty() || components[0].chars().all(char::is_whitespace) {
                true => return Err(Error::custom("Module path is empty!")),
                false => {}
            },
            _ => {}
        }

        Ok(ModulePath(components))
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
