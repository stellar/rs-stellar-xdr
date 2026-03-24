use crate::ast::{
    CfgExpr, Definition, Enum, EnumMember, Size, Struct, StructMember, Type, Typedef, Union,
    UnionArm, UnionCase, UnionCaseValue, UnionDiscriminant,
};
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
            cfg: None,
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
                    cfg: None,
                },
                EnumMember {
                    name: "GREEN".to_string(),
                    stripped_name: "GREEN".to_string(),
                    value: 1,
                    cfg: None,
                },
                EnumMember {
                    name: "BLUE".to_string(),
                    stripped_name: "BLUE".to_string(),
                    value: 2,
                    cfg: None,
                },
            ],
            member_prefix: String::new(),
            source: "enum Color { RED = 0, GREEN = 1, BLUE = 2 };".to_string(),
            file_index: 0,
            cfg: None,
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
            cfg: None,
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

// =============================================================================
// #ifdef / #else / #endif tests
// =============================================================================

#[test]
fn test_ifdef_simple() {
    let input = r#"
        #ifdef FEATURE_X
        struct Foo { int x; };
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 1);
    assert_eq!(spec.definitions[0].name(), "Foo");
    assert_eq!(
        spec.definitions[0].cfg(),
        Some(&CfgExpr::Feature("FEATURE_X".to_string()))
    );
}

#[test]
fn test_ifdef_else() {
    let input = r#"
        #ifdef FEATURE_X
        struct Foo { int x; };
        #else
        struct Bar { int y; };
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 2);

    assert_eq!(spec.definitions[0].name(), "Foo");
    assert_eq!(
        spec.definitions[0].cfg(),
        Some(&CfgExpr::Feature("FEATURE_X".to_string()))
    );

    assert_eq!(spec.definitions[1].name(), "Bar");
    assert_eq!(
        spec.definitions[1].cfg(),
        Some(&CfgExpr::Not(Box::new(CfgExpr::Feature(
            "FEATURE_X".to_string()
        ))))
    );
}

#[test]
fn test_ifdef_multiple_definitions() {
    let input = r#"
        #ifdef FEATURE_X
        struct Foo { int x; };
        struct Bar { int y; };
        const MAX_SIZE = 100;
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 3);

    for def in &spec.definitions {
        assert_eq!(def.cfg(), Some(&CfgExpr::Feature("FEATURE_X".to_string())));
    }
}

#[test]
fn test_ifdef_mixed_with_unconditional() {
    let input = r#"
        struct Always { int x; };
        #ifdef FEATURE_X
        struct Sometimes { int y; };
        #endif
        struct AlsoAlways { int z; };
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 3);

    assert_eq!(spec.definitions[0].name(), "Always");
    assert_eq!(spec.definitions[0].cfg(), None);

    assert_eq!(spec.definitions[1].name(), "Sometimes");
    assert_eq!(
        spec.definitions[1].cfg(),
        Some(&CfgExpr::Feature("FEATURE_X".to_string()))
    );

    assert_eq!(spec.definitions[2].name(), "AlsoAlways");
    assert_eq!(spec.definitions[2].cfg(), None);
}

#[test]
fn test_ifdef_nested() {
    let input = r#"
        #ifdef FEATURE_A
        #ifdef FEATURE_B
        struct Both { int x; };
        #endif
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 1);
    assert_eq!(spec.definitions[0].name(), "Both");
    assert_eq!(
        spec.definitions[0].cfg(),
        Some(&CfgExpr::All(vec![
            CfgExpr::Feature("FEATURE_A".to_string()),
            CfgExpr::Feature("FEATURE_B".to_string()),
        ]))
    );
}

#[test]
fn test_ifdef_const() {
    let input = r#"
        #ifdef FEATURE_X
        const MAX_SIZE = 100;
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 1);
    assert_eq!(spec.definitions[0].name(), "MAX_SIZE");
    assert_eq!(
        spec.definitions[0].cfg(),
        Some(&CfgExpr::Feature("FEATURE_X".to_string()))
    );
}

#[test]
fn test_ifdef_enum() {
    let input = r#"
        #ifdef FEATURE_X
        enum Color { RED = 0, GREEN = 1 };
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 1);
    assert_eq!(spec.definitions[0].name(), "Color");
    assert_eq!(
        spec.definitions[0].cfg(),
        Some(&CfgExpr::Feature("FEATURE_X".to_string()))
    );
}

