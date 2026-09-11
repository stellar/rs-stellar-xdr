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

/// Generates into a temporary directory and returns the `const` module — the
/// module file and every per-definition file under it, concatenated — since
/// that is where the const forms live.
fn generate_const_from_xdr(xdr: &str) -> String {
    let spec = xdr_parser::parser::parse(xdr).unwrap();
    let options = RustOptions {
        custom_default_impl: HashSet::new(),
        custom_str_impl: HashSet::new(),
        no_display_fromstr: HashSet::new(),
    };
    let generator = RustGenerator::new(&spec, options);

    let dir = std::env::temp_dir().join(format!(
        "xdr-generator-test-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let out = dir.join("generated");
    generator
        .generate_to_dir(&spec, &dir.join("generated.rs"), &out)
        .unwrap();

    let mut files = vec![out.join("const.rs")];
    let mut const_files: Vec<_> = std::fs::read_dir(out.join("const"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    const_files.sort();
    files.append(&mut const_files);

    let output = files
        .iter()
        .map(|f| std::fs::read_to_string(f).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::remove_dir_all(&dir).unwrap();
    output
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
fn test_const_form_for_both_ifdef_branches() {
    // Foo holds heap data only in the FEATURE_X branch. Both branches get a
    // const form, each gated like its definition, so the name resolves under
    // every cfg; the heap-free branch mirrors the owned fields.
    let output = generate_const_from_xdr(
        r#"
        #ifdef FEATURE_X
        struct Foo { string s<10>; };
        #else
        struct Foo { int y; };
        #endif
    "#,
    );
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Foo {
    pub s: StringM<10>,
}"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Foo {
    pub y: i32,
}"#,
    );
}

#[test]
fn test_const_form_for_both_ifdef_branches_typedef() {
    let output = generate_const_from_xdr(
        r#"
        #ifdef FEATURE_X
        typedef string Foo<10>;
        #else
        typedef opaque Foo[4];
        #endif
    "#,
    );
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Foo(pub StringM<10>);"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Foo(pub [u8; 4]);"#,
    );
}

#[test]
fn test_const_form_for_both_ifdef_branches_union() {
    let output = generate_const_from_xdr(
        r#"
        #ifdef FEATURE_X
        union Foo switch (int v) { case 0: string s<10>; };
        #else
        union Foo switch (int v) { case 0: int y; };
        #endif
    "#,
    );
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum Foo {
    #[cfg(feature = "feature_x")]
    V0(StringM<10>),
}"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum Foo {
    #[cfg(not(feature = "feature_x"))]
    V0(i32),
}"#,
    );
}

#[test]
fn test_const_form_when_only_a_cfg_gated_arm_borrows() {
    // The union's only heap sits behind a cfg-gated arm. The const form is
    // emitted unconditionally with the arm gated inside it, so the arm's
    // payload is the borrowing StringM wherever the cfg turns it on.
    let output = generate_const_from_xdr(
        r#"
        union Foo switch (int v) {
            case 0: int y;
            #ifdef FEATURE_X
            case 1: string s<10>;
            #endif
        };
    "#,
    );
    assert_contains(
        &output,
        r#"pub enum Foo {
    V0(i32),
    #[cfg(feature = "feature_x")]
    V1(StringM<10>),
}"#,
    );
}

#[test]
fn test_cfg_gated_heap_gets_a_const_form() {
    // Exec borrows only via its cfg-gated arm, so types holding an Exec name
    // the const module's Exec in that position under every cfg. Leaving the
    // owned Exec there would not compile with the feature on: serializing it
    // reaches a Vec-backed StringM from a const fn.
    let output = generate_const_from_xdr(
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
    assert_contains(
        &output,
        r#"#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Parent {
    pub exec: Exec,
    pub label: StringM<32>,
}"#,
    );
    assert_contains(
        &output,
        r#"#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct OnlyExec {
    pub exec: Exec,
    pub n: i32,
}"#,
    );
}

#[test]
fn test_no_const_form_for_heap_free_types() {
    // A type with no heap under any cfg gets no borrowing form of its own,
    // since there is nothing for it to borrow. The const module re-exports the
    // owned type under the same name instead, so the name still resolves
    // there.
    let output = generate_const_from_xdr(
        r#"
        struct Flat { int a; opaque b[4]; };
    "#,
    );
    assert_not_contains(&output, "pub struct Flat {");
    assert_contains(&output, "pub use super::Flat;");
}
