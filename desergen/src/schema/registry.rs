use std::{
    collections::HashMap,
    fs, io,
    path::Path,
    time::{SystemTimeError, UNIX_EPOCH},
};

use convert_case::{Case, Casing};
use miette::Diagnostic;
use thiserror::Error;
use uuid::Uuid;

use crate::schema::info::raw::RawSchemaInfo;

use super::{
    info::{
        raw::{RawSchemaInfoError, RawSchemaInfoResult},
        SchemaInfo,
    },
    module_path::ModulePath,
    raw::{RawSchema, RawSchemaMemberType},
    MapKeyType, OptType, Schema, SchemaError, SchemaMemberType,
};

#[derive(Debug, Default)]
pub struct Registry {
    mapping: HashMap<Uuid, SchemaInfo>,
}

impl Registry {
    pub fn schemas(&self) -> impl Iterator<Item = &SchemaInfo> {
        self.mapping.values()
    }

    pub fn get(&self, id: &Uuid) -> RegistryResult<&SchemaInfo> {
        self.mapping.get(id).ok_or(RegistryError::IdNotFound(*id))
    }

    pub fn process_schema_files(
        &mut self,
        schemas_root_dir: impl AsRef<Path>,
        schemas: Vec<ModulePath>,
    ) -> RegistryInitResult<()> {
        let schemas_root = schemas_root_dir.as_ref();

        tracing::info!("Reading schema files...");

        let raw_schemas = schemas
            .into_iter()
            .map(|mod_path| {
                RawSchemaInfo::open(schemas_root, mod_path.clone()).map(|raw| (mod_path, raw))
            })
            .collect::<RawSchemaInfoResult<Vec<_>>>()?;

        tracing::info!("Creating and id map for schemas...");

        let mod_path_id_mapping = HashMap::<ModulePath, Uuid>::from_iter(
            raw_schemas
                .iter()
                .map(|(mod_path, _)| (mod_path.clone(), Uuid::now_v7())),
        );

        tracing::info!("Processing schemas...");

        for (mod_path, (raw_schema_info, path)) in raw_schemas {
            let maybe_file_name = mod_path.last().clone();
            let maybe_name = maybe_file_name.to_case(Case::Pascal);

            let RawSchemaInfo {
                name,
                file_name,
                mod_path: raw_mod_path,
                schema,
                validation,
            } = raw_schema_info;

            let file_name = file_name.unwrap_or(maybe_file_name);
            let name = name.unwrap_or(maybe_name);
            let mod_path = raw_mod_path.unwrap_or(mod_path);

            tracing::info!("Processing schema for {name} ({mod_path})...");
            let schema = Self::process_schema(schema, &mod_path_id_mapping)?;
            tracing::info!("Done processing");

            let id = mod_path_id_mapping
                .get(&mod_path)
                .cloned()
                .ok_or(RegistryInitError::IdNotFound(mod_path.clone()))?;

            let file_meta = fs::metadata(path)?;
            let last_updated = file_meta
                .modified()?
                .duration_since(UNIX_EPOCH)?
                .as_millis();

            self.mapping.insert(
                id,
                SchemaInfo {
                    name,
                    file_name,
                    mod_path,
                    schema,
                    validation,
                    last_updated,
                },
            );
        }

        Ok(())
    }

    fn process_schema(
        raw_schema: RawSchema,
        mod_path_id_mapping: &HashMap<ModulePath, Uuid>,
    ) -> RegistryInitResult<Schema> {
        match raw_schema {
            RawSchema::Class(class_schema) => {
                let new_class_schema = class_schema
                    .into_iter()
                    .map(|(field_name, field_type)| {
                        Self::process_schema_member_type(field_type, mod_path_id_mapping)
                            .map(|schema_member_type| (field_name, schema_member_type))
                    })
                    .collect::<RegistryInitResult<Vec<_>>>()?
                    .into_iter()
                    .collect();

                Ok(Schema::Class(new_class_schema))
            }
            RawSchema::Enum(enum_schema) => Ok(Schema::Enum(enum_schema)),
        }
    }

    fn process_schema_member_type(
        raw_schema_member_type: RawSchemaMemberType,
        mod_path_id_mapping: &HashMap<ModulePath, Uuid>,
    ) -> RegistryInitResult<SchemaMemberType> {
        match raw_schema_member_type {
            RawSchemaMemberType::Num => Ok(SchemaMemberType::Num),
            RawSchemaMemberType::Str => Ok(SchemaMemberType::Str),
            RawSchemaMemberType::Bool => Ok(SchemaMemberType::Bool),
            RawSchemaMemberType::Arr(arr_ty) => {
                Self::process_schema_member_type(*arr_ty, mod_path_id_mapping)
                    .map(|schema_member_type| SchemaMemberType::Arr(Box::new(schema_member_type)))
            }
            RawSchemaMemberType::Map(map_key_ty, map_val_ty) => {
                let map_key_ty = MapKeyType::try_from(Self::process_schema_member_type(
                    map_key_ty.into(),
                    mod_path_id_mapping,
                )?)?;
                let map_val_ty =
                    Self::process_schema_member_type(*map_val_ty, mod_path_id_mapping)?;

                Ok(SchemaMemberType::Map(map_key_ty, Box::new(map_val_ty)))
            }
            RawSchemaMemberType::Opt(opt_ty) => Ok(SchemaMemberType::Opt(OptType::try_from(
                Self::process_schema_member_type(opt_ty.into(), mod_path_id_mapping)?,
            )?)),
            RawSchemaMemberType::DefClass(mod_path) => mod_path_id_mapping
                .get(&mod_path)
                .ok_or(RegistryInitError::IdNotFound(mod_path))
                .cloned()
                .map(SchemaMemberType::DefClass),
            RawSchemaMemberType::DefEnum(mod_path) => mod_path_id_mapping
                .get(&mod_path)
                .cloned()
                .ok_or(RegistryInitError::IdNotFound(mod_path))
                .map(SchemaMemberType::DefEnum),
        }
    }
}

pub type RegistryResult<T> = Result<T, RegistryError>;

#[derive(Debug, Error, Diagnostic)]
pub enum RegistryError {
    #[error(
        "[Registry] Failed to find schema id for '{0}' (Something is wrong and should not happen)"
    )]
    IdNotFound(Uuid),
    #[error("[Registry] {0}")]
    Init(#[from] RegistryInitError),
}

pub type RegistryInitResult<T> = Result<T, RegistryInitError>;

#[derive(Debug, Error, Diagnostic)]
pub enum RegistryInitError {
    #[error("[Init] {0}")]
    IO(#[from] io::Error),
    #[error("[Init] {0}")]
    SystemTime(#[from] SystemTimeError),
    #[error("[Init] {0}")]
    RawSchemaInfo(#[from] RawSchemaInfoError),
    #[error(
        "[Init] Failed to find schema id for '{0}' (Something is wrong and should not happen)"
    )]
    IdNotFound(ModulePath),
    #[error("[Init] {0}")]
    Schema(#[from] SchemaError),
}