#[test]
fn test_ifdef_typedef() {
    let input = r#"
        #ifdef FEATURE_X
        typedef opaque Hash[32];
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 1);
    assert_eq!(spec.definitions[0].name(), "Hash");
    assert_eq!(
        spec.definitions[0].cfg(),
        Some(&CfgExpr::Feature("FEATURE_X".to_string()))
    );
}

#[test]
fn test_ifdef_union() {
    let input = r#"
        #ifdef FEATURE_X
        union Foo switch (int v) {
            case 0: void;
        };
        #endif
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 1);
    assert_eq!(spec.definitions[0].name(), "Foo");
    assert_eq!(
        spec.definitions[0].cfg(),
        Some(&CfgExpr::Feature("FEATURE_X".to_string()))
    );
}

#[test]
fn test_ifdef_empty_block() {
    let input = r#"
        #ifdef FEATURE_X
        #endif
        struct Foo { int x; };
    "#;
    let spec = parse(input).unwrap();
    assert_eq!(spec.definitions.len(), 1);
    assert_eq!(spec.definitions[0].name(), "Foo");
    assert_eq!(spec.definitions[0].cfg(), None);
}

#[test]
fn test_ifdef_nested_types_inherit_cfg() {
    let input = r#"
        #ifdef FEATURE_X
        union Outer switch (int v) {
            case 0:
                struct { int x; } innerField;
        };
        #endif
    "#;
    let spec = parse(input).unwrap();

    // Both the outer union and the extracted inner struct should have the cfg
    let cfg = CfgExpr::Feature("FEATURE_X".to_string());
    for def in &spec.definitions {
        assert_eq!(
            def.cfg(),
            Some(&cfg),
            "definition '{}' should have cfg",
            def.name()
        );
    }
}

#[test]
fn test_stray_else_error() {
    let input = "#else\nstruct Foo { int x; };";
    let result = parse(input);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("else"), "error should mention #else: {err}");
}

#[test]
fn test_stray_endif_error() {
    let input = "#endif";
    let result = parse(input);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("endif"), "error should mention #endif: {err}");
}

// =============================================================================
// Inline #ifdef inside enum/union body tests
// =============================================================================

#[test]
fn test_ifdef_inline_enum_members() {
    let input = r#"
        enum Color {
            RED = 0,
            #ifdef FEATURE_X
            GREEN = 1,
            #else
            BLUE = 2,
            #endif
            YELLOW = 3
        };
    "#;
    let spec = parse(input).unwrap();
    let Definition::Enum(e) = &spec.definitions[0] else {
        panic!("expected enum");
    };
    assert_eq!(e.members.len(), 4);

    assert_eq!(e.members[0].stripped_name, "RED");
    assert_eq!(e.members[0].cfg, None);

    assert_eq!(e.members[1].stripped_name, "GREEN");
    assert_eq!(
        e.members[1].cfg,
        Some(CfgExpr::Feature("FEATURE_X".to_string()))
    );

    assert_eq!(e.members[2].stripped_name, "BLUE");
    assert_eq!(
        e.members[2].cfg,
        Some(CfgExpr::Not(Box::new(CfgExpr::Feature(
            "FEATURE_X".to_string()
        ))))
    );

    assert_eq!(e.members[3].stripped_name, "YELLOW");
    assert_eq!(e.members[3].cfg, None);
}

#[test]
fn test_ifdef_inline_enum_no_else() {
    let input = r#"
        enum Foo {
            A = 0,
            #ifdef FEATURE_X
            B = 1
            #endif
        };
    "#;
    let spec = parse(input).unwrap();
    let Definition::Enum(e) = &spec.definitions[0] else {
        panic!("expected enum");
    };
    assert_eq!(e.members.len(), 2);
    assert_eq!(e.members[0].cfg, None);
    assert_eq!(
        e.members[1].cfg,
        Some(CfgExpr::Feature("FEATURE_X".to_string()))
    );
}

