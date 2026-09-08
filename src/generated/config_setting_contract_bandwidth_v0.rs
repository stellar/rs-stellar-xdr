#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingContractBandwidthV0 is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingContractBandwidthV0
/// {
///     // Maximum sum of all transaction sizes in the ledger in bytes
///     uint32 ledgerMaxTxsSizeBytes;
///     // Maximum size in bytes for a transaction
///     uint32 txMaxSizeBytes;
///
///     // Fee for 1 KB of transaction size
///     int64 feeTxSize1KB;
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
pub struct ConfigSettingContractBandwidthV0 {
    pub ledger_max_txs_size_bytes: u32,
    pub tx_max_size_bytes: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_tx_size1_kb: i64,
}

impl ReadXdr for ConfigSettingContractBandwidthV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_max_txs_size_bytes: u32::read_xdr(r)?,
                tx_max_size_bytes: u32::read_xdr(r)?,
                fee_tx_size1_kb: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingContractBandwidthV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_max_txs_size_bytes.write_xdr(w)?;
            self.tx_max_size_bytes.write_xdr(w)?;
            self.fee_tx_size1_kb.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ConfigSettingContractBandwidthV0 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_config_setting_contract_bandwidth_v0(self);
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
        w.write_type_config_setting_contract_bandwidth_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ConfigSettingContractBandwidthV0`], mirroring `<ConfigSettingContractBandwidthV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_config_setting_contract_bandwidth_v0(
        &mut self,
        v: &ConfigSettingContractBandwidthV0,
    ) {
        self.write_u32(v.ledger_max_txs_size_bytes);
        self.write_u32(v.tx_max_size_bytes);
        self.write_i64(v.fee_tx_size1_kb);
    }
}
