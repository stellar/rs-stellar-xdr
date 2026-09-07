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

/// FreezeBypassTxsDeltaView is a borrowing equivalent of [`FreezeBypassTxsDelta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreezeBypassTxsDeltaView<'a> {
    pub add_txs: VecMView<'a, Hash>,
    pub remove_txs: VecMView<'a, Hash>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for FreezeBypassTxsDeltaView<'_> {
    type Owned = FreezeBypassTxsDelta;
    fn into_owned(self) -> FreezeBypassTxsDelta {
        FreezeBypassTxsDelta {
            add_txs: self.add_txs.into_owned(),
            remove_txs: self.remove_txs.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&FreezeBypassTxsDeltaView<'_>> for FreezeBypassTxsDelta {
    #[must_use]
    fn from(v: &FreezeBypassTxsDeltaView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<FreezeBypassTxsDeltaView<'_>> for FreezeBypassTxsDelta {
    #[must_use]
    fn from(v: FreezeBypassTxsDeltaView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for FreezeBypassTxsDeltaView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.add_txs.write_xdr(w)?;
            self.remove_txs.write_xdr(w)?;
            Ok(())
        })
    }
}
