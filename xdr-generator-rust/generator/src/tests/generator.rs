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

fn assert_contains(output: &str, expected: &str) {
    assert!(
        output.contains(expected),
        "expected output to contain:\n{expected}\n\nfull output:\n{output}"
    );
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
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Foo {"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
impl ReadXdr for Foo {"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
impl WriteXdr for Foo {"#,
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
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Foo {"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Bar {"#,
    );
}

#[test]
fn test_ifdef_same_name_both_branches() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        struct Foo { int x; };
        #else
        struct Foo { int y; };
        #endif
    "#,
    );
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Foo {
    pub x: i32,
}"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Foo {
    pub y: i32,
}"#,
    );
    // Same name in both branches: TypeVariant entry has no per-branch cfg,
    // but the whole type enum API is behind the `type` feature.
    assert_contains(
        &output,
        "#[cfg(feature = \"type_enum\")]\n#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]",
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
    assert_contains(
        &output,
        r#"pub enum Color {
    #[cfg_attr(feature = "alloc", default)]
    Red = 0,
    #[cfg(feature = "feature_x")]
    Green = 1,
}"#,
    );
    assert_contains(
        &output,
        r#"let e = match i {
            0 => Color::Red,
            #[cfg(feature = "feature_x")]
            1 => Color::Green,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };"#,
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
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
pub const MAX_SIZE: u64 = 100;"#,
    );
}
