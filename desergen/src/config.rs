use std::path::{Path, PathBuf};

use miette::Diagnostic;
use serde::Deserialize;
use thiserror::Error;

use crate::schema::module_path::ModulePath;

pub type ConfigResult<T> = Result<T, ConfigError>;

macro_rules! default_path_funcs {
    ($($ident:ident = $path:literal),+ $(,)*) => {
        paste::paste! {$(
            fn [< default_ $ident >]() -> std::path::PathBuf {
                PathBuf::from($path)
            }
        )+}
    };
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_src_root")]
    pub src_root: PathBuf,
    #[serde(default = "Config::default_desergen_root")]
    pub desergen_root: PathBuf,
    #[serde(default = "Config::default_src_output_root")]
    pub src_output_root: PathBuf,
    pub schemas: Vec<ModulePath>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ConfigError::DoesNotExist(path.into()));
        }

        let ext = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ConfigFormat::try_from(ext)?,
            None => return Err(ConfigError::NoFileExtension(ConfigFormat::ALL)),
        };
        let config_str = std::fs::read_to_string(path)?;

        Ok(match ext {
            ConfigFormat::TOML => toml::from_str(&config_str)?,
            ConfigFormat::JSON => serde_json::from_str(&config_str)?,
            ConfigFormat::YAML => serde_yaml::from_str(&config_str)?,
        })
    }

    default_path_funcs![
        src_root = "src",
        desergen_root = "desergen",
        src_output_root = "desergen",
    ];
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum ConfigFormat {
    TOML,
    JSON,
    YAML,
}

pub type AllConfigFormats = [ConfigFormat; 3];

impl ConfigFormat {
    const ALL: AllConfigFormats = [ConfigFormat::TOML, ConfigFormat::JSON, ConfigFormat::YAML];
}

impl<'a> TryFrom<&'a str> for ConfigFormat {
    type Error = ConfigError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "toml" | "TOML" => Self::TOML,
            "json" | "JSON" => Self::JSON,
            "yaml" | "YAML" => Self::YAML,
            ext => return Err(ConfigError::WrongFormat(ext.into(), Self::ALL)),
        })
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Error, Diagnostic)]
pub enum ConfigError {
    #[error("[Config] {0}")]
    IO(#[from] std::io::Error),
    #[error("[Config] {0}")]
    TOML(#[from] toml::de::Error),
    #[error("[Config] {0}")]
    JSON(#[from] serde_json::Error),
    #[error("[Config] {0}")]
    YAML(#[from] serde_yaml::Error),
    #[error("[Config] {0:?} does not exist")]
    DoesNotExist(PathBuf),
    #[error("[Config] No file extensions determined, available formats: {0:?}")]
    NoFileExtension(AllConfigFormats),
    #[error("[Config] Wrong Format {0}, Available ones: {1:?}")]
    WrongFormat(String, AllConfigFormats),
}
