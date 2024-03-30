use miette::Diagnostic;
use thiserror::Error;

use crate::schema::validation_info::EnumSchemaValidationInfo;

use super::{
    str::{EnumStringGenerator, StringGenerator, UtilStringGenerator},
    EnumSchemaGenInfo,
};

pub fn generate_enum(gen_info: EnumSchemaGenInfo) -> EnumGeneratorResult<GenerateEnumResult> {
    let generator: EnumGenerator = (&gen_info).into();
    let type_str = generator.generate();

    tracing::debug!("{type_str}");

    Ok(GenerateEnumResult { type_str })
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
    fn generate(&self) -> String {
        String::new()
            .export()
            .enum_()
            .name(self.name)
            .variants(self.variants)
            .finish()
    }
}

pub struct GenerateEnumResult {
    type_str: String,
}

pub type EnumGeneratorResult<T> = Result<T, EnumGeneratorError>;

#[derive(Debug, Error, Diagnostic)]
pub enum EnumGeneratorError {}
