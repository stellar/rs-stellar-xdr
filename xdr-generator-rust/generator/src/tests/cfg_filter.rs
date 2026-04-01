use std::collections::HashSet;
use xdr_parser::ast::{CfgExpr, Definition, XdrSpec};

use crate::{filter_spec, validate_union_coverage};

fn features(names: &[&str]) -> HashSet<String> {
    names.iter().map(|s| s.to_lowercase()).collect()
}

fn parse(input: &str) -> XdrSpec {
    xdr_parser::parser::parse(input).unwrap()
}

// =============================================================================
// CfgExpr::evaluate
// =============================================================================

#[test]
fn test_evaluate_cfg_feature_present() {
    let expr = CfgExpr::Feature("NEXT".to_string());
    assert!(expr.evaluate(&features(&["next"])));
}

#[test]
fn test_evaluate_cfg_feature_absent() {
    let expr = CfgExpr::Feature("NEXT".to_string());
    assert!(!expr.evaluate(&features(&["other"])));
}

#[test]
fn test_evaluate_cfg_feature_case_insensitive() {
    let expr = CfgExpr::Feature("Next".to_string());
    assert!(expr.evaluate(&features(&["NEXT"])));
}

#[test]
fn test_evaluate_cfg_not() {
    let expr = CfgExpr::Not(Box::new(CfgExpr::Feature("NEXT".to_string())));
    assert!(expr.evaluate(&features(&["other"])));
    assert!(!expr.evaluate(&features(&["next"])));
}

#[test]
fn test_evaluate_cfg_all() {
    let expr = CfgExpr::All(vec![
        CfgExpr::Feature("A".to_string()),
        CfgExpr::Feature("B".to_string()),
    ]);
    assert!(expr.evaluate(&features(&["a", "b"])));
    assert!(!expr.evaluate(&features(&["a"])));
    assert!(!expr.evaluate(&features(&["b"])));
}

#[test]
fn test_evaluate_cfg_empty_features() {
    let expr = CfgExpr::Feature("X".to_string());
    assert!(!expr.evaluate(&features(&[])));
}

// =============================================================================
// definition cfg integration
// =============================================================================

#[test]
fn test_def_without_cfg_has_no_cfg() {
    let spec = parse("struct Foo { int x; };");
    assert!(spec.definitions[0].cfg().is_none());
}

#[test]
fn test_def_cfg_evaluates_correctly() {
    let spec = parse(
        r#"
        #ifdef NEXT
        struct Foo { int x; };
        #endif
    "#,
    );
    let cfg = spec.definitions[0].cfg().unwrap();
    assert!(cfg.evaluate(&features(&["next"])));
    assert!(!cfg.evaluate(&features(&["other"])));
}

// =============================================================================
// filter_spec — definitions
// =============================================================================

#[test]
fn test_filter_spec_removes_gated_definitions() {
    let mut spec = parse(
        r#"
        struct Always { int x; };
        #ifdef NEXT
        struct OnlyNext { int y; };
        #endif
    "#,
    );
    filter_spec(&mut spec, &features(&["other"])).unwrap();
    let names: Vec<&str> = spec.definitions.iter().map(|d| d.name()).collect();
    assert_eq!(names, vec!["Always"]);
}

#[test]
fn test_filter_spec_keeps_matching_definitions() {
    let mut spec = parse(
        r#"
        struct Always { int x; };
        #ifdef NEXT
        struct OnlyNext { int y; };
        #endif
    "#,
    );
    filter_spec(&mut spec, &features(&["next"])).unwrap();
    let names: Vec<&str> = spec.definitions.iter().map(|d| d.name()).collect();
    assert_eq!(names, vec!["Always", "OnlyNext"]);
}

#[test]
fn test_filter_spec_clears_cfg_on_surviving_definitions() {
    let mut spec = parse(
        r#"
        #ifdef NEXT
        struct Foo { int x; };
        #endif
    "#,
    );
    filter_spec(&mut spec, &features(&["next"])).unwrap();
    assert!(spec.definitions[0].cfg().is_none());
}

// =============================================================================
// filter_spec — union arms
// =============================================================================

#[test]
fn test_filter_spec_removes_gated_union_arms() {
    // Both the enum member and union arm are cfg-gated, so filtering
    // removes both and the union remains complete.
    let mut spec = parse(
        r#"
        enum E {
            A = 0,
            #ifdef NEXT
            B = 1,
            #endif
        };
        union U switch (E v) {
            case A:
                int a;
            #ifdef NEXT
            case B:
                int b;
            #endif
        };
    "#,
    );
    filter_spec(&mut spec, &features(&["other"])).unwrap();
    let u = match &spec.definitions[1] {
        Definition::Union(u) => u,
        _ => panic!("expected Union"),
    };
    assert_eq!(u.arms.len(), 1);
    assert!(u.arms[0].cfg.is_none());
}

