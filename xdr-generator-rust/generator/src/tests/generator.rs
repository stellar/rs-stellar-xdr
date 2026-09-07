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

fn assert_not_contains(output: &str, unexpected: &str) {
    assert!(
        !output.contains(unexpected),
        "expected output not to contain:\n{unexpected}\n\nfull output:\n{output}"
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
    // but the whole type enum API is behind the `type_enum` feature.
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
pub const MAX_SIZE: u32 = 100;"#,
    );
}

#[test]
fn test_no_ref_form_when_only_one_ifdef_branch_borrows() {
    // Foo holds heap data only in the FEATURE_X branch, so it borrows under
    // some cfgs but not all. A cfg-gated FooRef would be named unconditionally
    // by any always-borrowing type holding a Foo, so no Ref form is emitted.
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        struct Foo { string s<10>; };
        #else
        struct Foo { int y; };
        #endif
    "#,
    );
    assert_not_contains(&output, "FooRef");
}

#[test]
fn test_no_ref_form_when_only_one_ifdef_branch_borrows_typedef() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        typedef string Foo<10>;
        #else
        typedef opaque Foo[4];
        #endif
    "#,
    );
    assert_not_contains(&output, "FooRef");
}

#[test]
fn test_no_ref_form_when_only_one_ifdef_branch_borrows_union() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        union Foo switch (int v) { case 0: string s<10>; };
        #else
        union Foo switch (int v) { case 0: int y; };
        #endif
    "#,
    );
    assert_not_contains(&output, "FooRef");
}

#[test]
fn test_no_ref_form_when_only_a_cfg_gated_arm_borrows() {
    // The union's only heap sits behind a cfg-gated arm, so it borrows under
    // some cfgs but not all and gets no Ref form.
    let output = generate_from_xdr(
        r#"
        union Foo switch (int v) {
            case 0: int y;
            #ifdef FEATURE_X
            case 1: string s<10>;
            #endif
        };
    "#,
    );
    assert_not_contains(&output, "FooRef");
}

#[test]
fn test_cfg_conditional_heap_leaves_containing_types_owned() {
    // Exec borrows only via its cfg-gated arm, so neither it nor OnlyExec —
    // whose sole heap comes from Exec — gets a Ref form. Parent has heap of
    // its own, so it does, and holds the owned Exec in that position. That
    // compiles whether or not the feature is on, which a cfg-gated ExecRef
    // named by the unconditional ParentRef would not.
    let output = generate_from_xdr(
        r#"
        union Exec switch (int type)
        {
        case 0:
            void;
        #ifdef FEATURE_X
        case 1:
            string tag<64>;
        #endif
        };
        struct OnlyExec { Exec exec; int n; };
        struct Parent { Exec exec; string label<32>; };
    "#,
    );
    assert_not_contains(&output, "ExecRef");
    assert_not_contains(&output, "OnlyExecRef");
    assert_contains(
        &output,
        r#"#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParentRef<'a> {
    pub exec: Exec,
    pub label: StringMRef<'a, 32>,
}"#,
    );
}

#[test]
fn test_no_ref_form_for_heap_free_types() {
    // A type with no heap under any cfg gets no Ref form at all, since there
    // is nothing for it to borrow.
    let output = generate_from_xdr(
        r#"
        struct Flat { int a; opaque b[4]; };
    "#,
    );
    assert_not_contains(&output, "FlatRef");
}
