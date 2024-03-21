#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
}

use schemars::{
    gen::SchemaSettings,
    visit::{self, Visitor},
};
use std::str::FromStr;

pub struct Schema(pub schemars::schema::RootSchema);

impl TryFrom<crate::curr::TypeVariant> for Schema {
    type Error = Error;
    fn try_from(variant: crate::curr::TypeVariant) -> Result<Schema, Error> {
        println!("hhh");
        let settings = SchemaSettings::draft07().with_visitor(ReplaceAdditionalProperties);
        let generator = settings.into_generator();
        Ok(Schema(variant.json_schema(generator)))
    }
}

impl FromStr for Schema {
    type Err = Error;
    fn from_str(s: &str) -> Result<Schema, Error> {
        s.parse::<crate::curr::TypeVariant>()
            .map_err(|_| {
                Error::UnknownType(s.to_string(), &crate::curr::TypeVariant::VARIANTS_STR)
            })?
            .try_into()
    }
}

/// This visitor will remote "additionalProperties" from all objects in the schema.
#[derive(Debug, Clone)]
pub struct ReplaceAdditionalProperties;

impl Visitor for ReplaceAdditionalProperties {
    fn visit_schema_object(&mut self, schema: &mut schemars::schema::SchemaObject) {
        schema.object().additional_properties = None;
        add_titles(schema.subschemas());
        visit::visit_schema_object(self, schema);
    }
}

/// This function will add titles to all one_of schemas in the schema. So that it is more readable.
/// E.g. it was `Option 1`, etc before
fn add_titles(sub_schema: &mut schemars::schema::SubschemaValidation) {
    if let Some(ref mut one_of) = sub_schema.one_of {
        one_of.iter_mut().for_each(|schema| {
            if let schemars::schema::Schema::Object(ref mut obj) = schema {
                if let Some(inner_obj) = &mut obj.object {
                    if let Some(title) = inner_obj.required.first() {
                        obj.metadata().title = Some(title.clone());
                    }
                } else if let Some(enum_values) = &obj.enum_values {
                    if let Some(title) = enum_values.get(2) {
                        obj.metadata().title = Some(title.to_string());
                    }
                }
            }
        });
    }
}
