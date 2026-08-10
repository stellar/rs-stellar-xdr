#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecEntryV2 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecEntryV2
/// {
///     opaque id[SC_SPEC_ID_LEN];
///     union switch (SCSpecEntryKind kind)
///     {
///     case SC_SPEC_ENTRY_FUNCTION_V0:
///         SCSpecFunctionV0 functionV0;
///     case SC_SPEC_ENTRY_UDT_STRUCT_V0:
///         SCSpecUDTStructV0 udtStructV0;
///     case SC_SPEC_ENTRY_UDT_UNION_V0:
///         SCSpecUDTUnionV0 udtUnionV0;
///     case SC_SPEC_ENTRY_UDT_ENUM_V0:
///         SCSpecUDTEnumV0 udtEnumV0;
///     case SC_SPEC_ENTRY_UDT_ERROR_ENUM_V0:
///         SCSpecUDTErrorEnumV0 udtErrorEnumV0;
///     case SC_SPEC_ENTRY_EVENT_V0:
///         SCSpecEventV0 eventV0;
///     } body;
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
pub struct ScSpecEntryV2 {
    pub id: [u8; 8],
    pub body: ScSpecEntryV2Body,
}

impl ReadXdr for ScSpecEntryV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                id: <[u8; 8]>::read_xdr(r)?,
                body: ScSpecEntryV2Body::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecEntryV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.body.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecEntryV2View is a borrowing equivalent of [`ScSpecEntryV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecEntryV2View<'a> {
    pub id: [u8; 8],
    pub body: ScSpecEntryV2BodyView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecEntryV2View<'_>> for ScSpecEntryV2 {
    #[must_use]
    fn from(v: &ScSpecEntryV2View<'_>) -> Self {
        Self {
            id: v.id,
            body: (&v.body).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecEntryV2View<'_>> for ScSpecEntryV2 {
    #[must_use]
    fn from(v: ScSpecEntryV2View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecEntryV2View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.body.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(feature = "const")]
impl ScSpecEntryV2View<'_> {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to the owned type's
    /// [`WriteXdr::write_xdr`].
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_fixed_opaque(&self.id);
        self.body.const_write_xdr(w);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
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
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}
