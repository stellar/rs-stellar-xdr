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

/// InnerTransactionResultPairRef is a borrowing equivalent of [`InnerTransactionResultPair`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InnerTransactionResultPairRef<'a> {
    pub transaction_hash: Hash,
    pub result: InnerTransactionResultRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&InnerTransactionResultPairRef<'_>> for InnerTransactionResultPair {
    #[must_use]
    fn from(v: &InnerTransactionResultPairRef<'_>) -> Self {
        Self {
            transaction_hash: v.transaction_hash.clone(),
            result: (&v.result).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<InnerTransactionResultPairRef<'_>> for InnerTransactionResultPair {
    #[must_use]
    fn from(v: InnerTransactionResultPairRef<'_>) -> Self {
        Self::from(&v)
    }
}
