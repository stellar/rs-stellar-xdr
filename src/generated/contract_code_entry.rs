#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractCodeEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractCodeEntry {
///     union switch (int v)
///     {
///         case 0:
///             void;
///         case 1:
///             struct
///             {
///                 ExtensionPoint ext;
///                 ContractCodeCostInputs costInputs;
///             } v1;
///     } ext;
///
///     Hash hash;
///     opaque code<>;
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
pub struct ContractCodeEntry {
    pub ext: ContractCodeEntryExt,
    pub hash: Hash,
    pub code: BytesM,
}

impl ReadXdr for ContractCodeEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ContractCodeEntryExt::read_xdr(r)?,
                hash: Hash::read_xdr(r)?,
                code: BytesM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCodeEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.hash.write_xdr(w)?;
            self.code.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ContractCodeEntryRef is a borrowing equivalent of [`ContractCodeEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCodeEntryRef<'a> {
    pub ext: ContractCodeEntryExt,
    pub hash: Hash,
    pub code: BytesMRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ContractCodeEntryRef<'_> {
    type Owned = ContractCodeEntry;
    fn into_owned(self) -> ContractCodeEntry {
        ContractCodeEntry {
            ext: self.ext.into_owned(),
            hash: self.hash.into_owned(),
            code: self.code.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ContractCodeEntryRef<'_>> for ContractCodeEntry {
    #[must_use]
    fn from(v: &ContractCodeEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ContractCodeEntryRef<'_>> for ContractCodeEntry {
    #[must_use]
    fn from(v: ContractCodeEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ContractCodeEntryRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.hash.write_xdr(w)?;
            self.code.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ContractCodeEntryView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_contract_code_entry(self);
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
        w.write_type_contract_code_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ContractCodeEntry`], mirroring `<ContractCodeEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_contract_code_entry(&mut self, v: &ContractCodeEntryView<'_>) {
        self.write_type_contract_code_entry_ext(&v.ext);
        self.write_type_hash(&v.hash);
        self.write_var_opaque(v.code.as_slice());
    }
}
