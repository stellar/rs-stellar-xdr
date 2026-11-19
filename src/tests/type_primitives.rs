#![cfg(all(feature = "type_enum", feature = "base64", feature = "serde_json"))]

use std::str::FromStr;

use crate::{BytesM, Error, Limited, Limits, Type, TypeVariant, WriteXdr};

fn variant(name: &str) -> TypeVariant {
    TypeVariant::from_str(name).unwrap()
}

#[test]
fn names() {
    for (name, v, display) in [
        ("int", TypeVariant::Int, "int"),
        ("unsigned int", TypeVariant::UnsignedInt, "unsigned int"),
        ("hyper", TypeVariant::Hyper, "hyper"),
        ("unsigned hyper", TypeVariant::UnsignedHyper, "unsigned hyper"),
        ("bool", TypeVariant::Bool, "bool"),
        ("opaque[32]", TypeVariant::OpaqueFixed(32), "opaque[32]"),
        ("opaque<64>", TypeVariant::OpaqueVar(64), "opaque<64>"),
        ("opaque<>", TypeVariant::OpaqueVar(u32::MAX), "opaque<>"),
        ("opaque<4294967295>", TypeVariant::OpaqueVar(u32::MAX), "opaque<>"),
        ("string<28>", TypeVariant::StringVar(28), "string<28>"),
        ("string<>", TypeVariant::StringVar(u32::MAX), "string<>"),
    ] {
        assert_eq!(variant(name), v, "{name}");
        assert_eq!(v.to_string(), display, "{name}");
    }
    for name in ["opaque[]", "opaque[x]", "opaque<-1>", "string[4]", "Int", "uint"] {
        assert_eq!(TypeVariant::from_str(name), Err(Error::Invalid), "{name}");
    }
    assert_eq!(TypeVariant::OpaqueFixed(32).name(), "opaque[n]");
    assert!(TypeVariant::VARIANTS_STR.contains(&"unsigned hyper"));
}

#[test]
fn xdr_and_json() {
    for (name, xdr, json) in [
        ("int", "/////w==", "-1"),
        ("unsigned int", "/////w==", "4294967295"),
        ("hyper", "f/////////8=", "\"9223372036854775807\""),
        ("unsigned hyper", "AAAAAAAAAAU=", "\"5\""),
        ("bool", "AAAAAQ==", "true"),
        ("opaque[3]", "AQIDAA==", "\"010203\""),
        ("opaque<4>", "AAAAAwECAwA=", "\"010203\""),
        ("opaque<>", "AAAAAA==", "\"\""),
        ("string<5>", "AAAABWhlbGxvAAAA", "\"hello\""),
    ] {
        let v = variant(name);
        let t = Type::from_xdr_base64(v, xdr, Limits::none()).unwrap();
        assert_eq!(t.variant(), v, "{name}");
        assert_eq!(t.to_xdr_base64(Limits::none()).unwrap(), xdr, "{name}");
        assert_eq!(serde_json::to_string(&t).unwrap(), json, "{name}");
        assert_eq!(Type::from_json(v, json.as_bytes()).unwrap(), t, "{name}");
    }
}

#[test]
fn json_hyper_accepts_number() {
    let t = Type::from_json(TypeVariant::Hyper, "5".as_bytes()).unwrap();
    assert_eq!(t, Type::Hyper(Box::new(5)));
}

#[test]
fn length_errors() {
    let l = Limits::none();
    // Length prefix of 4 for an opaque<3>.
    assert_eq!(
        Type::from_xdr_base64(variant("opaque<3>"), "AAAABAECAwQ=", l.clone()),
        Err(Error::LengthExceedsMax)
    );
    // Padding of 4 after an opaque[3].
    assert_eq!(
        Type::from_xdr_base64(variant("opaque[3]"), "AQIDBA==", l.clone()),
        Err(Error::NonZeroPadding)
    );
    assert!(matches!(
        Type::from_json(variant("opaque[4]"), "\"010203\"".as_bytes()),
        Err(Error::LengthMismatch)
    ));
    assert!(matches!(
        Type::from_json(variant("string<2>"), "\"abc\"".as_bytes()),
        Err(Error::LengthExceedsMax)
    ));
    let t = Type::OpaqueFixed(4, Box::new(BytesM::try_from([1, 2, 3]).unwrap()));
    assert_eq!(t.to_xdr(l), Err(Error::LengthMismatch));
}

#[test]
fn read_iters() {
    let v = TypeVariant::Int;
    let mut r = Limited::new("AAAAAQAAAAI=".as_bytes(), Limits::none());
    let ts = Type::read_xdr_base64_iter(v, &mut r)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(ts, [Type::Int(Box::new(1)), Type::Int(Box::new(2))]);

    // Two frames, each holding an int.
    let framed = [
        0x80, 0, 0, 4, 0, 0, 0, 1, //
        0x80, 0, 0, 4, 0, 0, 0, 2,
    ];
    let mut r = Limited::new(&framed[..], Limits::none());
    let ts = Type::read_xdr_framed_iter(v, &mut r)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(ts, [Type::Int(Box::new(1)), Type::Int(Box::new(2))]);
}

#[test]
fn default() {
    assert_eq!(
        serde_json::to_string(&Type::default(variant("opaque[3]"))).unwrap(),
        "\"000000\""
    );
    assert_eq!(
        serde_json::to_string(&Type::default(TypeVariant::Hyper)).unwrap(),
        "\"0\""
    );
}
