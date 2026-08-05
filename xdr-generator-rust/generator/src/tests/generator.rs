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
fn test_ifdef_same_name_ref_alias_struct() {
    // Foo has a Ref form because the FEATURE_X branch contains heap data. The
    // heap-free branch borrows nothing, so its Ref form is a transparent alias
    // of the owned type rather than a struct with an unused lifetime.
    let output = generate_from_xdr(
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
pub struct FooRef<'a> {
    pub s: StringMRef<'a, 10>,
}"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
pub type FooRef<'a> = Foo;"#,
    );
    // The alias branch emits no Ref struct of its own, and no phantom.
    assert_not_contains(&output, "_Phantom");
    assert_not_contains(&output, "PhantomData");
    assert_not_contains(
        &output,
        r#"pub struct FooRef<'a> {
    pub y: i32,"#,
    );
}

#[test]
fn test_ifdef_same_name_ref_alias_typedef() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        typedef string Foo<10>;
        #else
        typedef opaque Foo[4];
        #endif
    "#,
    );
    assert_contains(&output, r#"pub struct FooRef<'a>(pub StringMRef<'a, 10>);"#);
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
pub type FooRef<'a> = Foo;"#,
    );
    assert_not_contains(&output, "PhantomData");
}

#[test]
fn test_ifdef_same_name_ref_alias_union() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        union Foo switch (int v) { case 0: string s<10>; };
        #else
        union Foo switch (int v) { case 0: int y; };
        #endif
    "#,
    );
    // The parser propagates the definition-level cfg onto each arm. That arm
    // cfg is implied by the enum's own cfg, so the enum is gated on it once
    // rather than on `all(c, c)` with a dead `all(c, not(c))` alias beside it.
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum FooRef<'a> {
    #[cfg(feature = "feature_x")]
    V0(StringMRef<'a, 10>),
}"#,
    );
    assert_not_contains(&output, "not(feature = \"feature_x\")))");
    // The heap-free branch has no borrowing arm, so it becomes an alias.
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
pub type FooRef<'a> = Foo;"#,
    );
    assert_not_contains(&output, "_Phantom");
}

#[test]
fn test_cfg_arm_only_heap_union_ref_alias_cfg() {
    // The union borrows only via its cfg-gated arm, so the real Ref enum is
    // gated on that cfg and the alias covers its negation. Neither form
    // carries a phantom variant, so matching stays exhaustive over the real
    // XDR cases only.
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
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum FooRef<'a> {
    V0(i32),
    #[cfg(feature = "feature_x")]
    V1(StringMRef<'a, 10>),
}"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
pub type FooRef<'a> = Foo;"#,
    );
    assert_not_contains(&output, "_Phantom");
}

#[test]
fn test_cfg_conditional_heap_cascades_to_containing_types() {
    // Exec borrows only via its cfg-gated arm. OnlyExec's sole heap comes from
    // Exec, so its Ref form must degenerate to an alias under the same cfg —
    // otherwise its `'a` would be unused and fail to compile. Parent has heap
    // of its own, so its Ref form stays unconditional and names ExecRef<'a>
    // uniformly across cfgs rather than being cfg-split.
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
    // Exec: real enum under the feature, alias without it.
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ExecRef<'a> {"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
pub type ExecRef<'a> = Exec;"#,
    );
    // OnlyExec inherits the condition from Exec.
    assert_contains(
        &output,
        r#"#[cfg(feature = "feature_x")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OnlyExecRef<'a> {
    pub exec: ExecRef<'a>,
    pub n: i32,
}"#,
    );
    assert_contains(
        &output,
        r#"#[cfg(not(feature = "feature_x"))]
pub type OnlyExecRef<'a> = OnlyExec;"#,
    );
    // Parent always borrows, so it gets no alias and no cfg on its Ref struct.
    assert_contains(
        &output,
        r#"#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParentRef<'a> {
    pub exec: ExecRef<'a>,
    pub label: StringMRef<'a, 32>,
}"#,
    );
    assert_not_contains(&output, "pub type ParentRef");
    assert_not_contains(&output, "_Phantom");
    assert_not_contains(&output, "PhantomData");
}

#[test]
fn test_no_ref_form_for_heap_free_types() {
    // A type with no heap under any cfg gets no Ref form at all, not even an
    // alias, since nothing can reference a Ref form of it.
    let output = generate_from_xdr(
        r#"
        struct Flat { int a; opaque b[4]; };
    "#,
    );
    assert_not_contains(&output, "FlatRef");
}
