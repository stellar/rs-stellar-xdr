use crate::parser::parse;

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
        assert_eq!(union_def.arms[0].name.as_deref(), Some("myField"));
        assert_eq!(union_def.arms[1].name, None); // void arm
    } else {
        panic!("expected Union");
    }
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
        assert_eq!(union_def.arms[0].name.as_deref(), Some("myInlineField"));
    } else {
        panic!("expected Union");
    }
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
        assert_eq!(union_def.arms[0].cases.len(), 2);
        assert_eq!(union_def.arms[0].name.as_deref(), Some("sharedField"));
        assert_eq!(union_def.arms[1].cases.len(), 1);
        assert_eq!(union_def.arms[1].name, None);
    } else {
        panic!("expected Union");
    }
}
