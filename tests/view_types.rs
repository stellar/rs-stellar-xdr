//! Tests for the borrowing `View` types.
//!
//! A `View` is a borrowed mirror of an owned generated type: where the owned
//! type holds a `VecM`, `BytesM`, `StringM`, or a `Box` for a cyclic reference,
//! the `View` holds a `VecMView`, `BytesMView`, `StringMView`, or a plain
//! reference. Types that own no heap data get no `View` and appear directly
//! inside the `View`s that contain them.
//!
//! Rather than assert the contents of hand-built values, these tests check the
//! properties that make a `View` a faithful mirror:
//!
//! 1. The runtime `View` types borrow their input (no copy) and enforce `MAX`
//!    exactly as the owned types do, with an error that is usable in const.
//! 2. `View`s are constructible in const contexts, including nested and cyclic
//!    ones; the fixtures below are `const` items, evaluated under every feature
//!    set, and are that proof.
//! 3. A `View` converts to its owned type by value and by reference alike, and
//!    the runtime types round-trip through their owned counterparts.
//! 4. A `View` encodes to exactly the bytes its owned counterpart does, and
//!    those bytes decode back to that owned value. The owned `WriteXdr` and
//!    `ReadXdr` (untouched by the `View` work) are the oracle.
//!
//! `View`s cannot implement `ReadXdr`: decoding must produce owned data, so it
//! always targets the owned type. That is the one asymmetry, and 4 relies on
//! it.
//!
//! The fixtures cover one example of each shape the generator emits for
//! `View`s: a struct mixing heap-free fields with nested and optional `View`s;
//! a union with void, scalar, fixed-opaque and heap arms; an `int`-switched
//! union with a heap arm; a newtype over `VecM`; a type that is cyclic through
//! an option (borrowed instead of boxed); a type that is cyclic through a
//! direct reference; and a type that recurses through `VecM` with an optional
//! `View` in a union arm. No generated `View` holds a fixed array of `View`s,
//! so that shape has no fixture.

use stellar_xdr::{
    BytesMView, ClaimPredicateView, Error, ErrorLengthExceedsMax, Hash, LedgerFootprintView,
    MemoType, MemoView, ScSpecTypeDefView, ScSpecTypeOptionView, ScValView, ScVecView,
    ScpBallotView, ScpStatementPrepareView, SorobanResourcesView, SorobanTransactionDataExtView,
    SorobanTransactionDataView, StringMView, TransactionExtView, TxDemandVectorView, ValueView,
    VecMView,
};

// ---------------------------------------------------------------------------
// Fixtures. All `const`: building them is the const-constructibility test.
// ---------------------------------------------------------------------------

/// A struct mixing heap-free fields (`Hash`, `u32`, used directly since they
/// have no `View`) with a nested `View` and both the `Some` and `None` cases
/// of an optional `View`.
const PREPARE: ScpStatementPrepareView = ScpStatementPrepareView {
    quorum_set_hash: Hash([0xAB; 32]),
    ballot: ScpBallotView {
        counter: 1,
        value: ValueView(BytesMView::try_from_slice_or_panic(b"ballot")),
    },
    prepared: Some(ScpBallotView {
        counter: 2,
        value: ValueView(BytesMView::try_from_slice_or_panic(b"prepared")),
    }),
    prepared_prime: None,
    n_c: 3,
    n_h: 4,
};

/// Every arm of a union with an enum discriminant: void, a heap newtype over
/// `StringM`, a scalar, and two fixed-opaque newtypes used directly.
const MEMOS: [MemoView; 5] = [
    MemoView::None,
    MemoView::Text(StringMView::try_from_str_or_panic("hello")),
    MemoView::Id(7),
    MemoView::Hash(Hash([1; 32])),
    MemoView::Return(Hash([2; 32])),
];

/// An `int`-switched union: the void arm, and the heap arm holding a nested
/// struct whose `VecMView`s are empty.
const TX_EXT_V0: TransactionExtView = TransactionExtView::V0;
const TX_EXT_V1: TransactionExtView = TransactionExtView::V1(SorobanTransactionDataView {
    ext: SorobanTransactionDataExtView::V0,
    resources: SorobanResourcesView {
        footprint: LedgerFootprintView {
            read_only: VecMView::try_from_slice_or_panic(&[]),
            read_write: VecMView::try_from_slice_or_panic(&[]),
        },
        instructions: 100,
        disk_read_bytes: 200,
        write_bytes: 300,
    },
    resource_fee: 999,
});

