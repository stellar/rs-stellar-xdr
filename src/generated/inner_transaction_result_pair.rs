#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// InnerTransactionResultPair is an XDR Struct defined as:
///
/// ```text
/// struct InnerTransactionResultPair
/// {
///     Hash transactionHash;          // hash of the inner transaction
///     InnerTransactionResult result; // result for the inner transaction
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
pub struct InnerTransactionResultPair {
    pub transaction_hash: Hash,
    pub result: InnerTransactionResult,
}

impl ReadXdr for InnerTransactionResultPair {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                transaction_hash: Hash::read_xdr(r)?,
                result: InnerTransactionResult::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for InnerTransactionResultPair {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.transaction_hash.write_xdr(w)?;
            self.result.write_xdr(w)?;
            Ok(())
        })
    }
}

/// InnerTransactionResultPairView is a borrowing equivalent of [`InnerTransactionResultPair`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InnerTransactionResultPairView<'a> {
    pub transaction_hash: Hash,
    pub result: InnerTransactionResultView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for InnerTransactionResultPairView<'_> {
    type Owned = InnerTransactionResultPair;
    fn into_owned(&self) -> InnerTransactionResultPair {
        InnerTransactionResultPair {
            transaction_hash: IntoOwned::into_owned(&self.transaction_hash),
            result: IntoOwned::into_owned(&self.result),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&InnerTransactionResultPairView<'_>> for InnerTransactionResultPair {
    #[must_use]
    fn from(v: &InnerTransactionResultPairView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<InnerTransactionResultPairView<'_>> for InnerTransactionResultPair {
    #[must_use]
    fn from(v: InnerTransactionResultPairView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for InnerTransactionResultPairView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.transaction_hash.write_xdr(w)?;
            self.result.write_xdr(w)?;
            Ok(())
        })
    }
}
