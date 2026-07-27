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
pub const MAX_SIZE: u64 = 100;"#,
    );
}

#[test]
fn test_ifdef_same_name_ref_phantom_struct() {
    // Foo requires a Ref variant because the FEATURE_X branch contains heap
    // data; the heap-free branch must bind the Ref lifetime with a phantom
    // member.
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
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FooRef<'a> {
    pub y: i32,
    /// Binds the `'a` lifetime when no member borrows.
    #[doc(hidden)]
    pub _phantom: core::marker::PhantomData<&'a ()>,
}"#,
    );
}

#[test]
fn test_ifdef_same_name_ref_phantom_typedef() {
    let output = generate_from_xdr(
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
        r#"pub struct FooRef<'a>(pub StringMRef<'a, 10>);"#,
    );
    assert_contains(
        &output,
        r#"pub struct FooRef<'a>(pub [u8; 4], pub core::marker::PhantomData<&'a ()>);"#,
    );
}

#[test]
fn test_ifdef_same_name_ref_phantom_union() {
    let output = generate_from_xdr(
        r#"
        #ifdef FEATURE_X
        union Foo switch (int v) { case 0: string s<10>; };
        #else
        union Foo switch (int v) { case 0: int y; };
        #endif
    "#,
    );
    // The parser propagates a definition-level #ifdef cfg onto each union
    // arm, so the lifetime-using arm is cfg-gated and the phantom variant
    // appears under the negated cfg (harmlessly dead, since the whole enum is
    // gated on the positive cfg).
    assert_contains(
        &output,
        r#"pub enum FooRef<'a> {
    #[cfg(feature = "feature_x")]
    V0(StringMRef<'a, 10>),
    /// Uninhabited variant binding the `'a` lifetime when every
    /// lifetime-using variant is compiled out.
    #[cfg(not(feature = "feature_x"))]
    #[doc(hidden)]
    _Phantom(core::convert::Infallible, core::marker::PhantomData<&'a ()>),
}"#,
    );
    // The heap-free branch has no lifetime-using arm at all, so its phantom
    // variant is unconditional.
    assert_contains(
        &output,
        r#"pub enum FooRef<'a> {
    #[cfg(not(feature = "feature_x"))]
    V0(i32),
    /// Uninhabited variant binding the `'a` lifetime when every
    /// lifetime-using variant is compiled out.
    #[doc(hidden)]
    _Phantom(core::convert::Infallible, core::marker::PhantomData<&'a ()>),
}"#,
    );
}

#[test]
fn test_cfg_arm_only_heap_union_ref_phantom_cfg() {
    // The union requires a Ref variant only via the cfg-gated arm, so the
    // phantom variant appears under the negated cfg.
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
        r#"pub enum FooRef<'a> {
    V0(i32),
    #[cfg(feature = "feature_x")]
    V1(StringMRef<'a, 10>),
    /// Uninhabited variant binding the `'a` lifetime when every
    /// lifetime-using variant is compiled out.
    #[cfg(not(feature = "feature_x"))]
    #[doc(hidden)]
    _Phantom(core::convert::Infallible, core::marker::PhantomData<&'a ()>),
}"#,
    );
}
