#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAuthorizationEntries is an XDR Typedef defined as:
///
/// ```text
/// typedef SorobanAuthorizationEntry SorobanAuthorizationEntries<>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
pub struct SorobanAuthorizationEntries(pub VecM<SorobanAuthorizationEntry>);

impl From<SorobanAuthorizationEntries> for VecM<SorobanAuthorizationEntry> {
    #[must_use]
    fn from(x: SorobanAuthorizationEntries) -> Self {
        x.0
    }
}

impl From<VecM<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn from(x: VecM<SorobanAuthorizationEntry>) -> Self {
        SorobanAuthorizationEntries(x)
    }
}

impl AsRef<VecM<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn as_ref(&self) -> &VecM<SorobanAuthorizationEntry> {
        &self.0
    }
}

impl ReadXdr for SorobanAuthorizationEntries {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<SorobanAuthorizationEntry>::read_xdr(r)?;
            let v = SorobanAuthorizationEntries(i);
            Ok(v)
        })
    }
}

impl WriteXdr for SorobanAuthorizationEntries {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for SorobanAuthorizationEntries {
    type Target = VecM<SorobanAuthorizationEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SorobanAuthorizationEntries> for Vec<SorobanAuthorizationEntry> {
    #[must_use]
    fn from(x: SorobanAuthorizationEntries) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(x: Vec<SorobanAuthorizationEntry>) -> Result<Self, Error> {
        Ok(SorobanAuthorizationEntries(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    type Error = Error;
    fn try_from(x: &Vec<SorobanAuthorizationEntry>) -> Result<Self, Error> {
        Ok(SorobanAuthorizationEntries(x.try_into()?))
    }
}

impl AsRef<Vec<SorobanAuthorizationEntry>> for SorobanAuthorizationEntries {
    #[must_use]
    fn as_ref(&self) -> &Vec<SorobanAuthorizationEntry> {
        &self.0 .0
    }
}

impl AsRef<[SorobanAuthorizationEntry]> for SorobanAuthorizationEntries {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[SorobanAuthorizationEntry] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[SorobanAuthorizationEntry] {
        self.0 .0
    }
}

/// SorobanAuthorizationEntriesView is a borrowing equivalent of [`SorobanAuthorizationEntries`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAuthorizationEntriesView<'a>(pub VecMView<'a, SorobanAuthorizationEntryView<'a>>);

#[cfg(feature = "alloc")]
impl From<&SorobanAuthorizationEntriesView<'_>> for SorobanAuthorizationEntries {
    #[must_use]
    fn from(v: &SorobanAuthorizationEntriesView<'_>) -> Self {
        Self(v.0.to_vecm())
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAuthorizationEntriesView<'_>> for SorobanAuthorizationEntries {
    #[must_use]
    fn from(v: SorobanAuthorizationEntriesView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SorobanAuthorizationEntriesView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "const")]
impl SorobanAuthorizationEntriesView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_authorization_entries(self);
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
        w.write_type_soroban_authorization_entries(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanAuthorizationEntries`], mirroring `<SorobanAuthorizationEntries as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_authorization_entries(
        &mut self,
        v: &SorobanAuthorizationEntriesView<'_>,
    ) {
        self.write_type_vec_soroban_authorization_entry(&v.0);
    }
}