#[test]
fn test_filter_spec_keeps_matching_union_arms() {
    let mut spec = parse(
        r#"
        enum E {
            A = 0,
            #ifdef NEXT
            B = 1,
            #endif
        };
        union U switch (E v) {
            case A:
                int a;
            #ifdef NEXT
            case B:
                int b;
            #endif
        };
    "#,
    );
    filter_spec(&mut spec, &features(&["next"])).unwrap();
    let u = match &spec.definitions[1] {
        Definition::Union(u) => u,
        _ => panic!("expected Union"),
    };
    assert_eq!(u.arms.len(), 2);
    // All cfg annotations cleared.
    assert!(u.arms.iter().all(|a| a.cfg.is_none()));
}

// =============================================================================
// filter_spec — enum members
// =============================================================================

#[test]
fn test_filter_spec_removes_gated_enum_members() {
    let mut spec = parse(
        r#"
        enum Color {
            RED = 0,
            #ifdef NEXT
            GREEN = 1,
            #endif
            BLUE = 2
        };
    "#,
    );
    filter_spec(&mut spec, &features(&["other"])).unwrap();
    let e = match &spec.definitions[0] {
        Definition::Enum(e) => e,
        _ => panic!("expected Enum"),
    };
    let values: Vec<i32> = e.members.iter().map(|m| m.value).collect();
    assert_eq!(values, vec![0, 2]);
    assert!(e.members.iter().all(|m| m.cfg.is_none()));
}

// =============================================================================
// validate_union_coverage
// =============================================================================

#[test]
fn test_validate_union_coverage_passes_when_complete() {
    let spec = parse(
        r#"
        enum E { A = 0, B = 1 };
        union U switch (E v) {
            case A: int a;
            case B: int b;
        };
    "#,
    );
    validate_union_coverage(&spec).unwrap();
}

#[test]
fn test_validate_union_coverage_fails_when_missing_arm() {
    // Manually construct a spec where the union is missing a case for enum value B.
    let mut spec = parse(
        r#"
        enum E { A = 0, B = 1 };
        union U switch (E v) {
            case A: int a;
            case B: int b;
        };
    "#,
    );
    // Remove the B arm to simulate cfg filtering.
    if let Definition::Union(ref mut u) = spec.definitions[1] {
        u.arms.retain(|arm| {
            arm.cases.iter().any(|c| match &c.value {
                xdr_parser::ast::UnionCaseValue::Ident { ident } => ident != "B",
                _ => true,
            })
        });
    }
    let result = validate_union_coverage(&spec);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("U"), "error should mention union name: {err}");
}

#[test]
fn test_validate_union_coverage_ok_with_int_discriminant() {
    // Unions with non-enum discriminants are not checked.
    let spec = parse(
        r#"
        union U switch (int v) {
            case 0: int a;
        };
    "#,
    );
    validate_union_coverage(&spec).unwrap();
}

// =============================================================================
// filter_spec — union coverage error via cfg filtering
// =============================================================================

#[test]
fn test_filter_spec_fails_when_cfg_creates_incomplete_union() {
    // The union arm for B is cfg-gated, but the enum member B is NOT.
    // After filtering, the enum still has B but the union doesn't cover it.
    let mut spec = parse(
        r#"
        enum E { A = 0, B = 1 };
        union U switch (E v) {
            case A:
                int a;
            #ifdef NEXT
            case B:
                int b;
            #endif
        };
    "#,
    );
    let result = filter_spec(&mut spec, &features(&["other"]));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("U"), "error should mention union name: {err}");
    assert!(err.contains("1"), "error should mention missing value 1: {err}");
}

// =============================================================================
// filter_spec — ifdef/else dual definitions
// =============================================================================

#[test]
fn test_filter_spec_ifdef_else_keeps_correct_branch() {
    let mut spec = parse(
        r#"
        #ifdef NEXT
        struct Foo { int x; };
        #else
        struct Foo { int x; int y; };
        #endif
    "#,
    );
    filter_spec(&mut spec, &features(&["next"])).unwrap();
    let names: Vec<&str> = spec.definitions.iter().map(|d| d.name()).collect();
    assert_eq!(names, vec!["Foo"]);
    if let Definition::Struct(s) = &spec.definitions[0] {
        assert_eq!(s.members.len(), 1); // The #ifdef branch has 1 member.
    } else {
        panic!("expected Struct");
    }
}

#[test]
fn test_filter_spec_ifdef_else_keeps_else_branch() {
    let mut spec = parse(
        r#"
        #ifdef NEXT
        struct Foo { int x; };
        #else
        struct Foo { int x; int y; };
        #endif
    "#,
    );
    filter_spec(&mut spec, &features(&["other"])).unwrap();
    let names: Vec<&str> = spec.definitions.iter().map(|d| d.name()).collect();
    assert_eq!(names, vec!["Foo"]);
    if let Definition::Struct(s) = &spec.definitions[0] {
        assert_eq!(s.members.len(), 2); // The #else branch has 2 members.
    } else {
        panic!("expected Struct");
    }
}
