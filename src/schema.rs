#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
}

use schemars::gen::SchemaSettings;
use std::str::FromStr;

impl TryFrom<crate::curr::TypeVariant> for Schema {
    type Error = Error;
    fn try_from(variant: crate::curr::TypeVariant) -> Result<Schema, Error> {
        let settings = SchemaSettings::draft07();
        let generator = settings.into_generator();
        Ok(Schema(variant.json_schema(generator)))
    }
}

impl FromStr for Schema {
    type Err = Error;
    fn from_str(s: &str) -> Result<Schema, Error> {
        s.parse::<crate::curr::TypeVariant>()
            .map_err(|_| Error::UnknownType(s.to_string(), &crate::curr::TypeVariant::VARIANTS_STR))?
            .try_into()
    }
}

pub struct Schema(pub schemars::schema::RootSchema);
