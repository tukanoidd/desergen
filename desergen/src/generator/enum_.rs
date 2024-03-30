use convert_case::Casing;
use miette::Diagnostic;
use thiserror::Error;

use crate::schema::validation_info::EnumSchemaValidationInfo;

use super::{
    str::{EnumStringGenerator, TypeStringGenerator, UtilStringGenerator},
    EnumSchemaGenInfo,
};

pub fn generate_enum(gen_info: EnumSchemaGenInfo) -> EnumGeneratorResult<GenerateEnumResult> {
    let generator: EnumGenerator = (&gen_info).into();
    let type_output = generator.generate();

    tracing::debug!("Exports: {:?}", type_output.exports);
    tracing::debug!("File String:\n{}", type_output.file_str);

    Ok(GenerateEnumResult { type_output })
}

struct EnumGenerator<'a> {
    name: &'a String,
    variants: &'a Vec<String>,
    validation: &'a Option<EnumSchemaValidationInfo<'a>>,
}

impl<'a> From<&'a EnumSchemaGenInfo<'a>> for EnumGenerator<'a> {
    fn from(
        EnumSchemaGenInfo {
            name,
            enum_schema,
            validation,
            ..
        }: &'a EnumSchemaGenInfo<'a>,
    ) -> Self {
        Self {
            name,
            variants: enum_schema,
            validation,
        }
    }
}

impl<'a> EnumGenerator<'a> {
    fn generate(&self) -> EnumTypeOutput {
        let default_var_name = format!(
            "{}_DEFAULT",
            self.name.to_case(convert_case::Case::UpperSnake)
        );
        let default_var: &String = self
            .validation
            .as_ref()
            .and_then(|EnumSchemaValidationInfo { default, .. }| *default)
            .unwrap_or(&self.variants[0]);

        let deserialization_function_name = format!("deserialize{}", self.name);

        let file_str = String::new()
            .export()
            .enum_()
            .name(self.name)
            .variants(self.variants)
            .new_lines(2)
            .default(self.name, &default_var_name, default_var)
            .finish();

        EnumTypeOutput {
            exports: EnumTypeExports {
                ty: self.name.clone(),
                default: default_var_name,
                deserialization_function: deserialization_function_name,
            },
            file_str,
        }
    }
}

#[derive(Debug)]
struct EnumTypeOutput {
    exports: EnumTypeExports,
    file_str: String,
}

#[derive(Debug)]
pub struct EnumTypeExports {
    pub ty: String,
    pub default: String,
    pub deserialization_function: String,
}

pub struct GenerateEnumResult {
    pub type_output: EnumTypeOutput,
}

pub type EnumGeneratorResult<T> = Result<T, EnumGeneratorError>;

#[derive(Debug, Error, Diagnostic)]
pub enum EnumGeneratorError {}
