#![cfg(feature = "cli")]

use crate::{Frame, Limits, ReadXdr, Uint256, WriteXdr};
use std::io::Cursor;

#[test]
fn test_frame_write_xdr() -> Result<(), crate::Error> {
    let val = Uint256([0x42; 32]);
    let framed = Frame(val.clone());

    let bytes = framed.to_xdr(Limits::none())?;
    assert_eq!(bytes.len(), 4 + 32);

    // RFC 5531 Section 11 Record Marking:
    // Highest bit set (0x8000_0000) for last fragment, lower 31 bits are length (32)
    let expected_header: [u8; 4] = (0x8000_0000u32 | 32).to_be_bytes();
    assert_eq!(&bytes[..4], &expected_header);
    assert_eq!(&bytes[4..], &[0x42; 32]);

    // Verify ReadXdr on Frame parses it back exactly
    let mut cursor = crate::Limited::new(Cursor::new(&bytes), Limits::none());
    let read_framed: Frame<Uint256> = Frame::read_xdr(&mut cursor)?;
    assert_eq!(read_framed.0, val);

    Ok(())
}

#[test]
fn test_encode_stream_and_stream_base64_and_framed() -> Result<(), Box<dyn std::error::Error>> {
    let json_input = r#"
        "0000000000000000000000000000000000000000000000000000000000000001"
        "0000000000000000000000000000000000000000000000000000000000000002"
    "#;

    let r#type = crate::TypeVariant::Uint256;

    // Test stream encoding
    let mut stream_bytes = Vec::new();
    let mut de =
        serde_json::Deserializer::new(serde_json::de::IoRead::new(Cursor::new(json_input)));
    loop {
        let (t, _) = match crate::Type::deserialize_json_collecting_ignored_fields(r#type, &mut de)
        {
            Ok(r) => r,
            Err(crate::Error::Json(ref inner)) if inner.is_eof() => break,
            Err(e) => return Err(e.into()),
        };
        stream_bytes.extend_from_slice(&t.to_xdr(Limits::none())?);
    }
    assert_eq!(stream_bytes.len(), 64);

    // Verify read_xdr_iter on the stream bytes
    let mut l = crate::Limited::new(Cursor::new(&stream_bytes), Limits::none());
    let items: Vec<crate::Type> =
        crate::Type::read_xdr_iter(r#type, &mut l).collect::<Result<_, _>>()?;
    assert_eq!(items.len(), 2);

    // Test stream-framed encoding
    let mut framed_bytes = Vec::new();
    let mut de =
        serde_json::Deserializer::new(serde_json::de::IoRead::new(Cursor::new(json_input)));
    loop {
        let (t, _) = match crate::Type::deserialize_json_collecting_ignored_fields(r#type, &mut de)
        {
            Ok(r) => r,
            Err(crate::Error::Json(ref inner)) if inner.is_eof() => break,
            Err(e) => return Err(e.into()),
        };
        let b = t.to_xdr(Limits::none())?;
        let len: u32 = b.len().try_into()?;
        let header = (0x8000_0000u32 | len).to_be_bytes();
        framed_bytes.extend_from_slice(&header);
        framed_bytes.extend_from_slice(&b);
    }
    assert_eq!(framed_bytes.len(), (4 + 32) * 2);

    // Verify read_xdr_framed_iter on the framed bytes
    let mut l_framed = crate::Limited::new(Cursor::new(&framed_bytes), Limits::none());
    let framed_items: Vec<crate::Type> =
        crate::Type::read_xdr_framed_iter(r#type, &mut l_framed).collect::<Result<_, _>>()?;
    assert_eq!(framed_items.len(), 2);
    assert_eq!(framed_items, items);

    // Test stream-base64 encoding
    let mut base64_lines = Vec::new();
    let mut de =
        serde_json::Deserializer::new(serde_json::de::IoRead::new(Cursor::new(json_input)));
    loop {
        let (t, _) = match crate::Type::deserialize_json_collecting_ignored_fields(r#type, &mut de)
        {
            Ok(r) => r,
            Err(crate::Error::Json(ref inner)) if inner.is_eof() => break,
            Err(e) => return Err(e.into()),
        };
        base64_lines.push(t.to_xdr_base64(Limits::none())?);
    }
    assert_eq!(base64_lines.len(), 2);
    // Verify each base64 string decodes back to the expected Uint256
    for (line, expected_type) in base64_lines.iter().zip(items.iter()) {
        let decoded = crate::Type::from_xdr_base64(r#type, line, Limits::none())?;
        assert_eq!(&decoded, expected_type);
    }

    Ok(())
}
