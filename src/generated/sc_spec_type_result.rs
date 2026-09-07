#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeResult is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeResult
/// {
///     SCSpecTypeDef okType;
///     SCSpecTypeDef errorType;
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
pub struct ScSpecTypeResult {
    pub ok_type: Box<ScSpecTypeDef>,
    pub error_type: Box<ScSpecTypeDef>,
}

impl ReadXdr for ScSpecTypeResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ok_type: Box::<ScSpecTypeDef>::read_xdr(r)?,
                error_type: Box::<ScSpecTypeDef>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ok_type.write_xdr(w)?;
            self.error_type.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecTypeResultRef is a borrowing equivalent of [`ScSpecTypeResult`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeResultRef<'a> {
    pub ok_type: &'a ScSpecTypeDefRef<'a>,
    pub error_type: &'a ScSpecTypeDefRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecTypeResultRef<'_> {
    type Owned = ScSpecTypeResult;
    fn into_owned(self) -> ScSpecTypeResult {
        ScSpecTypeResult {
            ok_type: Box::new(self.ok_type.into_owned()),
            error_type: Box::new(self.error_type.into_owned()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeResultRef<'_>> for ScSpecTypeResult {
    #[must_use]
    fn from(v: &ScSpecTypeResultRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeResultRef<'_>> for ScSpecTypeResult {
    #[must_use]
    fn from(v: ScSpecTypeResultRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecTypeResultRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ok_type.write_xdr(w)?;
            self.error_type.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ScSpecTypeResultRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_spec_type_result(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_sc_spec_type_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScSpecTypeResult`], mirroring `<ScSpecTypeResult as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_type_result(&mut self, v: &ScSpecTypeResultRef<'_>) {
        self.write_type_sc_spec_type_def(v.ok_type);
        self.write_type_sc_spec_type_def(v.error_type);
    }
}