/// A newtype over a `VecM` of heap-free elements.
const DEMAND: TxDemandVectorView = TxDemandVectorView(VecMView::try_from_slice_or_panic(&[
    Hash([1; 32]),
    Hash([2; 32]),
]));

/// Cyclic through an option: the owned type boxes (`Option<Box<Self>>`), the
/// `View` borrows (`Option<&Self>`). Also a `VecMView` of `View`s.
const PREDICATE: ClaimPredicateView = ClaimPredicateView::Not(Some(&ClaimPredicateView::And(
    VecMView::try_from_slice_or_panic(&[
        ClaimPredicateView::Unconditional,
        ClaimPredicateView::BeforeAbsoluteTime(123),
    ]),
)));

/// Cyclic through a direct reference: the owned type boxes (`Box<Self>`), the
/// `View` borrows (`&Self`), at two levels.
const SPEC_OPTION: ScSpecTypeDefView = ScSpecTypeDefView::Option(&ScSpecTypeOptionView {
    value_type: &ScSpecTypeDefView::U32,
});

/// Recursive through `VecM` rather than `Box`, with an optional `View` in a
/// union arm in both its `Some` and `None` forms.
const SCVAL: ScValView = ScValView::Vec(Some(ScVecView(VecMView::try_from_slice_or_panic(&[
    ScValView::I32(1),
    ScValView::Vec(Some(ScVecView(VecMView::try_from_slice_or_panic(&[
        ScValView::Bool(true),
    ])))),
    ScValView::Vec(None),
]))));

// ---------------------------------------------------------------------------
// 1. The runtime View types borrow, and enforce MAX with a const-usable error.
// ---------------------------------------------------------------------------

/// A `View` is a borrow, not a copy: its slice is the caller's memory.
#[test]
fn runtime_views_borrow_their_input() {
    let words = [1u32, 2, 3];
    let bytes = *b"bytes";
    let text = "text";

    let v = VecMView::<u32>::try_from_slice(&words).unwrap();
    assert_eq!(v.as_slice().as_ptr(), words.as_ptr());
    assert_eq!(v.as_slice(), &words);

    let b = BytesMView::<8>::try_from_slice(&bytes).unwrap();
    assert_eq!(b.as_slice().as_ptr(), bytes.as_ptr());

    let s = StringMView::<8>::try_from_str(text).unwrap();
    assert_eq!(s.as_slice().as_ptr(), text.as_ptr());

    // Iteration likewise walks the borrowed slice.
    assert!(v.iter().eq(words.iter()));
    assert!((&v).into_iter().eq(words.iter()));
}

/// `MAX` is inclusive: exactly `MAX` elements are accepted, one more is not.
#[test]
fn runtime_views_enforce_max_len() {
    let three = [1u32, 2, 3];
    let exactly_max = VecMView::<u32, 3>::try_from_slice(&three).unwrap();
    assert_eq!(exactly_max.max_len(), 3);
    assert_eq!(
        VecMView::<u32, 2>::try_from_slice(&three),
        Err(ErrorLengthExceedsMax)
    );

    assert!(BytesMView::<3>::try_from_slice(b"abc").is_ok());
    assert_eq!(
        BytesMView::<2>::try_from_slice(b"abc"),
        Err(ErrorLengthExceedsMax)
    );

    assert!(StringMView::<3>::try_from_str("abc").is_ok());
    assert_eq!(
        StringMView::<2>::try_from_str("abc"),
        Err(ErrorLengthExceedsMax)
    );
    // The str and slice constructors are the same check on the same bytes.
    assert_eq!(
        StringMView::<3>::try_from_str("abc"),
        StringMView::<3>::try_from_slice(b"abc")
    );

    // `MAX` defaults to `u32::MAX`, matching the owned types.
    assert_eq!(
        VecMView::<u32>::try_from_slice(&three).unwrap().max_len(),
        u32::MAX as usize
    );

    // `TryFrom` is the same check surfaced through the crate's `Error`.
    assert!(matches!(
        VecMView::<u32, 2>::try_from(&three[..]),
        Err(Error::LengthExceedsMax)
    ));
}

/// The const-friendly error converts into the crate's `Error`, so runtime
/// callers propagate it with `?` like any other.
#[test]
fn error_length_exceeds_max_propagates_as_error() {
    fn build(v: &[u32]) -> Result<VecMView<'_, u32, 2>, Error> {
        Ok(VecMView::try_from_slice(v)?)
    }
    assert!(build(&[1, 2]).is_ok());
    assert!(matches!(build(&[1, 2, 3]), Err(Error::LengthExceedsMax)));
}

