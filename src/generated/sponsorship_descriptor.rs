#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SponsorshipDescriptor is an XDR Typedef defined as:
///
/// ```text
/// typedef AccountID* SponsorshipDescriptor;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct SponsorshipDescriptor(pub Option<AccountId>);

impl From<SponsorshipDescriptor> for Option<AccountId> {
    #[must_use]
    fn from(x: SponsorshipDescriptor) -> Self {
        x.0
    }
}

impl From<Option<AccountId>> for SponsorshipDescriptor {
    #[must_use]
    fn from(x: Option<AccountId>) -> Self {
        SponsorshipDescriptor(x)
    }
}

impl AsRef<Option<AccountId>> for SponsorshipDescriptor {
    #[must_use]
    fn as_ref(&self) -> &Option<AccountId> {
        &self.0
    }
}

impl ReadXdr for SponsorshipDescriptor {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = Option::<AccountId>::read_xdr(r)?;
            let v = SponsorshipDescriptor(i);
            Ok(v)
        })
    }
}

impl WriteXdr for SponsorshipDescriptor {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for SponsorshipDescriptor {
    type Owned = SponsorshipDescriptor;
    fn into_owned(self) -> SponsorshipDescriptor {
        self
    }
}

#[cfg(feature = "const")]
impl SponsorshipDescriptor {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sponsorship_descriptor(self);
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
        w.write_type_sponsorship_descriptor(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SponsorshipDescriptor`], mirroring `<SponsorshipDescriptor as WriteXdr>::write_xdr`.
    pub const fn write_type_sponsorship_descriptor(&mut self, v: &SponsorshipDescriptor) {
        self.write_type_option_account_id(&v.0);
    }

    /// Serializes a variable-length array of [`SponsorshipDescriptor`], mirroring `<VecM<SponsorshipDescriptor, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_sponsorship_descriptor<const MAX: u32>(
        &mut self,
        v: &VecMRef<'_, SponsorshipDescriptor, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_sponsorship_descriptor(&s[i]);
            i += 1;
        }
    }
}
