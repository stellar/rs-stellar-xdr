use schemars::{self, gen, schema, schema::*, JsonSchema};

use super::ReadXdr;

impl<T: JsonSchema, const MAX: u32> JsonSchema for super::VecM<T, MAX> {
    fn schema_name() -> String {
        format!("VecM<{}, {}>", T::schema_name(), MAX)
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut gen::SchemaGenerator) -> schema::Schema {
        mut_array(Vec::<T>::json_schema(gen), |array| ArrayValidation {
            max_items: Some(MAX),
            ..array
        })
    }
}

impl<const MAX: u32> JsonSchema for super::BytesM<MAX> {
    fn schema_name() -> String {
        format!("BytesM<{}>", MAX)
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut gen::SchemaGenerator) -> schema::Schema {
        mut_array(Vec::<u8>::json_schema(gen), |array| ArrayValidation {
            max_items: Some(MAX),
            min_items: Some(MAX),
            ..array
        })
    }
}
impl<T: JsonSchema + ReadXdr> JsonSchema for super::Frame<T> {
    fn schema_name() -> String {
        format!("Frame<{}>", T::schema_name())
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut gen::SchemaGenerator) -> Schema {
        T::json_schema(gen)
    }
}
impl<const MAX: u32> JsonSchema for super::StringM<MAX> {
    fn schema_name() -> String {
        format!("StringM<{}>", MAX)
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut gen::SchemaGenerator) -> Schema {
        mut_string(String::json_schema(gen), |string| StringValidation {
            max_length: Some(MAX),
            ..string
        })
    }
}

pub fn mut_array(schema: Schema, f: impl FnOnce(ArrayValidation) -> ArrayValidation) -> Schema {
    if let Schema::Object(mut schema) = schema {
        if let Some(array) = schema.array.clone() {
            schema.array = Some(Box::new(f(*array)));
        }
        schema.into()
    } else {
        schema
    }
}

pub fn mut_string(schema: Schema, f: impl FnOnce(StringValidation) -> StringValidation) -> Schema {
    if let Schema::Object(mut schema) = schema {
        let string = *schema.string.unwrap_or_default().clone();
        let s = f(string);
        schema.string = Some(Box::new(s));

        schema.into()
    } else {
        schema
    }
}

#[cfg(test)]
mod test {
    use crate::curr::ScBytes;

    use super::super::{BytesM, StringM, VecM};
    use super::*;
    use schemars::JsonSchema;

    #[test]
    fn test_vecm() {
        let schema = VecM::<u32, 10>::json_schema(&mut gen::SchemaGenerator::default());
        println!("{schema:#?}");
    }

    #[test]
    fn test_bytesm() {
        let schema = BytesM::<10>::json_schema(&mut gen::SchemaGenerator::default());
        println!("{schema:#?}");
    }
    #[test]
    fn test_bytes() {
        let schema = ScBytes::json_schema(&mut gen::SchemaGenerator::default());
        println!("{schema:#?}");
    }
    #[test]
    fn test_string() {
        let schema = StringM::<42>::json_schema(&mut gen::SchemaGenerator::default());
        println!("{schema:#?}");
    }
}
