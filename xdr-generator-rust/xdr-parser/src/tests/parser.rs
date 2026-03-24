use crate::ast::{Definition, Enum, EnumMember, Size, Struct, StructMember, Type, Typedef};
use crate::parser::parse;

#[test]
fn test_parse_struct() {
    let input = "struct Foo { int x; unsigned hyper y; };";
    let spec = parse(input).unwrap();
    assert_eq!(
        spec.definitions,
        [Definition::Struct(Struct {
            name: "Foo".to_string(),
            members: vec![
                StructMember {
                    name: "x".to_string(),
                    type_: Type::Int,
                },
                StructMember {
                    name: "y".to_string(),
                    type_: Type::UnsignedHyper,
                },
            ],
            source: "struct Foo { int x; unsigned hyper y; };".to_string(),
            is_nested: false,
            parent: None,
            file_index: 0,
        })]
    );
}

#[test]
fn test_parse_enum() {
    let input = "enum Color { RED = 0, GREEN = 1, BLUE = 2 };";
    let spec = parse(input).unwrap();
    assert_eq!(
        spec.definitions,
        [Definition::Enum(Enum {
            name: "Color".to_string(),
            members: vec![
                EnumMember {
                    name: "RED".to_string(),
                    stripped_name: "RED".to_string(),
                    value: 0,
                },
                EnumMember {
                    name: "GREEN".to_string(),
                    stripped_name: "GREEN".to_string(),
                    value: 1,
                },
                EnumMember {
                    name: "BLUE".to_string(),
                    stripped_name: "BLUE".to_string(),
                    value: 2,
                },
            ],
            member_prefix: String::new(),
            source: "enum Color { RED = 0, GREEN = 1, BLUE = 2 };".to_string(),
            file_index: 0,
        })]
    );
}

#[test]
fn test_parse_typedef() {
    let input = "typedef opaque Hash[32];";
    let spec = parse(input).unwrap();
    assert_eq!(
        spec.definitions,
        [Definition::Typedef(Typedef {
            name: "Hash".to_string(),
            type_: Type::OpaqueFixed { size: Size::Literal { literal: 32 } },
            source: "typedef opaque Hash[32];".to_string(),
            file_index: 0,
        })]
    );
}

#[test]
fn test_parse_namespace() {
    let input = "namespace stellar { struct Foo { int x; }; };";
    let spec = parse(input).unwrap();

    assert_eq!(spec.namespaces.len(), 1);
    assert_eq!(spec.namespaces[0].name, "stellar");
    assert_eq!(spec.namespaces[0].definitions.len(), 1);
}

#[test]
fn test_deeply_nested_parents_assigned_during_parse() {
    let input = r#"
        union Outer switch (int v) {
            case 0:
                struct {
                    union switch (int w) { case 0: void; } innerField;
                } outerField;
        };
    "#;
    let spec = parse(input).unwrap();

    let inner_union = spec
        .definitions
        .iter()
        .find(|d| d.name() == "OuterOuterFieldInnerField")
        .unwrap();
    assert_eq!(
        inner_union.parent(),
        Some("OuterOuterField"),
        "inner union parent should be the inline struct"
    );

    let inline_struct = spec
        .definitions
        .iter()
        .find(|d| d.name() == "OuterOuterField")
        .unwrap();
    assert_eq!(
        inline_struct.parent(),
        Some("Outer"),
        "inline struct parent should be the top-level union"
    );
}
