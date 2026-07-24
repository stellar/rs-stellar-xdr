#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HashIdPreimageOperationId is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID sourceAccount;
///         SequenceNumber seqNum;
///         uint32 opNum;
///     }
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
pub struct HashIdPreimageOperationId {
    pub source_account: AccountId,
    pub seq_num: SequenceNumber,
    pub op_num: u32,
}

impl ReadXdr for HashIdPreimageOperationId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account: AccountId::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                op_num: u32::read_xdr(r)?,
            })
        })
    }
}

impl HashIdPreimageOperationId {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.source_account.const_write_xdr(w);
        self.seq_num.const_write_xdr(w);
        w.write_u32(self.op_num);
        w.leave_depth();
    }
}

impl WriteXdr for HashIdPreimageOperationId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.op_num.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
