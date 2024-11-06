#![cfg(all(feature = "curr", feature = "hex"))]
use stellar_xdr::curr::Hash;

#[test]
fn hash_padding() {
    let padded = "0000000000000000000000000000000011111111111111111111111111111111";
    let non_padded = "11111111111111111111111111111111";
    assert_eq!(Hash::from_hex(padded), Hash::from_hex(non_padded));
}
