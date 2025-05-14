#![cfg(all(feature = "next", feature = "arbitrary"))]

use arbitrary::{Arbitrary, Unstructured};
use stellar_xdr::next::ScMap;

#[test]
fn arb() {
    let bytes: Vec<u8> = (1u8..255).collect();
    let mut unstructured = Unstructured::new(&bytes);
    for _ in 1..10 {
        let x: ScMap = ScMap::arbitrary(&mut unstructured).unwrap();
        eprintln!("{x:?}");
    }
}
