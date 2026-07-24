#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// StellarValueProposedValue is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             Hash txSetHash;
///             Hash previousLedgerHash;
///             uint32 previousLedgerVersion;
///             LedgerCloseValueSignature lcValueSignature;
///         }
/// ```
///
#[cfg(feature = "cap_0083")]
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
pub struct StellarValueProposedValue {
    pub tx_set_hash: Hash,
    pub previous_ledger_hash: Hash,
    pub previous_ledger_version: u32,
    pub lc_value_signature: LedgerCloseValueSignature,
}

#[cfg(feature = "cap_0083")]
impl ReadXdr for StellarValueProposedValue {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_set_hash: Hash::read_xdr(r)?,
                previous_ledger_hash: Hash::read_xdr(r)?,
                previous_ledger_version: u32::read_xdr(r)?,
                lc_value_signature: LedgerCloseValueSignature::read_xdr(r)?,
            })
        })
    }
}

#[cfg(feature = "cap_0083")]
impl StellarValueProposedValue {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.tx_set_hash.const_to_xdr(w);
        self.previous_ledger_hash.const_to_xdr(w);
        w.write_u32(self.previous_ledger_version);
        self.lc_value_signature.const_to_xdr(w);
        w.leave_depth();
    }
}

#[cfg(feature = "cap_0083")]
impl WriteXdr for StellarValueProposedValue {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_set_hash.write_xdr(w)?;
            self.previous_ledger_hash.write_xdr(w)?;
            self.previous_ledger_version.write_xdr(w)?;
            self.lc_value_signature.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
