#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingContractComputeV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractComputeV0
/// {
///     // Maximum instructions per ledger
///     int64 ledgerMaxInstructions;
///     // Maximum instructions per transaction
///     int64 txMaxInstructions;
///     // Cost of 10000 instructions
///     int64 feeRatePerInstructionsIncrement;
///
///     // Memory limit per transaction. Unlike instructions, there is no fee
///     // for memory, just the limit.
///     uint32 txMemoryLimit;
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
pub struct ConfigSettingContractComputeV0 {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub ledger_max_instructions: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub tx_max_instructions: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_rate_per_instructions_increment: i64,
    pub tx_memory_limit: u32,
}

impl ReadXdr for ConfigSettingContractComputeV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_instructions: i64::read_xdr(r)?,
                tx_max_instructions: i64::read_xdr(r)?,
                fee_rate_per_instructions_increment: i64::read_xdr(r)?,
                tx_memory_limit: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractComputeV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_instructions.write_xdr(w)?;
            self.tx_max_instructions.write_xdr(w)?;
            self.fee_rate_per_instructions_increment.write_xdr(w)?;
            self.tx_memory_limit.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for ConfigSettingContractComputeV0 {
    type Owned = ConfigSettingContractComputeV0;
    fn into_owned(self) -> ConfigSettingContractComputeV0 {
        self
    }
}

#[cfg(feature = "const")]
impl ConfigSettingContractComputeV0 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_config_setting_contract_compute_v0(self);
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
        w.write_type_config_setting_contract_compute_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ConfigSettingContractComputeV0`], mirroring `<ConfigSettingContractComputeV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_config_setting_contract_compute_v0(
        &mut self,
        v: &ConfigSettingContractComputeV0,
    ) {
        self.write_i64(v.ledger_max_instructions);
        self.write_i64(v.tx_max_instructions);
        self.write_i64(v.fee_rate_per_instructions_increment);
        self.write_u32(v.tx_memory_limit);
    }
}