/// The `_or_panic` constructors are the fallible ones with the failure turned
/// into a panic, for use where a `Result` cannot be handled: their success
/// path is identical.
#[test]
fn or_panic_constructors_match_fallible_ones_on_success() {
    let words = [1u32, 2, 3];
    assert_eq!(
        VecMView::<u32, 3>::try_from_slice_or_panic(&words),
        VecMView::<u32, 3>::try_from_slice(&words).unwrap()
    );
    assert_eq!(
        BytesMView::<3>::try_from_slice_or_panic(b"abc"),
        BytesMView::<3>::try_from_slice(b"abc").unwrap()
    );
    assert_eq!(
        StringMView::<3>::try_from_str_or_panic("abc"),
        StringMView::<3>::try_from_str("abc").unwrap()
    );
}

// The panic message is the `Display` of `ErrorLengthExceedsMax`, checked
// below, so the two ways of reporting the failure read the same.
#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn vecm_view_or_panic_panics_over_max() {
    let _ = VecMView::<u32, 2>::try_from_slice_or_panic(&[1u32, 2, 3]);
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn bytesm_view_or_panic_panics_over_max() {
    let _ = BytesMView::<2>::try_from_slice_or_panic(b"abc");
}

#[test]
#[should_panic(expected = "xdr value max length exceeded")]
fn stringm_view_or_panic_panics_over_max() {
    let _ = StringMView::<2>::try_from_str_or_panic("abc");
}

/// `ErrorLengthExceedsMax` prints the same message the `_or_panic`
/// constructors panic with.
#[test]
fn error_length_exceeds_max_display() {
    assert_eq!(
        ErrorLengthExceedsMax.to_string(),
        "xdr value max length exceeded"
    );
}

/// An empty `View` is `Default`, and is what an empty slice constructs.
#[test]
fn empty_runtime_views() {
    assert_eq!(
        VecMView::<u32, 4>::default(),
        VecMView::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        BytesMView::<4>::default(),
        BytesMView::try_from_slice(&[]).unwrap()
    );
    assert_eq!(
        StringMView::<4>::default(),
        StringMView::try_from_str("").unwrap()
    );
    assert!(VecMView::<u32, 4>::default().is_empty());
    assert_eq!(VecMView::<u32, 4>::default().len(), 0);
}

/// Equality is by contents, not by which memory is borrowed.
#[test]
fn runtime_view_equality_is_by_contents() {
    let a = [1u32, 2];
    let b = [1u32, 2];
    let c = [2u32, 1];
    assert_ne!(a.as_ptr(), b.as_ptr());
    assert_eq!(
        VecMView::<u32>::try_from_slice(&a).unwrap(),
        VecMView::<u32>::try_from_slice(&b).unwrap()
    );
    assert_ne!(
        VecMView::<u32>::try_from_slice(&a).unwrap(),
        VecMView::<u32>::try_from_slice(&c).unwrap()
    );
}

// ---------------------------------------------------------------------------
// 2. Const evaluation.
// ---------------------------------------------------------------------------

/// The fallible constructors are usable in const because their error,
/// `ErrorLengthExceedsMax`, has no destructor: a `Result` holding it can be
/// matched and discarded during const evaluation, where a `Result<_, Error>`
/// could not be. This is the reason that error type exists.
///
/// The assertions are `const` blocks, so a regression here is a compile error
/// rather than a test failure: the test is that this compiles.
#[test]
fn fallible_construction_is_const_evaluable() {
    const {
        assert!(VecMView::<u32, 3>::try_from_slice(&[1, 2, 3]).is_ok());
        assert!(matches!(
            VecMView::<u32, 2>::try_from_slice(&[1, 2, 3]),
            Err(ErrorLengthExceedsMax)
        ));
        assert!(matches!(
            StringMView::<2>::try_from_str("abc"),
            Err(ErrorLengthExceedsMax)
        ));
    }
}

/// Every fixture is evaluated at compile time here, under every feature set.
///
/// An unused `const` is never evaluated, and most fixtures are otherwise only
/// used by the feature-gated modules below. Without this, an over-length slice
/// passed to a `_or_panic` constructor, or a borrow that fails to promote,
/// would go unnoticed whenever those features are off. The test is that this
/// compiles.
#[test]
fn fixtures_evaluate_at_compile_time() {
    const {
        let _ = PREPARE;
        let _ = MEMOS;
        let _ = TX_EXT_V0;
        let _ = TX_EXT_V1;
        let _ = DEMAND;
        let _ = PREDICATE;
        let _ = SPEC_OPTION;
        let _ = SCVAL;
    }
}

