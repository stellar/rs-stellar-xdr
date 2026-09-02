#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// UpgradeEntryMeta is an XDR Struct defined as:
///
/// ```text
/// struct UpgradeEntryMeta
/// {
///     LedgerUpgrade upgrade;
///     LedgerEntryChanges changes;
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
pub struct UpgradeEntryMeta {
    pub upgrade: LedgerUpgrade,
    pub changes: LedgerEntryChanges,
}

impl ReadXdr for UpgradeEntryMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                upgrade: LedgerUpgrade::read_xdr(r)?,
                changes: LedgerEntryChanges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for UpgradeEntryMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.upgrade.write_xdr(w)?;
            self.changes.write_xdr(w)?;
            Ok(())
        })
    }
}

/// UpgradeEntryMetaView is a borrowing equivalent of [`UpgradeEntryMeta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpgradeEntryMetaView<'a> {
    pub upgrade: LedgerUpgrade,
    pub changes: LedgerEntryChangesView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&UpgradeEntryMetaView<'_>> for UpgradeEntryMeta {
    #[must_use]
    fn from(v: &UpgradeEntryMetaView<'_>) -> Self {
        Self {
            upgrade: v.upgrade.clone(),
            changes: (&v.changes).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<UpgradeEntryMetaView<'_>> for UpgradeEntryMeta {
    #[must_use]
    fn from(v: UpgradeEntryMetaView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for UpgradeEntryMetaView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.upgrade.write_xdr(w)?;
            self.changes.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl UpgradeEntryMetaView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_upgrade_entry_meta(self);
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
        w.write_type_upgrade_entry_meta(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`UpgradeEntryMeta`], mirroring `<UpgradeEntryMeta as WriteXdr>::write_xdr`.
    pub const fn write_type_upgrade_entry_meta(&mut self, v: &UpgradeEntryMetaView<'_>) {
        self.write_type_ledger_upgrade(&v.upgrade);
        self.write_type_ledger_entry_changes(&v.changes);
    }

    /// Serializes a variable-length array of [`UpgradeEntryMeta`], mirroring `<VecM<UpgradeEntryMeta, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_upgrade_entry_meta<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, UpgradeEntryMetaView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_upgrade_entry_meta(&s[i]);
            i += 1;
        }
    }
}
