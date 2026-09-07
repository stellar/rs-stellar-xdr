#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResultPair is an XDR Struct defined as:
///
/// ```text
/// struct TransactionResultPair
/// {
///     Hash transactionHash;
///     TransactionResult result; // result for the transaction
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
pub struct TransactionResultPair {
    pub transaction_hash: Hash,
    pub result: TransactionResult,
}

impl ReadXdr for TransactionResultPair {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                transaction_hash: Hash::read_xdr(r)?,
                result: TransactionResult::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionResultPair {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.transaction_hash.write_xdr(w)?;
            self.result.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionResultPairView is a borrowing equivalent of [`TransactionResultPair`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultPairView<'a> {
    pub transaction_hash: Hash,
    pub result: TransactionResultView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionResultPairView<'_> {
    type Owned = TransactionResultPair;
    fn into_owned(self) -> TransactionResultPair {
        TransactionResultPair {
            transaction_hash: self.transaction_hash.into_owned(),
            result: self.result.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionResultPairView<'_>> for TransactionResultPair {
    #[must_use]
    fn from(v: &TransactionResultPairView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionResultPairView<'_>> for TransactionResultPair {
    #[must_use]
    fn from(v: TransactionResultPairView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionResultPairView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.transaction_hash.write_xdr(w)?;
            self.result.write_xdr(w)?;
            Ok(())
        })
    }
}