/// A union `View` reports its discriminant through a const method, so the
/// generated discriminant mapping works on the borrowed form too.
#[test]
fn const_union_views_report_their_discriminant() {
    const DISCRIMINANTS: [MemoType; 5] = [
        MEMOS[0].discriminant(),
        MEMOS[1].discriminant(),
        MEMOS[2].discriminant(),
        MEMOS[3].discriminant(),
        MEMOS[4].discriminant(),
    ];
    assert_eq!(
        DISCRIMINANTS,
        [
            MemoType::None,
            MemoType::Text,
            MemoType::Id,
            MemoType::Hash,
            MemoType::Return
        ]
    );
}

// ---------------------------------------------------------------------------
// 3. Conversion to owned types.
// ---------------------------------------------------------------------------

#[cfg(feature = "alloc")]
mod owned {
    use super::*;
    use stellar_xdr::{
        BytesM, ClaimPredicate, ScSpecTypeDef, ScSpecTypeOption, ScVal, ScpStatementPrepare,
        StringM, TransactionExt, TxDemandVector, VecM,
    };

    /// Converting by reference and by value are the same conversion.
    #[test]
    fn by_ref_and_by_value_conversions_agree() {
        assert_eq!(
            ScpStatementPrepare::from(&PREPARE),
            ScpStatementPrepare::from(PREPARE)
        );
        assert_eq!(ScVal::from(&SCVAL), ScVal::from(SCVAL));
        assert_eq!(
            TransactionExt::from(&TX_EXT_V1),
            TransactionExt::from(TX_EXT_V1)
        );
    }

    /// The runtime `View` types round-trip through their owned counterparts
    /// in both directions, and borrowing an owned value copies nothing.
    #[test]
    fn runtime_views_round_trip_through_owned() {
        let vecm: VecM<u32, 5> = vec![1, 2, 3].try_into().unwrap();
        let view = VecMView::from(&vecm);
        assert_eq!(view.as_slice().as_ptr(), vecm.as_slice().as_ptr());
        assert_eq!(view.to_vecm::<u32>(), vecm);

        let bytesm: BytesM<5> = vec![1u8, 2, 3].try_into().unwrap();
        let view = BytesMView::from(&bytesm);
        assert_eq!(view.as_slice().as_ptr(), bytesm.as_slice().as_ptr());
        assert_eq!(view.to_bytesm(), bytesm);

        let stringm: StringM<5> = "abc".try_into().unwrap();
        let view = StringMView::from(&stringm);
        assert_eq!(view.to_stringm(), stringm);
    }

    /// `to_vecm` converts elements as it goes, so a `VecMView` of `View`s
    /// becomes a `VecM` of owned values in one step. This is what the generated
    /// `From` impls use for every `VecM` field.
    #[test]
    fn to_vecm_converts_view_elements_to_owned() {
        let ScValView::Vec(Some(vec_view)) = SCVAL else {
            unreachable!()
        };
        let owned: VecM<ScVal> = vec_view.0.to_vecm();
        assert_eq!(owned.len(), 3);
        assert_eq!(owned.as_slice()[0], ScVal::I32(1));
    }

    /// An optional `View` maps `Some` and `None` through to the owned option.
    #[test]
    fn optional_views_convert_to_optional_owned() {
        let owned = ScpStatementPrepare::from(&PREPARE);
        assert_eq!(owned.prepared.map(|b| b.counter), Some(2));
        assert!(owned.prepared_prime.is_none());
    }

    /// Where a `View` borrows to break a cycle, the owned type boxes. This is
    /// the one place the shapes differ, so the mapping is pinned exactly.
    #[test]
    fn cyclic_borrows_convert_to_boxes() {
        let expected = ClaimPredicate::Not(Some(Box::new(ClaimPredicate::And(
            vec![
                ClaimPredicate::Unconditional,
                ClaimPredicate::BeforeAbsoluteTime(123),
            ]
            .try_into()
            .unwrap(),
        ))));
        assert_eq!(ClaimPredicate::from(&PREDICATE), expected);

        let expected = ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
            value_type: Box::new(ScSpecTypeDef::U32),
        }));
        assert_eq!(ScSpecTypeDef::from(&SPEC_OPTION), expected);
    }

    /// A newtype over `VecM` converts element-wise.
    #[test]
    fn newtype_over_vecm_converts() {
        let owned = TxDemandVector::from(&DEMAND);
        assert_eq!(owned.0.as_slice(), DEMAND.0.as_slice());
    }
}

