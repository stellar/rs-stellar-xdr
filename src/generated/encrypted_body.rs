#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// EncryptedBody is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque EncryptedBody<64000>;
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
pub struct EncryptedBody(pub BytesM::<64000>);


impl From<EncryptedBody> for BytesM::<64000> {
    #[must_use]
    fn from(x: EncryptedBody) -> Self {
        x.0
    }
}

impl From<BytesM::<64000>> for EncryptedBody {
    #[must_use]
    fn from(x: BytesM::<64000>) -> Self {
        EncryptedBody(x)
    }
}

impl AsRef<BytesM::<64000>> for EncryptedBody {
    #[must_use]
    fn as_ref(&self) -> &BytesM::<64000> {
        &self.0
    }
}

impl ReadXdr for EncryptedBody {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::<64000>::read_xdr(r)?;
            let v = EncryptedBody(i);
            Ok(v)
        })
    }
}

impl WriteXdr for EncryptedBody {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| { self.0.write_xdr(w) })
    }
}

impl Deref for EncryptedBody {
    type Target = BytesM::<64000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EncryptedBody> for Vec<u8> {
    #[must_use]
    fn from(x: EncryptedBody) -> Self {
        x.0.0
    }
}

impl TryFrom<Vec<u8>> for EncryptedBody {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(EncryptedBody(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for EncryptedBody {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(EncryptedBody(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for EncryptedBody {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0.0
    }
}

impl AsRef<[u8]> for EncryptedBody {
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
