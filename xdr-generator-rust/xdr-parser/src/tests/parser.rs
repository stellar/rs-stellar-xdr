use crate::ast::{Definition, Size, Type};
use crate::parser::parse;

#[test]
fn test_parse_struct() {
    let input = "struct Foo { int x; unsigned hyper y; };";
    let spec = parse(input).unwrap();

    assert_eq!(spec.definitions.len(), 1);
    if let Definition::Struct(s) = &spec.definitions[0] {
        assert_eq!(s.name, "Foo");
        assert_eq!(s.members.len(), 2);
        assert_eq!(s.members[0].name, "x");
        assert_eq!(s.members[0].type_, Type::Int);
        assert_eq!(s.members[1].name, "y");
        assert_eq!(s.members[1].type_, Type::UnsignedHyper);
    } else {
        panic!("Expected struct");
    }
}

#[test]
fn test_parse_enum() {
    let input = "enum Color { RED = 0, GREEN = 1, BLUE = 2 };";
    let spec = parse(input).unwrap();

    assert_eq!(spec.definitions.len(), 1);
    if let Definition::Enum(e) = &spec.definitions[0] {
        assert_eq!(e.name, "Color");
        assert_eq!(e.members.len(), 3);
        assert_eq!(e.members[0].name, "RED");
        assert_eq!(e.members[0].value, 0);
    } else {
        panic!("Expected enum");
    }
}

#[test]
fn test_parse_typedef() {
    let input = "typedef opaque Hash[32];";
    let spec = parse(input).unwrap();

    assert_eq!(spec.definitions.len(), 1);
    if let Definition::Typedef(t) = &spec.definitions[0] {
        assert_eq!(t.name, "Hash");
        assert_eq!(t.type_, Type::OpaqueFixed(Size::Literal(32)));
    } else {
        panic!("Expected typedef");
    }
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
