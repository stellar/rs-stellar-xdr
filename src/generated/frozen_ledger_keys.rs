#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FrozenLedgerKeys is an XDR Struct defined as:
///
/// ```text
/// struct FrozenLedgerKeys {
///     EncodedLedgerKey keys<>;
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
pub struct FrozenLedgerKeys {
    pub keys: VecM<EncodedLedgerKey>,
}

impl ReadXdr for FrozenLedgerKeys {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                keys: VecM::<EncodedLedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FrozenLedgerKeys {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.keys.write_xdr(w)?;
            Ok(())
        })
    }
}

/// FrozenLedgerKeysRef is a borrowing equivalent of [`FrozenLedgerKeys`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrozenLedgerKeysRef<'a> {
    pub keys: VecMRef<'a, EncodedLedgerKeyRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&FrozenLedgerKeysRef<'_>> for FrozenLedgerKeys {
    #[must_use]
    fn from(v: &FrozenLedgerKeysRef<'_>) -> Self {
        Self {
            keys: v.keys.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<FrozenLedgerKeysRef<'_>> for FrozenLedgerKeys {
    #[must_use]
    fn from(v: FrozenLedgerKeysRef<'_>) -> Self {
        Self::from(&v)
    }
}
