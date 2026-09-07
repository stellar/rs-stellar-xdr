#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanResourcesExtV0 is an XDR Struct defined as:
///
/// ```text
/// struct SorobanResourcesExtV0
/// {
///     // Vector of indices representing what Soroban
///     // entries in the footprint are archived, based on the
///     // order of keys provided in the readWrite footprint.
///     uint32 archivedSorobanEntries<>;
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
pub struct SorobanResourcesExtV0 {
    pub archived_soroban_entries: VecM<u32>,
}

impl ReadXdr for SorobanResourcesExtV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                archived_soroban_entries: VecM::<u32>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanResourcesExtV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.archived_soroban_entries.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanResourcesExtV0Ref is a borrowing equivalent of [`SorobanResourcesExtV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanResourcesExtV0Ref<'a> {
    pub archived_soroban_entries: VecMRef<'a, u32>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanResourcesExtV0Ref<'_> {
    type Owned = SorobanResourcesExtV0;
    fn into_owned(self) -> SorobanResourcesExtV0 {
        SorobanResourcesExtV0 {
            archived_soroban_entries: self.archived_soroban_entries.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanResourcesExtV0Ref<'_>> for SorobanResourcesExtV0 {
    #[must_use]
    fn from(v: &SorobanResourcesExtV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanResourcesExtV0Ref<'_>> for SorobanResourcesExtV0 {
    #[must_use]
    fn from(v: SorobanResourcesExtV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanResourcesExtV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.archived_soroban_entries.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SorobanResourcesExtV0Ref<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_resources_ext_v0(self);
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
        w.write_type_soroban_resources_ext_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanResourcesExtV0`], mirroring `<SorobanResourcesExtV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_resources_ext_v0(&mut self, v: &SorobanResourcesExtV0Ref<'_>) {
        self.write_vec_u32(&v.archived_soroban_entries);
    }
}