// ---------------------------------------------------------------------------
// 4. Encoding: a View produces exactly the bytes its owned counterpart does.
// ---------------------------------------------------------------------------

#[cfg(feature = "std")]
mod xdr {
    use super::*;
    use core::fmt::Debug;
    use stellar_xdr::{
        BytesM, ClaimPredicate, Limits, Memo, ReadXdr, ScSpecTypeDef, ScVal, ScpStatementPrepare,
        StringM, TransactionExt, TxDemandVector, VecM, WriteXdr,
    };

    /// The oracle: the `View`'s bytes must equal the owned value's bytes, and
    /// must decode (which only the owned type can do) back to that value.
    fn assert_encodes_like_owned<V, O>(view: &V, owned: &O)
    where
        V: WriteXdr,
        O: WriteXdr + ReadXdr + PartialEq + Debug,
    {
        let from_view = view.to_xdr(Limits::none()).unwrap();
        let from_owned = owned.to_xdr(Limits::none()).unwrap();
        assert_eq!(from_view, from_owned, "View and owned encodings differ");
        let decoded = O::from_xdr(&from_view, Limits::none()).unwrap();
        assert_eq!(
            &decoded, owned,
            "View encoding did not decode back to the owned value"
        );
    }

    /// Check a `View` against the owned value it converts to.
    fn assert_view_encodes<V, O>(view: &V)
    where
        V: WriteXdr,
        O: WriteXdr + ReadXdr + PartialEq + Debug + for<'a> From<&'a V>,
    {
        assert_encodes_like_owned(view, &O::from(view));
    }

    #[test]
    fn struct_with_nested_and_optional_views() {
        assert_view_encodes::<_, ScpStatementPrepare>(&PREPARE);
    }

    #[test]
    fn union_with_void_scalar_opaque_and_heap_arms() {
        for memo in &MEMOS {
            assert_view_encodes::<_, Memo>(memo);
        }
    }

    #[test]
    fn int_switched_union() {
        assert_view_encodes::<_, TransactionExt>(&TX_EXT_V0);
        assert_view_encodes::<_, TransactionExt>(&TX_EXT_V1);
    }

    #[test]
    fn newtype_over_vecm() {
        assert_view_encodes::<_, TxDemandVector>(&DEMAND);
    }

    #[test]
    fn cyclic_through_option() {
        assert_view_encodes::<_, ClaimPredicate>(&PREDICATE);
        assert_view_encodes::<_, ClaimPredicate>(&ClaimPredicateView::Not(None));
    }

    #[test]
    fn cyclic_through_reference() {
        assert_view_encodes::<_, ScSpecTypeDef>(&SPEC_OPTION);
    }

    #[test]
    fn recursive_through_vecm_with_optional_arm() {
        assert_view_encodes::<_, ScVal>(&SCVAL);
    }

    /// The runtime `View` types encode like the owned types they mirror,
    /// including the byte form `VecM<u8>` takes instead of a sequence of
    /// encoded integers, and including empty values.
    #[test]
    fn runtime_views_encode_like_owned() {
        let words = [1u32, 2, 3];
        let vecm: VecM<u32, 3> = words.to_vec().try_into().unwrap();
        assert_encodes_like_owned(&VecMView::<u32, 3>::try_from_slice(&words).unwrap(), &vecm);

        let bytes = *b"bytes";
        let vecm_u8: VecM<u8, 8> = bytes.to_vec().try_into().unwrap();
        assert_encodes_like_owned(
            &VecMView::<u8, 8>::try_from_slice(&bytes).unwrap(),
            &vecm_u8,
        );

        let bytesm: BytesM<8> = bytes.to_vec().try_into().unwrap();
        assert_encodes_like_owned(&BytesMView::<8>::try_from_slice(&bytes).unwrap(), &bytesm);

        let stringm: StringM<8> = "text".try_into().unwrap();
        assert_encodes_like_owned(&StringMView::<8>::try_from_str("text").unwrap(), &stringm);

        let empty: VecM<u32, 3> = VecM::default();
        assert_encodes_like_owned(&VecMView::<u32, 3>::default(), &empty);
    }
}
