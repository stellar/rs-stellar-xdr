use askama::Template;
use std::collections::HashSet;

use crate::generator::RustGenerator;
use crate::options::RustOptions;

fn generate_from_xdr(xdr: &str) -> String {
    let spec = xdr_parser::parser::parse(xdr).unwrap();
    let options = RustOptions {
        custom_default_impl: HashSet::new(),
        custom_str_impl: HashSet::new(),
        no_display_fromstr: HashSet::new(),
    };
    let generator = RustGenerator::new(&spec, options);
    let template = generator.generate(&spec, "// header\n");
    template.render().unwrap()
}

#[test]
fn test_ifdef_generates_cfg_on_struct() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        struct Foo { int x; };
        #endif
    "#,
    );
    assert!(
        output.contains(r#"#[cfg(feature = "feature_x")]"#),
        "output should contain #[cfg(feature = \"feature_x\")]\n{output}"
    );
    assert!(
        output.contains("pub struct Foo"),
        "output should contain struct Foo\n{output}"
    );
}

#[test]
fn test_ifdef_else_generates_both_cfgs() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        struct Foo { int x; };
        #else
        struct Bar { int y; };
        #endif
    "#,
    );
    assert!(
        output.contains(r#"#[cfg(feature = "feature_x")]"#),
        "output should have feature_x cfg"
    );
    assert!(
        output.contains(r#"#[cfg(not(feature = "feature_x"))]"#),
        "output should have not(feature_x) cfg"
    );
}

#[test]
fn test_ifdef_same_name_both_branches_no_cfg() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        struct Foo { int x; };
        #else
        struct Foo { int y; };
        #endif
    "#,
    );
    // When same name appears in both branches, cfg is cleared for the type enum
    // entry since the type is always present.
    assert!(
        output.contains("TypeVariant::Foo"),
        "type enum should include Foo"
    );
}

#[test]
fn test_ifdef_inline_enum_member_cfg() {
    let output = generate_from_xdr(
        r#"
        enum Color {
            RED = 0,
            #ifdef FEATURE_X
            GREEN = 1
            #endif
        };
    "#,
    );
    // The enum itself should not have a top-level cfg
    assert!(
        output.contains("pub enum Color"),
        "output should contain enum Color"
    );
    // GREEN variant should be cfg-gated
    assert!(
        output.contains(r#"#[cfg(feature = "feature_x")]"#),
        "output should cfg-gate GREEN"
    );
}

#[test]
fn test_ifdef_generates_cfg_on_const() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        const MAX_SIZE = 100;
        #endif
    "#,
    );
    assert!(
        output.contains(r#"#[cfg(feature = "feature_x")]"#),
        "const should be cfg-gated"
    );
    assert!(
        output.contains("pub const MAX_SIZE: u64 = 100"),
        "const should be present"
    );
}
