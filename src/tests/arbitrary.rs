#![cfg(feature = "arbitrary")]

use crate::{BytesM, ScMap, ScSpecEntry, StringM, VecM, SCSYMBOL_LIMIT};
use arbitrary::{Arbitrary, Unstructured};
use rand::{rngs::StdRng, RngCore, SeedableRng};

#[test]
fn arb() {
    let bytes: Vec<u8> = (1u8..255).collect();
    let mut unstructured = Unstructured::new(&bytes);
    for _ in 1..10 {
        let x: ScMap = ScMap::arbitrary(&mut unstructured).unwrap();
        eprintln!("{x:?}");
    }
}

/// The length limit is an invariant of the length-limited types, so the
/// `Arbitrary` impls must not produce values that exceed it. A derived impl
/// would, because the limit lives in a const parameter the derive cannot see.
#[test]
fn arbitrary_stays_within_max() {
    let mut bytes = [0u8; 4096];
    StdRng::seed_from_u64(0).fill_bytes(&mut bytes);
    for start in 0..bytes.len() / 2 {
        let mut u = Unstructured::new(&bytes[start..]);
        assert!(StringM::<3>::arbitrary(&mut u).unwrap().len() <= 3);
        assert!(BytesM::<5>::arbitrary(&mut u).unwrap().len() <= 5);
        assert!(VecM::<u32, 2>::arbitrary(&mut u).unwrap().len() <= 2);
    }
}

/// The minimum length is an invariant too, so input that runs out before the
/// minimum is padded rather than producing a shorter value.
#[test]
fn arbitrary_stays_within_min() {
    let mut u = Unstructured::new(&[]);
    assert_eq!(BytesM::<5, 2>::arbitrary(&mut u).unwrap().as_vec(), &[0, 0]);
    let u = Unstructured::new(&[]);
    assert_eq!(
        BytesM::<5, 2>::arbitrary_take_rest(u).unwrap().as_vec(),
        &[0, 0]
    );
}

/// Input past the limit must be left for the fields that follow, rather than
/// read into elements that are then thrown away.
#[test]
fn arbitrary_does_not_consume_past_max() {
    // `Vec`'s impl reads a continuation byte before each element, and every
    // byte here is non-zero, so the input describes far more elements than the
    // limit allows.
    let bytes = vec![1u8; 64];
    let mut u = Unstructured::new(&bytes);
    let v = VecM::<u8, 2>::arbitrary(&mut u).unwrap();
    assert_eq!(v.len(), 2);
    // Two elements: a continuation byte and an element byte each, and nothing
    // read for a third.
    assert_eq!(bytes.len() - u.len(), 4);
}

/// The same invariant, reached through a generated type: a function returns at
/// most one output, its name is at most `SCSYMBOL_LIMIT` bytes, and an input
/// name at most 30.
#[test]
fn arbitrary_generated_type_stays_within_max() {
    let mut bytes = [0u8; 8192];
    StdRng::seed_from_u64(0).fill_bytes(&mut bytes);
    for start in 0..bytes.len() / 2 {
        let mut u = Unstructured::new(&bytes[start..]);
        let Ok(entry) = ScSpecEntry::arbitrary(&mut u) else {
            continue;
        };
        if let ScSpecEntry::FunctionV0(f) = entry {
            assert!(f.outputs.len() <= 1);
            assert!(f.name.len() <= SCSYMBOL_LIMIT as usize);
            for i in &*f.inputs {
                assert!(i.name.len() <= 30);
            }
        }
    }
}
