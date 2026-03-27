#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FreezeBypassTxsDelta is an XDR Struct defined as:
///
/// ```text
/// struct FreezeBypassTxsDelta {
///     Hash addTxs<>;
///     Hash removeTxs<>;
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
pub struct FreezeBypassTxsDelta {
    pub add_txs: VecM<Hash>,
    pub remove_txs: VecM<Hash>,
}

impl ReadXdr for FreezeBypassTxsDelta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                add_txs: VecM::<Hash>::read_xdr(r)?,
                remove_txs: VecM::<Hash>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FreezeBypassTxsDelta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.add_txs.write_xdr(w)?;
            self.remove_txs.write_xdr(w)?;
            Ok(())
        })
    }
}
