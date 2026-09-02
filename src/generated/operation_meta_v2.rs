#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OperationMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct OperationMetaV2
/// {
///     ExtensionPoint ext;
///
///     LedgerEntryChanges changes;
///
///     ContractEvent events<>;
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
pub struct OperationMetaV2 {
    pub ext: ExtensionPoint,
    pub changes: LedgerEntryChanges,
    pub events: VecM<ContractEvent>,
}

impl ReadXdr for OperationMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                changes: LedgerEntryChanges::read_xdr(r)?,
                events: VecM::<ContractEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for OperationMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.changes.write_xdr(w)?;
            self.events.write_xdr(w)?;
            Ok(())
        })
    }
}

/// OperationMetaV2Ref is a borrowing equivalent of [`OperationMetaV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationMetaV2Ref<'a> {
    pub ext: ExtensionPoint,
    pub changes: LedgerEntryChangesRef<'a>,
    pub events: VecMRef<'a, ContractEventRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for OperationMetaV2Ref<'_> {
    type Owned = OperationMetaV2;
    fn into_owned(self) -> OperationMetaV2 {
        OperationMetaV2 {
            ext: self.ext.into_owned(),
            changes: self.changes.into_owned(),
            events: self.events.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&OperationMetaV2Ref<'_>> for OperationMetaV2 {
    #[must_use]
    fn from(v: &OperationMetaV2Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<OperationMetaV2Ref<'_>> for OperationMetaV2 {
    #[must_use]
    fn from(v: OperationMetaV2Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for OperationMetaV2Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.changes.write_xdr(w)?;
            self.events.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl OperationMetaV2View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_operation_meta_v2(self);
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
        w.write_type_operation_meta_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`OperationMetaV2`], mirroring `<OperationMetaV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_operation_meta_v2(&mut self, v: &OperationMetaV2View<'_>) {
        self.write_type_extension_point(&v.ext);
        self.write_type_ledger_entry_changes(&v.changes);
        self.write_type_vec_contract_event(&v.events);
    }

    /// Serializes a variable-length array of [`OperationMetaV2`], mirroring `<VecM<OperationMetaV2, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_operation_meta_v2<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, OperationMetaV2View<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_operation_meta_v2(&s[i]);
            i += 1;
        }
    }
}
