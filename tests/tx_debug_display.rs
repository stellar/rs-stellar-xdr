#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{BytesM, Error, Hash, StringM, VecM};

#[test]
fn test_debug() {
    // Renders as array.
    assert_eq!(
        format!(
            "{:?}",
            <_ as TryInto<VecM<u8>>>::try_into("hello world").unwrap()
        ),
        "VecM([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100])"
    );
    // Renders as hex.
    assert_eq!(
        format!(
            "{:?}",
            <_ as TryInto<BytesM>>::try_into("hello world").unwrap()
        ),
        "BytesM(68656c6c6f20776f726c64)"
    );
    // Renders as string.
    assert_eq!(
        format!(
            "{:?}",
            <_ as TryInto<StringM>>::try_into("hello world").unwrap()
        ),
        "StringM(hello world)"
    );
    // Renders as hex.
    assert_eq!(
        format!(
            "{:?}",
            <_ as Into<Hash>>::into(*b"01234567890123456789013456789012")
        ),
        "Hash(3031323334353637383930313233343536373839303133343536373839303132)"
    );
}

#[test]
fn test_debug_invalid_utf8() -> Result<(), Error> {
    // VecM is unchanged.
    assert_eq!(
        format!(
            "{:?}",
            <_ as TryInto<VecM<u8>>>::try_into(b"hello\xc3\x28world")?
        ),
        "VecM([104, 101, 108, 108, 111, 195, 40, 119, 111, 114, 108, 100])"
    );
    // BytesM is unchanged.
    assert_eq!(
        format!(
            "{:?}",
            <_ as TryInto<BytesM>>::try_into(b"hello\xc3\x28world")?
        ),
        "BytesM(68656c6c6fc328776f726c64)"
    );
    // StringM escapes strings.
    assert_eq!(
        format!(
            "{:?}",
            <_ as TryInto<StringM>>::try_into(b"hello\xc3\x28world")?
        ),
        r"StringM(hello\xc3(world)"
    );
    Ok(())
}

#[test]
fn test_display() -> Result<(), Error> {
    // Renders as hex.
    assert_eq!(
        format!("{}", <_ as TryInto<BytesM>>::try_into("hello world")?),
        "68656c6c6f20776f726c64"
    );
    // Renders as string.
    assert_eq!(
        format!("{}", <_ as TryInto<StringM>>::try_into("hello world")?),
        "hello world"
    );
    // Renders as hex.
    assert_eq!(
        format!(
            "{}",
            <_ as Into<Hash>>::into(*b"01234567890123456789013456789012")
        ),
        "3031323334353637383930313233343536373839303133343536373839303132"
    );
    Ok(())
}

#[test]
fn test_display_invalid_utf8() -> Result<(), Error> {
    // BytesM is unchanged.
    assert_eq!(
        format!(
            "{}",
            <_ as TryInto<BytesM>>::try_into(b"hello\xc3\x28world")?
        ),
        "68656c6c6fc328776f726c64"
    );
    // StringM escapes strings.
    assert_eq!(
        format!(
            "{}",
            <_ as TryInto<StringM>>::try_into(b"hello\xc3\x28world")?
        ),
        r"hello\xc3(world"
    );
    Ok(())
}
