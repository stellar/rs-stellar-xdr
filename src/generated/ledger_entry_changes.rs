#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryChanges is an XDR Typedef defined as:
///
/// ```text
/// typedef LedgerEntryChange LedgerEntryChanges<>;
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
pub struct LedgerEntryChanges(pub VecM<LedgerEntryChange>);

impl From<LedgerEntryChanges> for VecM<LedgerEntryChange> {
    #[must_use]
    fn from(x: LedgerEntryChanges) -> Self {
        x.0
    }
}

impl From<VecM<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn from(x: VecM<LedgerEntryChange>) -> Self {
        LedgerEntryChanges(x)
    }
}

impl AsRef<VecM<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn as_ref(&self) -> &VecM<LedgerEntryChange> {
        &self.0
    }
}

impl ReadXdr for LedgerEntryChanges {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<LedgerEntryChange>::read_xdr(r)?;
            let v = LedgerEntryChanges(i);
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerEntryChanges {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for LedgerEntryChanges {
    type Target = VecM<LedgerEntryChange>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<LedgerEntryChanges> for Vec<LedgerEntryChange> {
    #[must_use]
    fn from(x: LedgerEntryChanges) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<LedgerEntryChange>> for LedgerEntryChanges {
    type Error = Error;
    fn try_from(x: Vec<LedgerEntryChange>) -> Result<Self, Error> {
        Ok(LedgerEntryChanges(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<LedgerEntryChange>> for LedgerEntryChanges {
    type Error = Error;
    fn try_from(x: &Vec<LedgerEntryChange>) -> Result<Self, Error> {
        Ok(LedgerEntryChanges(x.try_into()?))
    }
}

impl AsRef<Vec<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn as_ref(&self) -> &Vec<LedgerEntryChange> {
        &self.0 .0
    }
}

impl AsRef<[LedgerEntryChange]> for LedgerEntryChanges {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[LedgerEntryChange] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[LedgerEntryChange] {
        self.0 .0
    }
}

/// LedgerEntryChangesConst is a borrowing equivalent of [`LedgerEntryChanges`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerEntryChangesConst(pub VecMConst<LedgerEntryChangeConst>);

#[cfg(feature = "const")]
impl LedgerEntryChangesConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_entry_changes(self);
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
        w.write_type_ledger_entry_changes(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerEntryChanges`], mirroring `<LedgerEntryChanges as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_entry_changes(&mut self, v: &LedgerEntryChangesConst) {
        self.write_type_vec_ledger_entry_change(&v.0);
    }
}
