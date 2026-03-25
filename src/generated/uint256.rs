#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// Uint256 is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque uint256[32];
/// ```
///
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
pub struct Uint256(pub [u8; 32]);

impl core::fmt::Debug for Uint256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        write!(f, "Uint256(")?;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
impl core::fmt::Display for Uint256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl core::str::FromStr for Uint256 {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        hex::decode(s).map_err(|_| Error::InvalidHex)?.try_into()
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Uint256 {
    fn schema_name() -> String {
        "Uint256".to_string()
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let schema = String::json_schema(gen);
        if let schemars::schema::Schema::Object(mut schema) = schema {
            schema.extensions.insert(
                "contentEncoding".to_owned(),
                serde_json::Value::String("hex".to_string()),
            );
            schema.extensions.insert(
                "contentMediaType".to_owned(),
                serde_json::Value::String("application/binary".to_string()),
            );
            let string = *schema.string.unwrap_or_default().clone();
            schema.string = Some(Box::new(schemars::schema::StringValidation {
                max_length: 32_u32.checked_mul(2).map(Some).unwrap_or_default(),
                min_length: 32_u32.checked_mul(2).map(Some).unwrap_or_default(),
                ..string
            }));
            schema.into()
        } else {
            schema
        }
    }
}
impl From<Uint256> for [u8; 32] {
    #[must_use]
    fn from(x: Uint256) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for Uint256 {
    #[must_use]
    fn from(x: [u8; 32]) -> Self {
        Uint256(x)
    }
}

impl AsRef<[u8; 32]> for Uint256 {
    #[must_use]
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl ReadXdr for Uint256 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = <[u8; 32]>::read_xdr(r)?;
            let v = Uint256(i);
            Ok(v)
        })
    }
}

impl WriteXdr for Uint256 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Uint256 {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for Uint256 {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for Uint256 {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for Uint256 {
    type Error = Error;
    fn try_from(x: &[u8]) -> Result<Self, Error> {
        Ok(Uint256(x.try_into()?))
    }
}

impl AsRef<[u8]> for Uint256 {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
