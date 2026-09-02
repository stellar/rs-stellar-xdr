#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractDataEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractDataEntry {
///     ExtensionPoint ext;
///
///     SCAddress contract;
///     SCVal key;
///     ContractDataDurability durability;
///     SCVal val;
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
pub struct ContractDataEntry {
    pub ext: ExtensionPoint,
    pub contract: ScAddress,
    pub key: ScVal,
    pub durability: ContractDataDurability,
    pub val: ScVal,
}

impl ReadXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                contract: ScAddress::read_xdr(r)?,
                key: ScVal::read_xdr(r)?,
                durability: ContractDataDurability::read_xdr(r)?,
                val: ScVal::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.contract.write_xdr(w)?;
            self.key.write_xdr(w)?;
            self.durability.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ContractDataEntryRef is a borrowing equivalent of [`ContractDataEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractDataEntryRef<'a> {
    pub ext: ExtensionPoint,
    pub contract: ScAddress,
    pub key: ScValRef<'a>,
    pub durability: ContractDataDurability,
    pub val: ScValRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ContractDataEntryRef<'_> {
    type Owned = ContractDataEntry;
    fn into_owned(self) -> ContractDataEntry {
        ContractDataEntry {
            ext: self.ext.into_owned(),
            contract: self.contract.into_owned(),
            key: self.key.into_owned(),
            durability: self.durability.into_owned(),
            val: self.val.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ContractDataEntryRef<'_>> for ContractDataEntry {
    #[must_use]
    fn from(v: &ContractDataEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ContractDataEntryRef<'_>> for ContractDataEntry {
    #[must_use]
    fn from(v: ContractDataEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ContractDataEntryRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.contract.write_xdr(w)?;
            self.key.write_xdr(w)?;
            self.durability.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ContractDataEntryView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_contract_data_entry(self);
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
        w.write_type_contract_data_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ContractDataEntry`], mirroring `<ContractDataEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_contract_data_entry(&mut self, v: &ContractDataEntryView<'_>) {
        self.write_type_extension_point(&v.ext);
        self.write_type_sc_address(&v.contract);
        self.write_type_sc_val(&v.key);
        self.write_type_contract_data_durability(&v.durability);
        self.write_type_sc_val(&v.val);
    }
}