#[test]
fn test_ifdef_inline_enum_nested() {
    let input = r#"
        enum Foo {
            #ifdef A
            #ifdef B
            X = 0
            #endif
            #endif
        };
    "#;
    let spec = parse(input).unwrap();
    let Definition::Enum(e) = &spec.definitions[0] else {
        panic!("expected enum");
    };
    assert_eq!(e.members.len(), 1);
    assert_eq!(
        e.members[0].cfg,
        Some(CfgExpr::All(vec![
            CfgExpr::Feature("A".to_string()),
            CfgExpr::Feature("B".to_string()),
        ]))
    );
}

#[test]
fn test_ifdef_inline_union_arms() {
    let input = r#"
        enum MsgType { A = 0, B = 1, C = 2 };
        union Msg switch (MsgType t) {
            case A:
                int x;
            #ifdef FEATURE_X
            case B:
                int y;
            #else
            case C:
                void;
            #endif
        };
    "#;
    let spec = parse(input).unwrap();
    let Definition::Union(u) = &spec.definitions[1] else {
        panic!("expected union");
    };
    assert_eq!(u.arms.len(), 3);
    assert_eq!(u.arms[0].cfg, None);
    assert_eq!(
        u.arms[1].cfg,
        Some(CfgExpr::Feature("FEATURE_X".to_string()))
    );
    assert_eq!(
        u.arms[2].cfg,
        Some(CfgExpr::Not(Box::new(CfgExpr::Feature(
            "FEATURE_X".to_string()
        ))))
    );
}

#[test]
fn test_ifdef_inline_enum_unclosed_error() {
    let input = r#"
        enum Foo {
            A = 0,
            #ifdef FEATURE_X
            B = 1
        };
    "#;
    let result = parse(input);
    assert!(result.is_err(), "unclosed #ifdef in enum should error");
}

#[test]
fn test_ifdef_inline_union_unclosed_error() {
    let input = r#"
        union Foo switch (int v) {
            #ifdef FEATURE_X
            case 0: void;
        };
    "#;
    let result = parse(input);
    assert!(result.is_err(), "unclosed #ifdef in union should error");
}

#[test]
fn test_cfg_expr_negate() {
    // negate(Feature) => Not(Feature)
    let expr = CfgExpr::Feature("X".to_string());
    assert_eq!(
        expr.negate(),
        CfgExpr::Not(Box::new(CfgExpr::Feature("X".to_string())))
    );

    // negate(Not(Feature)) => Feature (double-negation elimination)
    let expr = CfgExpr::Not(Box::new(CfgExpr::Feature("X".to_string())));
    assert_eq!(expr.negate(), CfgExpr::Feature("X".to_string()));
}

#[test]
fn test_cfg_expr_and() {
    // Feature AND Feature => All
    let a = CfgExpr::Feature("A".to_string());
    let b = CfgExpr::Feature("B".to_string());
    assert_eq!(
        a.and(b),
        CfgExpr::All(vec![
            CfgExpr::Feature("A".to_string()),
            CfgExpr::Feature("B".to_string()),
        ])
    );

    // All AND Feature => flattened All
    let a = CfgExpr::All(vec![
        CfgExpr::Feature("A".to_string()),
        CfgExpr::Feature("B".to_string()),
    ]);
    let c = CfgExpr::Feature("C".to_string());
    assert_eq!(
        a.and(c),
        CfgExpr::All(vec![
            CfgExpr::Feature("A".to_string()),
            CfgExpr::Feature("B".to_string()),
            CfgExpr::Feature("C".to_string()),
        ])
    );
}

#[test]
fn test_cfg_expr_render_feature() {
    let expr = CfgExpr::Feature("FEATURE_X".to_string());
    assert_eq!(expr.render(), r#"feature = "feature_x""#);
}

#[test]
fn test_cfg_expr_render_not() {
    let expr = CfgExpr::Not(Box::new(CfgExpr::Feature("FEATURE_X".to_string())));
    assert_eq!(expr.render(), r#"not(feature = "feature_x")"#);
}

#[test]
fn test_cfg_expr_render_all() {
    let expr = CfgExpr::All(vec![
        CfgExpr::Feature("A".to_string()),
        CfgExpr::Not(Box::new(CfgExpr::Feature("B".to_string()))),
    ]);
    assert_eq!(expr.render(), r#"all(feature = "a", not(feature = "b"))"#);
}
