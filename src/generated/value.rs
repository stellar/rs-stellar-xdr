#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Value is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque Value<>;
/// ```
///
#[cfg_eval::cfg_eval]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct Value(pub BytesM);


impl From<Value> for BytesM {
    #[must_use]
    fn from(x: Value) -> Self {
        x.0
    }
}

impl From<BytesM> for Value {
    #[must_use]
    fn from(x: BytesM) -> Self {
        Value(x)
    }
}

impl AsRef<BytesM> for Value {
    #[must_use]
    fn as_ref(&self) -> &BytesM {
        &self.0
    }
}

impl ReadXdr for Value {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::read_xdr(r)?;
            let v = Value(i);
            Ok(v)
        })
    }
}

impl WriteXdr for Value {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| { self.0.write_xdr(w) })
    }
}

impl Deref for Value {
    type Target = BytesM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Value> for Vec<u8> {
    #[must_use]
    fn from(x: Value) -> Self {
        x.0.0
    }
}

impl TryFrom<Vec<u8>> for Value {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(Value(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for Value {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(Value(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for Value {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0.0
    }
}

impl AsRef<[u8]> for Value {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0.0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0.0
    }
}
