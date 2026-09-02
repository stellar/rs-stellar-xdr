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
///     case STELLAR_VALUE_EMPTY_TX_SET:
///         struct
///         {
///             Hash txSetHash;
///             Hash previousLedgerHash;
///             uint32 previousLedgerVersion;
///             LedgerCloseValueSignature lcValueSignature;
///         } proposedValue;
///     }
///     ext;
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

/// StellarValueView is a borrowing equivalent of [`StellarValue`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StellarValueView<'a> {
    pub tx_set_hash: Hash,
    pub close_time: TimePoint,
    pub upgrades: VecMView<'a, UpgradeTypeView<'a>, 6>,
    pub ext: StellarValueExtView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&StellarValueView<'_>> for StellarValue {
    #[must_use]
    fn from(v: &StellarValueView<'_>) -> Self {
        Self {
            tx_set_hash: v.tx_set_hash.clone(),
            close_time: v.close_time.clone(),
            upgrades: v.upgrades.to_vecm(),
            ext: (&v.ext).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<StellarValueView<'_>> for StellarValue {
    #[must_use]
    fn from(v: StellarValueView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for StellarValueView<'_> {
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

#[cfg(feature = "const")]
impl StellarValueView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_stellar_value(self);
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
        w.write_type_stellar_value(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`StellarValue`], mirroring `<StellarValue as WriteXdr>::write_xdr`.
    pub const fn write_type_stellar_value(&mut self, v: &StellarValueView<'_>) {
        self.write_type_hash(&v.tx_set_hash);
        self.write_type_time_point(&v.close_time);
        self.write_type_vec_upgrade_type(&v.upgrades);
        self.write_type_stellar_value_ext(&v.ext);
    }
}
