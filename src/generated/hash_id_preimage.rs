#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HashIdPreimage is an XDR Union defined as:
///
/// ```text
/// union HashIDPreimage switch (EnvelopeType type)
/// {
/// case ENVELOPE_TYPE_OP_ID:
///     struct
///     {
///         AccountID sourceAccount;
///         SequenceNumber seqNum;
///         uint32 opNum;
///     } operationID;
/// case ENVELOPE_TYPE_POOL_REVOKE_OP_ID:
///     struct
///     {
///         AccountID sourceAccount;
///         SequenceNumber seqNum;
///         uint32 opNum;
///         PoolID liquidityPoolID;
///         Asset asset;
///     } revokeID;
/// case ENVELOPE_TYPE_CONTRACT_ID:
///     struct
///     {
///         Hash networkID;
///         ContractIDPreimage contractIDPreimage;
///     } contractID;
/// case ENVELOPE_TYPE_SOROBAN_AUTHORIZATION:
///     struct
///     {
///         Hash networkID;
///         int64 nonce;
///         uint32 signatureExpirationLedger;
///         SorobanAuthorizedInvocation invocation;
///     } sorobanAuthorization;
/// };
/// ```
///
// union with discriminant EnvelopeType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum HashIdPreimage {
    OpId(HashIdPreimageOperationId),
    PoolRevokeOpId(HashIdPreimageRevokeId),
    ContractId(HashIdPreimageContractId),
    SorobanAuthorization(HashIdPreimageSorobanAuthorization),
}

#[cfg(feature = "alloc")]
impl Default for HashIdPreimage {
    fn default() -> Self {
        Self::OpId(HashIdPreimageOperationId::default())
    }
}

impl HashIdPreimage {
    const _VARIANTS: &[EnvelopeType] = &[
        EnvelopeType::OpId,
        EnvelopeType::PoolRevokeOpId,
        EnvelopeType::ContractId,
        EnvelopeType::SorobanAuthorization,
    ];
    pub const VARIANTS: [EnvelopeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "OpId",
        "PoolRevokeOpId",
        "ContractId",
        "SorobanAuthorization",
    ];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::OpId(_) => "OpId",
            Self::PoolRevokeOpId(_) => "PoolRevokeOpId",
            Self::ContractId(_) => "ContractId",
            Self::SorobanAuthorization(_) => "SorobanAuthorization",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpId(_) => EnvelopeType::OpId,
            Self::PoolRevokeOpId(_) => EnvelopeType::PoolRevokeOpId,
            Self::ContractId(_) => EnvelopeType::ContractId,
            Self::SorobanAuthorization(_) => EnvelopeType::SorobanAuthorization,
        }
    }

    #[must_use]
    pub const fn variants() -> [EnvelopeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HashIdPreimage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<EnvelopeType> for HashIdPreimage {
    #[must_use]
    fn discriminant(&self) -> EnvelopeType {
        Self::discriminant(self)
    }
}

impl Variants<EnvelopeType> for HashIdPreimage {
    fn variants() -> slice::Iter<'static, EnvelopeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<EnvelopeType> for HashIdPreimage {}

impl ReadXdr for HashIdPreimage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                EnvelopeType::OpId => Self::OpId(HashIdPreimageOperationId::read_xdr(r)?),
                EnvelopeType::PoolRevokeOpId => {
                    Self::PoolRevokeOpId(HashIdPreimageRevokeId::read_xdr(r)?)
                }
                EnvelopeType::ContractId => {
                    Self::ContractId(HashIdPreimageContractId::read_xdr(r)?)
                }
                EnvelopeType::SorobanAuthorization => {
                    Self::SorobanAuthorization(HashIdPreimageSorobanAuthorization::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for HashIdPreimage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::OpId(v) => v.write_xdr(w)?,
                Self::PoolRevokeOpId(v) => v.write_xdr(w)?,
                Self::ContractId(v) => v.write_xdr(w)?,
                Self::SorobanAuthorization(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
