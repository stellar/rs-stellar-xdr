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

/// FreezeBypassTxsDeltaRef is a borrowing equivalent of [`FreezeBypassTxsDelta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreezeBypassTxsDeltaRef<'a> {
    pub add_txs: VecMRef<'a, Hash>,
    pub remove_txs: VecMRef<'a, Hash>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for FreezeBypassTxsDeltaRef<'_> {
    type Owned = FreezeBypassTxsDelta;
    fn into_owned(self) -> FreezeBypassTxsDelta {
        FreezeBypassTxsDelta {
            add_txs: self.add_txs.into_owned(),
            remove_txs: self.remove_txs.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&FreezeBypassTxsDeltaRef<'_>> for FreezeBypassTxsDelta {
    #[must_use]
    fn from(v: &FreezeBypassTxsDeltaRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<FreezeBypassTxsDeltaRef<'_>> for FreezeBypassTxsDelta {
    #[must_use]
    fn from(v: FreezeBypassTxsDeltaRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for FreezeBypassTxsDeltaRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.add_txs.write_xdr(w)?;
            self.remove_txs.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl FreezeBypassTxsDeltaView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_freeze_bypass_txs_delta(self);
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
        w.write_type_freeze_bypass_txs_delta(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`FreezeBypassTxsDelta`], mirroring `<FreezeBypassTxsDelta as WriteXdr>::write_xdr`.
    pub const fn write_type_freeze_bypass_txs_delta(&mut self, v: &FreezeBypassTxsDeltaView<'_>) {
        self.write_type_vec_hash(&v.add_txs);
        self.write_type_vec_hash(&v.remove_txs);
    }
}
