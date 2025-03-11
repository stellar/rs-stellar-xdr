pub(crate) fn padded_hex_from_str(s: &str) -> Result<[u8; 32], hex::FromHexError> {
    let n = 32;
    if s.len() > n * 2 {
        return Err(hex::FromHexError::InvalidStringLength);
    }
    let mut decoded = [0u8; 32];
    let mut padded = [b'0'; 64]; // 32 bytes * 2 chars per byte
    padded[(64 - s.len())..].copy_from_slice(s.as_bytes());
    hex::decode_to_slice(padded, &mut decoded)?;
    Ok(decoded)
}
