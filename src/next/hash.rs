#[cfg(feature = "sha2")]
mod bytes;

#[cfg(feature = "hex")]
impl super::Hash {
    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        Ok(super::Hash(crate::hex::padded_hex_from_str(s)?))
    }
}
