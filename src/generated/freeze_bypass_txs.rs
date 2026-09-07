#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FreezeBypassTxs is an XDR Struct defined as:
///
/// ```text
/// struct FreezeBypassTxs {
///     Hash txHashes<>;
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
pub struct FreezeBypassTxs {
    pub tx_hashes: VecM<Hash>,
}

impl ReadXdr for FreezeBypassTxs {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_hashes: VecM::<Hash>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FreezeBypassTxs {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_hashes.write_xdr(w)?;
            Ok(())
        })
    }
}

/// FreezeBypassTxsView is a borrowing equivalent of [`FreezeBypassTxs`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreezeBypassTxsView<'a> {
    pub tx_hashes: VecMView<'a, Hash>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for FreezeBypassTxsView<'_> {
    type Owned = FreezeBypassTxs;
    fn into_owned(&self) -> FreezeBypassTxs {
        FreezeBypassTxs {
            tx_hashes: self.tx_hashes.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&FreezeBypassTxsView<'_>> for FreezeBypassTxs {
    #[must_use]
    fn from(v: &FreezeBypassTxsView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<FreezeBypassTxsView<'_>> for FreezeBypassTxs {
    #[must_use]
    fn from(v: FreezeBypassTxsView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for FreezeBypassTxsView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_hashes.write_xdr(w)?;
            Ok(())
        })
    }
}
