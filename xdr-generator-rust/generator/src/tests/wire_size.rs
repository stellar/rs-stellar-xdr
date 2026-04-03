use crate::ir;

fn wire_sizes(input: &str) -> std::collections::HashMap<String, Option<u32>> {
    let spec = xdr_parser::parser::parse(input).unwrap();
    let ir = ir::build(&spec, Vec::new());
    let mut sizes = std::collections::HashMap::new();
    for def in &ir.definitions {
        let (name, fixed_size) = match def {
            ir::Definition::Struct(s) => (&s.name, s.fixed_size),
            ir::Definition::Union(u) => (&u.name, u.fixed_size),
            _ => continue,
        };
        sizes.insert(name.clone(), fixed_size);
    }
    sizes
}

#[test]
fn test_fixed_wire_size() {
    let input = r#"
        struct Fixed { int x; unsigned hyper y; };
        struct Variable { int x; opaque data<>; };
    "#;
    let sizes = wire_sizes(input);
    assert_eq!(sizes["Fixed"], Some(12));
    assert_eq!(sizes["Variable"], None);
}

#[test]
fn test_union_fixed() {
    let input = r#"
        union U switch (int v) {
            case 0: int a;
            case 1: int b;
        };
    "#;
    let sizes = wire_sizes(input);
    assert_eq!(sizes["U"], Some(8));
}

#[test]
fn test_union_variable() {
    let input = r#"
        union U switch (int v) {
            case 0: int a;
            case 1: opaque b<>;
        };
    "#;
    let sizes = wire_sizes(input);
    assert_eq!(sizes["U"], None);
}

#[test]
fn test_union_different_arm_sizes() {
    let input = r#"
        union U switch (int v) {
            case 0: int a;
            case 1: hyper b;
        };
    "#;
    let sizes = wire_sizes(input);
    assert_eq!(sizes["U"], None);
}

#[test]
fn test_opaque_padding() {
    let input = "typedef opaque Hash[32];";
    let ir = ir::build(&xdr_parser::parser::parse(input).unwrap(), Vec::new());
    // Check via the TypeRef size on the typedef
    if let ir::Definition::Typedef(t) = &ir.definitions[0] {
        if let ir::TypeRef::OpaqueFixed { size } = &t.type_ {
            assert_eq!(*size, 32);
        } else {
            panic!("expected OpaqueFixed");
        }
    }
}

#[test]
fn test_opaque_padding_unaligned() {
    let input = "typedef opaque ShortHash[3];";
    let ir = ir::build(&xdr_parser::parser::parse(input).unwrap(), Vec::new());
    if let ir::Definition::Typedef(t) = &ir.definitions[0] {
        if let ir::TypeRef::OpaqueFixed { size } = &t.type_ {
            assert_eq!(*size, 3); // raw size, consumer computes padding
        } else {
            panic!("expected OpaqueFixed");
        }
    }
}

#[test]
fn test_fixed_array() {
    let input = r#"
        struct Pair { int a; int b; };
        struct Container { Pair items[3]; };
    "#;
    let sizes = wire_sizes(input);
    assert_eq!(sizes["Pair"], Some(8));
    assert_eq!(sizes["Container"], Some(24));
}

#[test]
fn test_typedef_chain() {
    let input = r#"
        typedef int MyInt;
        typedef MyInt AnotherInt;
        struct S { AnotherInt x; };
    "#;
    let sizes = wire_sizes(input);
    assert_eq!(sizes["S"], Some(4));
}
