#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SError is an XDR Struct defined as:
///
/// ```text
/// struct Error
/// {
///     ErrorCode code;
///     string msg<100>;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SError {
    pub code: ErrorCode,
    pub msg: StringM<100>,
}

impl ReadXdr for SError {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                code: ErrorCode::read_xdr(r)?,
                msg: StringM::<100>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SError {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.code.write_xdr(w)?;
            self.msg.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SErrorRef is a borrowing equivalent of [`SError`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SErrorRef<'a> {
    pub code: ErrorCode,
    pub msg: StringMRef<'a, 100>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SErrorRef<'_> {
    type Owned = SError;
    fn into_owned(self) -> SError {
        SError {
            code: self.code.into_owned(),
            msg: self.msg.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SErrorRef<'_>> for SError {
    #[must_use]
    fn from(v: &SErrorRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SErrorRef<'_>> for SError {
    #[must_use]
    fn from(v: SErrorRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SErrorRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.code.write_xdr(w)?;
            self.msg.write_xdr(w)?;
            Ok(())
        })
    }
}
