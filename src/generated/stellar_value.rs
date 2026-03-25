#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// StellarValue is an XDR Struct defined as:
///
/// ```text
/// struct StellarValue
/// {
///     Hash txSetHash;      // transaction set to apply to previous ledger
///     TimePoint closeTime; // network close time
///
///     // upgrades to apply to the previous ledger (usually empty)
///     // this is a vector of encoded 'LedgerUpgrade' so that nodes can drop
///     // unknown steps during consensus if needed.
///     // see notes below on 'LedgerUpgrade' for more detail
///     // max size is dictated by number of upgrade types (+ room for future)
///     UpgradeType upgrades<6>;
///
///     // reserved for future use
///     union switch (StellarValueType v)
///     {
///     case STELLAR_VALUE_BASIC:
///         void;
///     case STELLAR_VALUE_SIGNED:
///         LedgerCloseValueSignature lcValueSignature;
///     }
///     ext;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct StellarValue {
    pub tx_set_hash: Hash,
    pub close_time: TimePoint,
    pub upgrades: VecM<UpgradeType, 6>,
    pub ext: StellarValueExt,
}

impl ReadXdr for StellarValue {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_set_hash: Hash::read_xdr(r)?,
                close_time: TimePoint::read_xdr(r)?,
                upgrades: VecM::<UpgradeType, 6>::read_xdr(r)?,
                ext: StellarValueExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for StellarValue {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_set_hash.write_xdr(w)?;
            self.close_time.write_xdr(w)?;
            self.upgrades.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
