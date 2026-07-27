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

/// FreezeBypassTxsRef is a borrowing equivalent of [`FreezeBypassTxs`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreezeBypassTxsRef<'a> {
    pub tx_hashes: VecMRef<'a, Hash>,
}

#[cfg(feature = "alloc")]
impl From<&FreezeBypassTxsRef<'_>> for FreezeBypassTxs {
    #[must_use]
    fn from(v: &FreezeBypassTxsRef<'_>) -> Self {
        Self {
            tx_hashes: v.tx_hashes.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<FreezeBypassTxsRef<'_>> for FreezeBypassTxs {
    #[must_use]
    fn from(v: FreezeBypassTxsRef<'_>) -> Self {
        Self::from(&v)
    }
}
