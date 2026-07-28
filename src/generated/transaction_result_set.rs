#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResultSet is an XDR Struct defined as:
///
/// ```text
/// struct TransactionResultSet
/// {
///     TransactionResultPair results<>;
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
pub struct TransactionResultSet {
    pub results: VecM<TransactionResultPair>,
}

impl ReadXdr for TransactionResultSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                results: VecM::<TransactionResultPair>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionResultSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.results.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionResultSetRef is a borrowing equivalent of [`TransactionResultSet`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultSetRef<'a> {
    pub results: VecMRef<'a, TransactionResultPairRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionResultSetRef<'_>> for TransactionResultSet {
    #[must_use]
    fn from(v: &TransactionResultSetRef<'_>) -> Self {
        Self {
            results: v.results.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionResultSetRef<'_>> for TransactionResultSet {
    #[must_use]
    fn from(v: TransactionResultSetRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionResultSetRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.results.write_xdr(w)?;
            Ok(())
        })
    }
}
