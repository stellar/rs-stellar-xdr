use stellar_xdr::{BytesM, Error, StringM, VecM};

#[test]
fn test_debug() -> Result<(), Error> {
    // Renders as array.
    assert_eq!(format!("{:?}", <_ as TryInto<VecM<u8>>>::try_into("hello world")?), "VecM([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100])");
    // Renders as hex.
    assert_eq!(format!("{:?}", <_ as TryInto<BytesM>>::try_into("hello world")?), "BytesM(68656c6c6f20776f726c64)");
    // Renders as string.
    assert_eq!(format!("{:?}", <_ as TryInto<StringM>>::try_into("hello world")?), "StringM(hello world)");
    Ok(())
}

#[test]
fn test_debug_invalid_utf8() -> Result<(), Error> {
    // VecM is unchanged.
    assert_eq!(format!("{:?}", <_ as TryInto<VecM<u8>>>::try_into(b"hello\xc3\x28world")?), "VecM([104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100])");
    // BytesM is unchanged.
    assert_eq!(format!("{:?}", <_ as TryInto<BytesM>>::try_into(b"hello\xc3\x28world")?), "BytesM(68656c6c6f20776f726c64)");
    // StringM replaces the invalid sequence with the Unicode replacement character.
    assert_eq!(format!("{:?}", <_ as TryInto<StringM>>::try_into(b"hello\xc3\x28world")?), "StringM(hello�(world)");
    Ok(())
}

#[test]
fn test_display() -> Result<(), Error> {
    // Renders as hex.
    assert_eq!(format!("{}", <_ as TryInto<BytesM>>::try_into("hello world")?), "68656c6c6f20776f726c64");
    // Renders as string.
    assert_eq!(format!("{}", <_ as TryInto<StringM>>::try_into("hello world")?), "hello world");
    Ok(())
}

#[test]
fn test_display_invalid_utf8() -> Result<(), Error> {
    // BytesM is unchanged.
    assert_eq!(format!("{}", <_ as TryInto<BytesM>>::try_into(b"hello\xc3\x28world")?), "68656c6c6f20776f726c64");
    // StringM replaces the invalid sequence with the Unicode replacement character.
    assert_eq!(format!("{}", <_ as TryInto<StringM>>::try_into(b"hello\xc3\x28world")?), "hello�(world");
    Ok(())
}
