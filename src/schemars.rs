use schemars::{
    gen::SchemaSettings,
    schema::{InstanceType, RootSchema, Schema, SchemaObject},
    visit::{visit_schema_object, Visitor},
};

// Returns a schema settings configured for JSON Schema Draft201909.
//
// Modifies the default settings to allow additional properties on all schemas, disallow
// unevaluated properties at the root, and allow a root level property $schema that can be used to
// reference via URL the schema.
//
// The schemars crate automatically adds additionalProperties to fields like enums/unions that
// create a oneOf. Additional properties when disabled create closed types that are not extendable
// in the root document. When the root document isn't extendable it's not possible to add add the
// root $schema field. Unevaluated properties were added in draft2019_09, and are compatible with
// oneOf types sitting at the top of a document alongside document fields like $schema. Instead of
// banning any additional property within a type specifically making it closed, they allow fields
// as long as they're evaluated somewhere in the schema. Use this visitor to remove
// additionalProperties in subschemas and then to get back essentially the same behavior, add
// evaluatedProperties to the root document.
#[must_use]
pub fn draft201909() -> SchemaSettings {
    SchemaSettings::draft2019_09()
        .with_visitor(RemoveAdditionalPropertiesFalseVisitor)
        .with_visitor(AddUnevaluatedPropertiesFalseRootVisitor)
        .with_visitor(AddDollarSchemaPropertyRootVisitor)
}

// Schema visitor to remove additionalProperties: false.
//
#[derive(Debug, Clone)]
struct RemoveAdditionalPropertiesFalseVisitor;
impl Visitor for RemoveAdditionalPropertiesFalseVisitor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        if let Some(obj) = &mut schema.object {
            if let Some(additional_props) = &obj.additional_properties {
                if let Schema::Bool(false) = additional_props.as_ref() {
                    obj.additional_properties = None;
                }
            }
        }
        visit_schema_object(self, schema);
    }
}

// Schema visitor to add the unevaluatedProperties: false to the root schema.
//
// Only adds the property if the root is an object.
#[derive(Debug, Clone)]
struct AddUnevaluatedPropertiesFalseRootVisitor;
impl Visitor for AddUnevaluatedPropertiesFalseRootVisitor {
    fn visit_root_schema(&mut self, root: &mut RootSchema) {
        if root.schema.has_type(InstanceType::Object) {
            root.schema
                .extensions
                .insert("unevaluatedProperties".to_string(), false.into());
        }
    }
}

// Schema visitor to add the $schema string property to the root schema.
//
// Only adds the property if the root is an object.
#[derive(Debug, Clone)]
struct AddDollarSchemaPropertyRootVisitor;

impl Visitor for AddDollarSchemaPropertyRootVisitor {
    fn visit_root_schema(&mut self, root: &mut RootSchema) {
        if root.schema.has_type(InstanceType::Object) {
            root.schema.object().properties.insert(
                "$schema".to_string(),
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    ..Default::default()
                }
                .into(),
            );
        }
    }
}
