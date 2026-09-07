#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FrozenLedgerKeysDelta is an XDR Struct defined as:
///
/// ```text
/// struct FrozenLedgerKeysDelta {
///     EncodedLedgerKey keysToFreeze<>;
///     EncodedLedgerKey keysToUnfreeze<>;
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
pub struct FrozenLedgerKeysDelta {
    pub keys_to_freeze: VecM<EncodedLedgerKey>,
    pub keys_to_unfreeze: VecM<EncodedLedgerKey>,
}

impl ReadXdr for FrozenLedgerKeysDelta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                keys_to_freeze: VecM::<EncodedLedgerKey>::read_xdr(r)?,
                keys_to_unfreeze: VecM::<EncodedLedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FrozenLedgerKeysDelta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.keys_to_freeze.write_xdr(w)?;
            self.keys_to_unfreeze.write_xdr(w)?;
            Ok(())
        })
    }
}

/// FrozenLedgerKeysDeltaView is a borrowing equivalent of [`FrozenLedgerKeysDelta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrozenLedgerKeysDeltaView<'a> {
    pub keys_to_freeze: VecMView<'a, EncodedLedgerKeyView<'a>>,
    pub keys_to_unfreeze: VecMView<'a, EncodedLedgerKeyView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for FrozenLedgerKeysDeltaView<'_> {
    type Owned = FrozenLedgerKeysDelta;
    fn into_owned(self) -> FrozenLedgerKeysDelta {
        FrozenLedgerKeysDelta {
            keys_to_freeze: self.keys_to_freeze.into_owned(),
            keys_to_unfreeze: self.keys_to_unfreeze.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&FrozenLedgerKeysDeltaView<'_>> for FrozenLedgerKeysDelta {
    #[must_use]
    fn from(v: &FrozenLedgerKeysDeltaView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<FrozenLedgerKeysDeltaView<'_>> for FrozenLedgerKeysDelta {
    #[must_use]
    fn from(v: FrozenLedgerKeysDeltaView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for FrozenLedgerKeysDeltaView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.keys_to_freeze.write_xdr(w)?;
            self.keys_to_unfreeze.write_xdr(w)?;
            Ok(())
        })
    }
}
