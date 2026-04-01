use crate::parser::parse;
use crate::types::{DefKind, TypeInfo};

fn build_props(input: &str) -> crate::types::ComputedProperties {
    let spec = parse(input).unwrap();
    let type_info = TypeInfo::build(&spec, &|name| name.to_string());
    type_info.compute_properties()
}

#[test]
fn test_union_arm_name() {
    let input = r#"
        union MyUnion switch (int v) {
            case 0:
                int myField;
            case 1:
                void;
        };
    "#;
    let spec = parse(input).unwrap();
    let u = spec.definitions.iter().find(|d| d.name() == "MyUnion").unwrap();
    if let crate::ast::Definition::Union(union_def) = u {
        assert_eq!(union_def.arms.len(), 2);
        assert_eq!(union_def.arms[0].name, "myField");
        assert_eq!(union_def.arms[1].name, ""); // void arm
    } else {
        panic!("expected Union");
    }
}

#[test]
fn test_compute_properties_fixed_wire_size() {
    let input = r#"
        struct Fixed { int x; unsigned hyper y; };
        struct Variable { int x; opaque data<>; };
        enum Color { RED = 0, GREEN = 1 };
    "#;
    let spec = parse(input).unwrap();
    let type_info = TypeInfo::build(&spec, &|name| name.to_string());
    let props = type_info.compute_properties();

    // Fixed struct: int(4) + unsigned hyper(8) = 12
    let fixed = props.types.get("Fixed").unwrap();
    assert_eq!(fixed.fixed_wire_size, Some(12));
    assert_eq!(fixed.kind, DefKind::Struct);

    // Variable struct: has opaque<>, no fixed size
    let var = props.types.get("Variable").unwrap();
    assert_eq!(var.fixed_wire_size, None);
    assert_eq!(var.kind, DefKind::Struct);

    // Enum: always 4 bytes
    let color = props.types.get("Color").unwrap();
    assert_eq!(color.fixed_wire_size, Some(4));
    assert_eq!(color.kind, DefKind::Enum);
}

#[test]
fn test_compute_wire_size_union_fixed() {
    let input = r#"
        union U switch (int v) {
            case 0: int a;
            case 1: int b;
        };
    "#;
    let props = build_props(input);
    // Both arms are int (4 bytes), discriminant is int (4 bytes) = 8
    assert_eq!(props.types.get("U").unwrap().fixed_wire_size, Some(8));
}

#[test]
fn test_compute_wire_size_union_variable() {
    let input = r#"
        union U switch (int v) {
            case 0: int a;
            case 1: opaque b<>;
        };
    "#;
    let props = build_props(input);
    // One arm is variable-size -> union is variable
    assert_eq!(props.types.get("U").unwrap().fixed_wire_size, None);
}

#[test]
fn test_compute_wire_size_union_different_arm_sizes() {
    let input = r#"
        union U switch (int v) {
            case 0: int a;
            case 1: hyper b;
        };
    "#;
    let props = build_props(input);
    // Arms have different sizes (4 vs 8) -> not fixed
    assert_eq!(props.types.get("U").unwrap().fixed_wire_size, None);
}

#[test]
fn test_compute_wire_size_opaque_padding() {
    let input = "typedef opaque Hash[32];";
    let props = build_props(input);
    // 32 bytes, already 4-byte aligned
    assert_eq!(props.types.get("Hash").unwrap().fixed_wire_size, Some(32));
}

#[test]
fn test_compute_wire_size_opaque_padding_unaligned() {
    let input = "typedef opaque ShortHash[3];";
    let props = build_props(input);
    // 3 bytes padded to 4
    assert_eq!(props.types.get("ShortHash").unwrap().fixed_wire_size, Some(4));
}

#[test]
fn test_inline_struct_arm_name() {
    let input = r#"
        union Outer switch (int v) {
            case 0:
                struct { int x; } myInlineField;
        };
    "#;
    let spec = parse(input).unwrap();
    let u = spec.definitions.iter().find(|d| d.name() == "Outer").unwrap();
    if let crate::ast::Definition::Union(union_def) = u {
        assert_eq!(union_def.arms[0].name, "myInlineField");
    } else {
        panic!("expected Union");
    }
}

#[test]
fn test_compute_properties_kind_values() {
    let input = r#"
        const MAX = 10;
        typedef int MyInt;
        struct S { int x; };
        enum E { A = 0 };
        union U switch (int v) { case 0: void; };
    "#;
    let props = build_props(input);
    assert_eq!(props.types.get("MAX").unwrap().kind, DefKind::Const);
    assert_eq!(props.types.get("MyInt").unwrap().kind, DefKind::Typedef);
    assert_eq!(props.types.get("S").unwrap().kind, DefKind::Struct);
    assert_eq!(props.types.get("E").unwrap().kind, DefKind::Enum);
    assert_eq!(props.types.get("U").unwrap().kind, DefKind::Union);
}

#[test]
fn test_compute_wire_size_fixed_array() {
    let input = r#"
        struct Pair { int a; int b; };
        struct Container { Pair items[3]; };
    "#;
    let props = build_props(input);
    // Pair = 8 bytes, items[3] = 24 bytes, Container = 24
    assert_eq!(props.types.get("Pair").unwrap().fixed_wire_size, Some(8));
    assert_eq!(props.types.get("Container").unwrap().fixed_wire_size, Some(24));
}

#[test]
fn test_compute_wire_size_typedef_chain() {
    let input = r#"
        typedef int MyInt;
        typedef MyInt AnotherInt;
        struct S { AnotherInt x; };
    "#;
    let props = build_props(input);
    assert_eq!(props.types.get("MyInt").unwrap().fixed_wire_size, Some(4));
    assert_eq!(props.types.get("AnotherInt").unwrap().fixed_wire_size, Some(4));
    assert_eq!(props.types.get("S").unwrap().fixed_wire_size, Some(4));
}

#[test]
fn test_union_arm_name_multi_case() {
    let input = r#"
        enum T { A = 0, B = 1, C = 2 };
        union U switch (T v) {
            case A:
            case B:
                int sharedField;
            case C:
                void;
        };
    "#;
    let spec = parse(input).unwrap();
    let u = spec.definitions.iter().find(|d| d.name() == "U").unwrap();
    if let crate::ast::Definition::Union(union_def) = u {
        // Cases A and B share one arm
        assert_eq!(union_def.arms[0].cases.len(), 2);
        assert_eq!(union_def.arms[0].name, "sharedField");
        // Case C is void
        assert_eq!(union_def.arms[1].cases.len(), 1);
        assert_eq!(union_def.arms[1].name, "");
    } else {
        panic!("expected Union");
    }
}

