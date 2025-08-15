// Module  is generated from:
//  xdr/next/Stellar-SCP.x
//  xdr/next/Stellar-contract-config-setting.x
//  xdr/next/Stellar-contract-env-meta.x
//  xdr/next/Stellar-contract-meta.x
//  xdr/next/Stellar-contract-spec.x
//  xdr/next/Stellar-contract.x
//  xdr/next/Stellar-exporter.x
//  xdr/next/Stellar-internal.x
//  xdr/next/Stellar-ledger-entries.x
//  xdr/next/Stellar-ledger.x
//  xdr/next/Stellar-overlay.x
//  xdr/next/Stellar-transaction.x
//  xdr/next/Stellar-types.x

#![allow(clippy::missing_errors_doc, clippy::unreadable_literal)]

/// `XDR_FILES_SHA256` is a list of pairs of source files and their SHA256 hashes.
pub const XDR_FILES_SHA256: [(&str, &str); 13] = [
    (
        "xdr/next/Stellar-SCP.x",
        "8f32b04d008f8bc33b8843d075e69837231a673691ee41d8b821ca229a6e802a",
    ),
    (
        "xdr/next/Stellar-contract-config-setting.x",
        "5d1d926e4288b0f2d1ce9f891ca2cab97de9246381d57fca22e25a0d276c6682",
    ),
    (
        "xdr/next/Stellar-contract-env-meta.x",
        "75a271414d852096fea3283c63b7f2a702f2905f78fc28eb60ec7d7bd366a780",
    ),
    (
        "xdr/next/Stellar-contract-meta.x",
        "f01532c11ca044e19d9f9f16fe373e9af64835da473be556b9a807ee3319ae0d",
    ),
    (
        "xdr/next/Stellar-contract-spec.x",
        "7bd048e1b008c274f667a4f9b8fcf5ae848e301aca0073cdc8b266ecd2c5f2f9",
    ),
    (
        "xdr/next/Stellar-contract.x",
        "dce61df115c93fef5bb352beac1b504a518cb11dcb8ee029b1bb1b5f8fe52982",
    ),
    (
        "xdr/next/Stellar-exporter.x",
        "a00c83d02e8c8382e06f79a191f1fb5abd097a4bbcab8481c67467e3270e0529",
    ),
    (
        "xdr/next/Stellar-internal.x",
        "227835866c1b2122d1eaf28839ba85ea7289d1cb681dda4ca619c2da3d71fe00",
    ),
    (
        "xdr/next/Stellar-ledger-entries.x",
        "5157cad76b008b3606fe5bc2cfe87596827d8e02d16cbec3cedc297bb571aa54",
    ),
    (
        "xdr/next/Stellar-ledger.x",
        "cf936606885dd265082e553aa433c2cf47b720b6d58839b154cf71096b885d1e",
    ),
    (
        "xdr/next/Stellar-overlay.x",
        "8c9b9c13c86fa4672f03d741705b41e7221be0fc48e1ea6eeb1ba07d31ec0723",
    ),
    (
        "xdr/next/Stellar-transaction.x",
        "7c4c951f233ad7cdabedd740abd9697626ec5bc03ce97bf60cbaeee1481a48d1",
    ),
    (
        "xdr/next/Stellar-types.x",
        "d37a4b8683d2ddb9f13f6d8a4e5111dfe7de4176516db52bc0517ec46a82c3d4",
    ),
];

use core::{array::TryFromSliceError, fmt, fmt::Debug, marker::Sized, ops::Deref, slice};

#[cfg(feature = "std")]
use core::marker::PhantomData;

// When feature alloc is turned off use static lifetime Box and Vec types.
#[cfg(not(feature = "alloc"))]
mod noalloc {
    pub mod boxed {
        pub type Box<T> = &'static T;
    }
    pub mod vec {
        pub type Vec<T> = &'static [T];
    }
}
#[cfg(not(feature = "alloc"))]
use noalloc::{boxed::Box, vec::Vec};

// When feature std is turned off, but feature alloc is turned on import the
// alloc crate and use its Box and Vec types.
#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{FromUtf8Error, String},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::string::FromUtf8Error;

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;

#[cfg(all(feature = "schemars", feature = "alloc", not(feature = "std")))]
use alloc::borrow::Cow;
#[cfg(all(feature = "schemars", feature = "alloc", feature = "std"))]
use std::borrow::Cow;

// TODO: Add support for read/write xdr fns when std not available.

#[cfg(feature = "std")]
use std::{
    error, io,
    io::{BufRead, BufReader, Cursor, Read, Write},
};

/// Error contains all errors returned by functions in this crate. It can be
/// compared via `PartialEq`, however any contained IO errors will only be
/// compared on their `ErrorKind`.
// Re-exported types from curr that are identical
pub use crate::curr::generated::AccountEntry;
pub use crate::curr::generated::AccountEntryExt;
pub use crate::curr::generated::AccountEntryExtensionV1;
pub use crate::curr::generated::AccountEntryExtensionV1Ext;
pub use crate::curr::generated::AccountEntryExtensionV2;
pub use crate::curr::generated::AccountEntryExtensionV2Ext;
pub use crate::curr::generated::AccountEntryExtensionV3;
pub use crate::curr::generated::AccountFlags;
pub use crate::curr::generated::AccountId;
pub use crate::curr::generated::AccountMergeResult;
pub use crate::curr::generated::AccountMergeResultCode;
pub use crate::curr::generated::AllowTrustOp;
pub use crate::curr::generated::AllowTrustResult;
pub use crate::curr::generated::AllowTrustResultCode;
pub use crate::curr::generated::AlphaNum12;
pub use crate::curr::generated::AlphaNum4;
pub use crate::curr::generated::Asset;
pub use crate::curr::generated::AssetCode;
pub use crate::curr::generated::AssetCode12;
pub use crate::curr::generated::AssetCode4;
pub use crate::curr::generated::AssetType;
pub use crate::curr::generated::Auth;
pub use crate::curr::generated::AuthCert;
pub use crate::curr::generated::AuthenticatedMessage;
pub use crate::curr::generated::AuthenticatedMessageV0;
pub use crate::curr::generated::BeginSponsoringFutureReservesOp;
pub use crate::curr::generated::BeginSponsoringFutureReservesResult;
pub use crate::curr::generated::BeginSponsoringFutureReservesResultCode;
pub use crate::curr::generated::BinaryFuseFilterType;
pub use crate::curr::generated::BucketEntry;
pub use crate::curr::generated::BucketEntryType;
pub use crate::curr::generated::BucketListType;
pub use crate::curr::generated::BucketMetadata;
pub use crate::curr::generated::BucketMetadataExt;
pub use crate::curr::generated::BumpSequenceOp;
pub use crate::curr::generated::BumpSequenceResult;
pub use crate::curr::generated::BumpSequenceResultCode;
pub use crate::curr::generated::BytesM;
pub use crate::curr::generated::ChangeTrustAsset;
pub use crate::curr::generated::ChangeTrustOp;
pub use crate::curr::generated::ChangeTrustResult;
pub use crate::curr::generated::ChangeTrustResultCode;
pub use crate::curr::generated::ClaimAtom;
pub use crate::curr::generated::ClaimAtomType;
pub use crate::curr::generated::ClaimClaimableBalanceOp;
pub use crate::curr::generated::ClaimClaimableBalanceResult;
pub use crate::curr::generated::ClaimClaimableBalanceResultCode;
pub use crate::curr::generated::ClaimLiquidityAtom;
pub use crate::curr::generated::ClaimOfferAtom;
pub use crate::curr::generated::ClaimOfferAtomV0;
pub use crate::curr::generated::ClaimPredicate;
pub use crate::curr::generated::ClaimPredicateType;
pub use crate::curr::generated::ClaimableBalanceEntry;
pub use crate::curr::generated::ClaimableBalanceEntryExt;
pub use crate::curr::generated::ClaimableBalanceEntryExtensionV1;
pub use crate::curr::generated::ClaimableBalanceEntryExtensionV1Ext;
pub use crate::curr::generated::ClaimableBalanceFlags;
pub use crate::curr::generated::ClaimableBalanceId;
pub use crate::curr::generated::ClaimableBalanceIdType;
pub use crate::curr::generated::Claimant;
pub use crate::curr::generated::ClaimantType;
pub use crate::curr::generated::ClaimantV0;
pub use crate::curr::generated::ClawbackClaimableBalanceOp;
pub use crate::curr::generated::ClawbackClaimableBalanceResult;
pub use crate::curr::generated::ClawbackClaimableBalanceResultCode;
pub use crate::curr::generated::ClawbackOp;
pub use crate::curr::generated::ClawbackResult;
pub use crate::curr::generated::ClawbackResultCode;
pub use crate::curr::generated::ConfigSettingContractBandwidthV0;
pub use crate::curr::generated::ConfigSettingContractComputeV0;
pub use crate::curr::generated::ConfigSettingContractEventsV0;
pub use crate::curr::generated::ConfigSettingContractExecutionLanesV0;
pub use crate::curr::generated::ConfigSettingContractHistoricalDataV0;
pub use crate::curr::generated::ConfigSettingContractLedgerCostExtV0;
pub use crate::curr::generated::ConfigSettingContractLedgerCostV0;
pub use crate::curr::generated::ConfigSettingContractParallelComputeV0;
pub use crate::curr::generated::ConfigSettingEntry;
pub use crate::curr::generated::ConfigSettingId;
pub use crate::curr::generated::ConfigSettingScpTiming;
pub use crate::curr::generated::ConfigUpgradeSet;
pub use crate::curr::generated::ConfigUpgradeSetKey;
pub use crate::curr::generated::ContractCodeCostInputs;
pub use crate::curr::generated::ContractCodeEntry;
pub use crate::curr::generated::ContractCodeEntryExt;
pub use crate::curr::generated::ContractCodeEntryV1;
pub use crate::curr::generated::ContractCostParamEntry;
pub use crate::curr::generated::ContractCostParams;
pub use crate::curr::generated::ContractCostType;
pub use crate::curr::generated::ContractDataDurability;
pub use crate::curr::generated::ContractDataEntry;
pub use crate::curr::generated::ContractEvent;
pub use crate::curr::generated::ContractEventBody;
pub use crate::curr::generated::ContractEventType;
pub use crate::curr::generated::ContractEventV0;
pub use crate::curr::generated::ContractExecutable;
pub use crate::curr::generated::ContractExecutableType;
pub use crate::curr::generated::ContractId;
pub use crate::curr::generated::ContractIdPreimage;
pub use crate::curr::generated::ContractIdPreimageFromAddress;
pub use crate::curr::generated::ContractIdPreimageType;
pub use crate::curr::generated::CreateAccountOp;
pub use crate::curr::generated::CreateAccountResult;
pub use crate::curr::generated::CreateAccountResultCode;
pub use crate::curr::generated::CreateClaimableBalanceOp;
pub use crate::curr::generated::CreateClaimableBalanceResult;
pub use crate::curr::generated::CreateClaimableBalanceResultCode;
pub use crate::curr::generated::CreateContractArgs;
pub use crate::curr::generated::CreateContractArgsV2;
pub use crate::curr::generated::CreatePassiveSellOfferOp;
pub use crate::curr::generated::CryptoKeyType;
pub use crate::curr::generated::Curve25519Public;
pub use crate::curr::generated::Curve25519Secret;
pub use crate::curr::generated::DataEntry;
pub use crate::curr::generated::DataEntryExt;
pub use crate::curr::generated::DataValue;
pub use crate::curr::generated::DecoratedSignature;
pub use crate::curr::generated::DependentTxCluster;
pub use crate::curr::generated::DiagnosticEvent;
pub use crate::curr::generated::DontHave;
pub use crate::curr::generated::Duration;
pub use crate::curr::generated::EncryptedBody;
pub use crate::curr::generated::EndSponsoringFutureReservesResult;
pub use crate::curr::generated::EndSponsoringFutureReservesResultCode;
pub use crate::curr::generated::EnvelopeType;
pub use crate::curr::generated::Error;
pub use crate::curr::generated::ErrorCode;
pub use crate::curr::generated::EvictionIterator;
pub use crate::curr::generated::ExtendFootprintTtlOp;
pub use crate::curr::generated::ExtendFootprintTtlResult;
pub use crate::curr::generated::ExtendFootprintTtlResultCode;
pub use crate::curr::generated::ExtensionPoint;
pub use crate::curr::generated::FeeBumpTransaction;
pub use crate::curr::generated::FeeBumpTransactionEnvelope;
pub use crate::curr::generated::FeeBumpTransactionExt;
pub use crate::curr::generated::FeeBumpTransactionInnerTx;
pub use crate::curr::generated::FloodAdvert;
pub use crate::curr::generated::FloodDemand;
pub use crate::curr::generated::Frame;
pub use crate::curr::generated::GeneralizedTransactionSet;
pub use crate::curr::generated::Hash;
pub use crate::curr::generated::HashIdPreimage;
pub use crate::curr::generated::HashIdPreimageContractId;
pub use crate::curr::generated::HashIdPreimageOperationId;
pub use crate::curr::generated::HashIdPreimageRevokeId;
pub use crate::curr::generated::HashIdPreimageSorobanAuthorization;
pub use crate::curr::generated::Hello;
pub use crate::curr::generated::HmacSha256Key;
pub use crate::curr::generated::HmacSha256Mac;
pub use crate::curr::generated::HostFunction;
pub use crate::curr::generated::HostFunctionType;
pub use crate::curr::generated::HotArchiveBucketEntry;
pub use crate::curr::generated::HotArchiveBucketEntryType;
pub use crate::curr::generated::InflationPayout;
pub use crate::curr::generated::InflationResult;
pub use crate::curr::generated::InflationResultCode;
pub use crate::curr::generated::InnerTransactionResult;
pub use crate::curr::generated::InnerTransactionResultExt;
pub use crate::curr::generated::InnerTransactionResultPair;
pub use crate::curr::generated::InnerTransactionResultResult;
pub use crate::curr::generated::Int128Parts;
pub use crate::curr::generated::Int256Parts;
pub use crate::curr::generated::Int32;
pub use crate::curr::generated::Int64;
pub use crate::curr::generated::InvokeContractArgs;
pub use crate::curr::generated::InvokeHostFunctionOp;
pub use crate::curr::generated::InvokeHostFunctionResult;
pub use crate::curr::generated::InvokeHostFunctionResultCode;
pub use crate::curr::generated::InvokeHostFunctionSuccessPreImage;
pub use crate::curr::generated::IpAddrType;
pub use crate::curr::generated::LedgerBounds;
pub use crate::curr::generated::LedgerCloseMeta;
pub use crate::curr::generated::LedgerCloseMetaBatch;
pub use crate::curr::generated::LedgerCloseMetaExt;
pub use crate::curr::generated::LedgerCloseMetaExtV1;
pub use crate::curr::generated::LedgerCloseMetaV0;
pub use crate::curr::generated::LedgerCloseMetaV1;
pub use crate::curr::generated::LedgerCloseMetaV2;
pub use crate::curr::generated::LedgerCloseValueSignature;
pub use crate::curr::generated::LedgerEntry;
pub use crate::curr::generated::LedgerEntryChange;
pub use crate::curr::generated::LedgerEntryChangeType;
pub use crate::curr::generated::LedgerEntryChanges;
pub use crate::curr::generated::LedgerEntryData;
pub use crate::curr::generated::LedgerEntryExt;
pub use crate::curr::generated::LedgerEntryExtensionV1;
pub use crate::curr::generated::LedgerEntryExtensionV1Ext;
pub use crate::curr::generated::LedgerEntryType;
pub use crate::curr::generated::LedgerFootprint;
pub use crate::curr::generated::LedgerHeader;
pub use crate::curr::generated::LedgerHeaderExt;
pub use crate::curr::generated::LedgerHeaderExtensionV1;
pub use crate::curr::generated::LedgerHeaderExtensionV1Ext;
pub use crate::curr::generated::LedgerHeaderFlags;
pub use crate::curr::generated::LedgerHeaderHistoryEntry;
pub use crate::curr::generated::LedgerHeaderHistoryEntryExt;
pub use crate::curr::generated::LedgerKey;
pub use crate::curr::generated::LedgerKeyAccount;
pub use crate::curr::generated::LedgerKeyClaimableBalance;
pub use crate::curr::generated::LedgerKeyConfigSetting;
pub use crate::curr::generated::LedgerKeyContractCode;
pub use crate::curr::generated::LedgerKeyContractData;
pub use crate::curr::generated::LedgerKeyData;
pub use crate::curr::generated::LedgerKeyLiquidityPool;
pub use crate::curr::generated::LedgerKeyOffer;
pub use crate::curr::generated::LedgerKeyTrustLine;
pub use crate::curr::generated::LedgerKeyTtl;
pub use crate::curr::generated::LedgerScpMessages;
pub use crate::curr::generated::LedgerUpgrade;
pub use crate::curr::generated::LedgerUpgradeType;
pub use crate::curr::generated::Liabilities;
pub use crate::curr::generated::Limited;
pub use crate::curr::generated::Limits;
pub use crate::curr::generated::LiquidityPoolConstantProductParameters;
pub use crate::curr::generated::LiquidityPoolDepositOp;
pub use crate::curr::generated::LiquidityPoolDepositResult;
pub use crate::curr::generated::LiquidityPoolDepositResultCode;
pub use crate::curr::generated::LiquidityPoolEntry;
pub use crate::curr::generated::LiquidityPoolEntryBody;
pub use crate::curr::generated::LiquidityPoolEntryConstantProduct;
pub use crate::curr::generated::LiquidityPoolParameters;
pub use crate::curr::generated::LiquidityPoolType;
pub use crate::curr::generated::LiquidityPoolWithdrawOp;
pub use crate::curr::generated::LiquidityPoolWithdrawResult;
pub use crate::curr::generated::LiquidityPoolWithdrawResultCode;
pub use crate::curr::generated::ManageBuyOfferOp;
pub use crate::curr::generated::ManageBuyOfferResult;
pub use crate::curr::generated::ManageBuyOfferResultCode;
pub use crate::curr::generated::ManageDataOp;
pub use crate::curr::generated::ManageDataResult;
pub use crate::curr::generated::ManageDataResultCode;
pub use crate::curr::generated::ManageOfferEffect;
pub use crate::curr::generated::ManageOfferSuccessResult;
pub use crate::curr::generated::ManageOfferSuccessResultOffer;
pub use crate::curr::generated::ManageSellOfferOp;
pub use crate::curr::generated::ManageSellOfferResult;
pub use crate::curr::generated::ManageSellOfferResultCode;
pub use crate::curr::generated::Memo;
pub use crate::curr::generated::MemoType;
pub use crate::curr::generated::MessageType;
pub use crate::curr::generated::MuxedAccount;
pub use crate::curr::generated::MuxedAccountMed25519;
pub use crate::curr::generated::MuxedEd25519Account;
pub use crate::curr::generated::NodeId;
pub use crate::curr::generated::NumberOrString;
pub use crate::curr::generated::OfferEntry;
pub use crate::curr::generated::OfferEntryExt;
pub use crate::curr::generated::OfferEntryFlags;
pub use crate::curr::generated::Operation;
pub use crate::curr::generated::OperationBody;
pub use crate::curr::generated::OperationMeta;
pub use crate::curr::generated::OperationMetaV2;
pub use crate::curr::generated::OperationResult;
pub use crate::curr::generated::OperationResultCode;
pub use crate::curr::generated::OperationResultTr;
pub use crate::curr::generated::OperationType;
pub use crate::curr::generated::ParallelTxExecutionStage;
pub use crate::curr::generated::ParallelTxsComponent;
pub use crate::curr::generated::PathPaymentStrictReceiveOp;
pub use crate::curr::generated::PathPaymentStrictReceiveResult;
pub use crate::curr::generated::PathPaymentStrictReceiveResultCode;
pub use crate::curr::generated::PathPaymentStrictReceiveResultSuccess;
pub use crate::curr::generated::PathPaymentStrictSendOp;
pub use crate::curr::generated::PathPaymentStrictSendResult;
pub use crate::curr::generated::PathPaymentStrictSendResultCode;
pub use crate::curr::generated::PathPaymentStrictSendResultSuccess;
pub use crate::curr::generated::PaymentOp;
pub use crate::curr::generated::PaymentResult;
pub use crate::curr::generated::PaymentResultCode;
pub use crate::curr::generated::PeerAddress;
pub use crate::curr::generated::PeerAddressIp;
pub use crate::curr::generated::PeerStats;
pub use crate::curr::generated::PersistedScpState;
pub use crate::curr::generated::PersistedScpStateV0;
pub use crate::curr::generated::PersistedScpStateV1;
pub use crate::curr::generated::PoolId;
pub use crate::curr::generated::PreconditionType;
pub use crate::curr::generated::Preconditions;
pub use crate::curr::generated::PreconditionsV2;
pub use crate::curr::generated::Price;
pub use crate::curr::generated::PublicKey;
pub use crate::curr::generated::PublicKeyType;
pub use crate::curr::generated::ReadXdrIter;
pub use crate::curr::generated::RestoreFootprintOp;
pub use crate::curr::generated::RestoreFootprintResult;
pub use crate::curr::generated::RestoreFootprintResultCode;
pub use crate::curr::generated::RevokeSponsorshipOp;
pub use crate::curr::generated::RevokeSponsorshipOpSigner;
pub use crate::curr::generated::RevokeSponsorshipResult;
pub use crate::curr::generated::RevokeSponsorshipResultCode;
pub use crate::curr::generated::RevokeSponsorshipType;
pub use crate::curr::generated::SError;
pub use crate::curr::generated::ScAddress;
pub use crate::curr::generated::ScAddressType;
pub use crate::curr::generated::ScBytes;
pub use crate::curr::generated::ScContractInstance;
pub use crate::curr::generated::ScEnvMetaEntry;
pub use crate::curr::generated::ScEnvMetaEntryInterfaceVersion;
pub use crate::curr::generated::ScEnvMetaKind;
pub use crate::curr::generated::ScError;
pub use crate::curr::generated::ScErrorCode;
pub use crate::curr::generated::ScErrorType;
pub use crate::curr::generated::ScMap;
pub use crate::curr::generated::ScMapEntry;
pub use crate::curr::generated::ScMetaEntry;
pub use crate::curr::generated::ScMetaKind;
pub use crate::curr::generated::ScMetaV0;
pub use crate::curr::generated::ScNonceKey;
pub use crate::curr::generated::ScSpecEntry;
pub use crate::curr::generated::ScSpecEntryKind;
pub use crate::curr::generated::ScSpecEventDataFormat;
pub use crate::curr::generated::ScSpecEventParamLocationV0;
pub use crate::curr::generated::ScSpecEventParamV0;
pub use crate::curr::generated::ScSpecEventV0;
pub use crate::curr::generated::ScSpecFunctionInputV0;
pub use crate::curr::generated::ScSpecFunctionV0;
pub use crate::curr::generated::ScSpecType;
pub use crate::curr::generated::ScSpecTypeBytesN;
pub use crate::curr::generated::ScSpecTypeDef;
pub use crate::curr::generated::ScSpecTypeMap;
pub use crate::curr::generated::ScSpecTypeOption;
pub use crate::curr::generated::ScSpecTypeResult;
pub use crate::curr::generated::ScSpecTypeTuple;
pub use crate::curr::generated::ScSpecTypeUdt;
pub use crate::curr::generated::ScSpecTypeVec;
pub use crate::curr::generated::ScSpecUdtEnumCaseV0;
pub use crate::curr::generated::ScSpecUdtEnumV0;
pub use crate::curr::generated::ScSpecUdtErrorEnumCaseV0;
pub use crate::curr::generated::ScSpecUdtErrorEnumV0;
pub use crate::curr::generated::ScSpecUdtStructFieldV0;
pub use crate::curr::generated::ScSpecUdtStructV0;
pub use crate::curr::generated::ScSpecUdtUnionCaseTupleV0;
pub use crate::curr::generated::ScSpecUdtUnionCaseV0;
pub use crate::curr::generated::ScSpecUdtUnionCaseV0Kind;
pub use crate::curr::generated::ScSpecUdtUnionCaseVoidV0;
pub use crate::curr::generated::ScSpecUdtUnionV0;
pub use crate::curr::generated::ScString;
pub use crate::curr::generated::ScSymbol;
pub use crate::curr::generated::ScVal;
pub use crate::curr::generated::ScValType;
pub use crate::curr::generated::ScVec;
pub use crate::curr::generated::ScpBallot;
pub use crate::curr::generated::ScpEnvelope;
pub use crate::curr::generated::ScpHistoryEntry;
pub use crate::curr::generated::ScpHistoryEntryV0;
pub use crate::curr::generated::ScpNomination;
pub use crate::curr::generated::ScpQuorumSet;
pub use crate::curr::generated::ScpStatement;
pub use crate::curr::generated::ScpStatementConfirm;
pub use crate::curr::generated::ScpStatementExternalize;
pub use crate::curr::generated::ScpStatementPledges;
pub use crate::curr::generated::ScpStatementPrepare;
pub use crate::curr::generated::ScpStatementType;
pub use crate::curr::generated::SendMore;
pub use crate::curr::generated::SendMoreExtended;
pub use crate::curr::generated::SequenceNumber;
pub use crate::curr::generated::SerializedBinaryFuseFilter;
pub use crate::curr::generated::SetOptionsOp;
pub use crate::curr::generated::SetOptionsResult;
pub use crate::curr::generated::SetOptionsResultCode;
pub use crate::curr::generated::SetTrustLineFlagsOp;
pub use crate::curr::generated::SetTrustLineFlagsResult;
pub use crate::curr::generated::SetTrustLineFlagsResultCode;
pub use crate::curr::generated::ShortHashSeed;
pub use crate::curr::generated::Signature;
pub use crate::curr::generated::SignatureHint;
pub use crate::curr::generated::SignedTimeSlicedSurveyRequestMessage;
pub use crate::curr::generated::SignedTimeSlicedSurveyResponseMessage;
pub use crate::curr::generated::SignedTimeSlicedSurveyStartCollectingMessage;
pub use crate::curr::generated::SignedTimeSlicedSurveyStopCollectingMessage;
pub use crate::curr::generated::Signer;
pub use crate::curr::generated::SignerKey;
pub use crate::curr::generated::SignerKeyEd25519SignedPayload;
pub use crate::curr::generated::SignerKeyType;
pub use crate::curr::generated::SimplePaymentResult;
pub use crate::curr::generated::SkipWhitespace;
pub use crate::curr::generated::SorobanAddressCredentials;
pub use crate::curr::generated::SorobanAuthorizationEntries;
pub use crate::curr::generated::SorobanAuthorizationEntry;
pub use crate::curr::generated::SorobanAuthorizedFunction;
pub use crate::curr::generated::SorobanAuthorizedFunctionType;
pub use crate::curr::generated::SorobanAuthorizedInvocation;
pub use crate::curr::generated::SorobanCredentials;
pub use crate::curr::generated::SorobanCredentialsType;
pub use crate::curr::generated::SorobanResources;
pub use crate::curr::generated::SorobanResourcesExtV0;
pub use crate::curr::generated::SorobanTransactionData;
pub use crate::curr::generated::SorobanTransactionDataExt;
pub use crate::curr::generated::SorobanTransactionMeta;
pub use crate::curr::generated::SorobanTransactionMetaExt;
pub use crate::curr::generated::SorobanTransactionMetaExtV1;
pub use crate::curr::generated::SorobanTransactionMetaV2;
pub use crate::curr::generated::SponsorshipDescriptor;
pub use crate::curr::generated::StateArchivalSettings;
pub use crate::curr::generated::StellarMessage;
pub use crate::curr::generated::StellarValue;
pub use crate::curr::generated::StellarValueExt;
pub use crate::curr::generated::StellarValueType;
pub use crate::curr::generated::StoredDebugTransactionSet;
pub use crate::curr::generated::StoredTransactionSet;
pub use crate::curr::generated::String32;
pub use crate::curr::generated::String64;
pub use crate::curr::generated::StringM;
pub use crate::curr::generated::SurveyMessageCommandType;
pub use crate::curr::generated::SurveyMessageResponseType;
pub use crate::curr::generated::SurveyRequestMessage;
pub use crate::curr::generated::SurveyResponseBody;
pub use crate::curr::generated::SurveyResponseMessage;
pub use crate::curr::generated::ThresholdIndexes;
pub use crate::curr::generated::Thresholds;
pub use crate::curr::generated::TimeBounds;
pub use crate::curr::generated::TimePoint;
pub use crate::curr::generated::TimeSlicedNodeData;
pub use crate::curr::generated::TimeSlicedPeerData;
pub use crate::curr::generated::TimeSlicedPeerDataList;
pub use crate::curr::generated::TimeSlicedSurveyRequestMessage;
pub use crate::curr::generated::TimeSlicedSurveyResponseMessage;
pub use crate::curr::generated::TimeSlicedSurveyStartCollectingMessage;
pub use crate::curr::generated::TimeSlicedSurveyStopCollectingMessage;
pub use crate::curr::generated::TopologyResponseBodyV2;
pub use crate::curr::generated::Transaction;
pub use crate::curr::generated::TransactionEnvelope;
pub use crate::curr::generated::TransactionEvent;
pub use crate::curr::generated::TransactionEventStage;
pub use crate::curr::generated::TransactionExt;
pub use crate::curr::generated::TransactionHistoryEntry;
pub use crate::curr::generated::TransactionHistoryEntryExt;
pub use crate::curr::generated::TransactionHistoryResultEntry;
pub use crate::curr::generated::TransactionHistoryResultEntryExt;
pub use crate::curr::generated::TransactionMeta;
pub use crate::curr::generated::TransactionMetaV1;
pub use crate::curr::generated::TransactionMetaV2;
pub use crate::curr::generated::TransactionMetaV3;
pub use crate::curr::generated::TransactionMetaV4;
pub use crate::curr::generated::TransactionPhase;
pub use crate::curr::generated::TransactionResult;
pub use crate::curr::generated::TransactionResultCode;
pub use crate::curr::generated::TransactionResultExt;
pub use crate::curr::generated::TransactionResultMeta;
pub use crate::curr::generated::TransactionResultMetaV1;
pub use crate::curr::generated::TransactionResultPair;
pub use crate::curr::generated::TransactionResultResult;
pub use crate::curr::generated::TransactionResultSet;
pub use crate::curr::generated::TransactionSet;
pub use crate::curr::generated::TransactionSetV1;
pub use crate::curr::generated::TransactionSignaturePayload;
pub use crate::curr::generated::TransactionSignaturePayloadTaggedTransaction;
pub use crate::curr::generated::TransactionV0;
pub use crate::curr::generated::TransactionV0Envelope;
pub use crate::curr::generated::TransactionV0Ext;
pub use crate::curr::generated::TransactionV1Envelope;
pub use crate::curr::generated::TrustLineAsset;
pub use crate::curr::generated::TrustLineEntry;
pub use crate::curr::generated::TrustLineEntryExt;
pub use crate::curr::generated::TrustLineEntryExtensionV2;
pub use crate::curr::generated::TrustLineEntryExtensionV2Ext;
pub use crate::curr::generated::TrustLineEntryV1;
pub use crate::curr::generated::TrustLineEntryV1Ext;
pub use crate::curr::generated::TrustLineFlags;
pub use crate::curr::generated::TtlEntry;
pub use crate::curr::generated::TxAdvertVector;
pub use crate::curr::generated::TxDemandVector;
pub use crate::curr::generated::TxSetComponent;
pub use crate::curr::generated::TxSetComponentTxsMaybeDiscountedFee;
pub use crate::curr::generated::TxSetComponentType;
pub use crate::curr::generated::TypeVariant;
pub use crate::curr::generated::UInt128Parts;
pub use crate::curr::generated::UInt256Parts;
pub use crate::curr::generated::Uint256;
pub use crate::curr::generated::Uint32;
pub use crate::curr::generated::Uint64;
pub use crate::curr::generated::UpgradeEntryMeta;
pub use crate::curr::generated::UpgradeType;
pub use crate::curr::generated::Value;
pub use crate::curr::generated::VecM;


pub enum Type {
    Value(Box<Value>),
    ScpBallot(Box<ScpBallot>),
    ScpStatementType(Box<ScpStatementType>),
    ScpNomination(Box<ScpNomination>),
    ScpStatement(Box<ScpStatement>),
    ScpStatementPledges(Box<ScpStatementPledges>),
    ScpStatementPrepare(Box<ScpStatementPrepare>),
    ScpStatementConfirm(Box<ScpStatementConfirm>),
    ScpStatementExternalize(Box<ScpStatementExternalize>),
    ScpEnvelope(Box<ScpEnvelope>),
    ScpQuorumSet(Box<ScpQuorumSet>),
    ConfigSettingContractExecutionLanesV0(Box<ConfigSettingContractExecutionLanesV0>),
    ConfigSettingContractComputeV0(Box<ConfigSettingContractComputeV0>),
    ConfigSettingContractParallelComputeV0(Box<ConfigSettingContractParallelComputeV0>),
    ConfigSettingContractLedgerCostV0(Box<ConfigSettingContractLedgerCostV0>),
    ConfigSettingContractLedgerCostExtV0(Box<ConfigSettingContractLedgerCostExtV0>),
    ConfigSettingContractHistoricalDataV0(Box<ConfigSettingContractHistoricalDataV0>),
    ConfigSettingContractEventsV0(Box<ConfigSettingContractEventsV0>),
    ConfigSettingContractBandwidthV0(Box<ConfigSettingContractBandwidthV0>),
    ContractCostType(Box<ContractCostType>),
    ContractCostParamEntry(Box<ContractCostParamEntry>),
    StateArchivalSettings(Box<StateArchivalSettings>),
    EvictionIterator(Box<EvictionIterator>),
    ConfigSettingScpTiming(Box<ConfigSettingScpTiming>),
    ContractCostParams(Box<ContractCostParams>),
    ConfigSettingId(Box<ConfigSettingId>),
    ConfigSettingEntry(Box<ConfigSettingEntry>),
    ScEnvMetaKind(Box<ScEnvMetaKind>),
    ScEnvMetaEntry(Box<ScEnvMetaEntry>),
    ScEnvMetaEntryInterfaceVersion(Box<ScEnvMetaEntryInterfaceVersion>),
    ScMetaV0(Box<ScMetaV0>),
    ScMetaKind(Box<ScMetaKind>),
    ScMetaEntry(Box<ScMetaEntry>),
    ScSpecType(Box<ScSpecType>),
    ScSpecTypeOption(Box<ScSpecTypeOption>),
    ScSpecTypeResult(Box<ScSpecTypeResult>),
    ScSpecTypeVec(Box<ScSpecTypeVec>),
    ScSpecTypeMap(Box<ScSpecTypeMap>),
    ScSpecTypeTuple(Box<ScSpecTypeTuple>),
    ScSpecTypeBytesN(Box<ScSpecTypeBytesN>),
    ScSpecTypeUdt(Box<ScSpecTypeUdt>),
    ScSpecTypeDef(Box<ScSpecTypeDef>),
    ScSpecUdtStructFieldV0(Box<ScSpecUdtStructFieldV0>),
    ScSpecUdtStructV0(Box<ScSpecUdtStructV0>),
    ScSpecUdtUnionCaseVoidV0(Box<ScSpecUdtUnionCaseVoidV0>),
    ScSpecUdtUnionCaseTupleV0(Box<ScSpecUdtUnionCaseTupleV0>),
    ScSpecUdtUnionCaseV0Kind(Box<ScSpecUdtUnionCaseV0Kind>),
    ScSpecUdtUnionCaseV0(Box<ScSpecUdtUnionCaseV0>),
    ScSpecUdtUnionV0(Box<ScSpecUdtUnionV0>),
    ScSpecUdtEnumCaseV0(Box<ScSpecUdtEnumCaseV0>),
    ScSpecUdtEnumV0(Box<ScSpecUdtEnumV0>),
    ScSpecUdtErrorEnumCaseV0(Box<ScSpecUdtErrorEnumCaseV0>),
    ScSpecUdtErrorEnumV0(Box<ScSpecUdtErrorEnumV0>),
    ScSpecFunctionInputV0(Box<ScSpecFunctionInputV0>),
    ScSpecFunctionV0(Box<ScSpecFunctionV0>),
    ScSpecEventParamLocationV0(Box<ScSpecEventParamLocationV0>),
    ScSpecEventParamV0(Box<ScSpecEventParamV0>),
    ScSpecEventDataFormat(Box<ScSpecEventDataFormat>),
    ScSpecEventV0(Box<ScSpecEventV0>),
    ScSpecEntryKind(Box<ScSpecEntryKind>),
    ScSpecEntry(Box<ScSpecEntry>),
    ScValType(Box<ScValType>),
    ScErrorType(Box<ScErrorType>),
    ScErrorCode(Box<ScErrorCode>),
    ScError(Box<ScError>),
    UInt128Parts(Box<UInt128Parts>),
    Int128Parts(Box<Int128Parts>),
    UInt256Parts(Box<UInt256Parts>),
    Int256Parts(Box<Int256Parts>),
    ContractExecutableType(Box<ContractExecutableType>),
    ContractExecutable(Box<ContractExecutable>),
    ScAddressType(Box<ScAddressType>),
    MuxedEd25519Account(Box<MuxedEd25519Account>),
    ScAddress(Box<ScAddress>),
    ScVec(Box<ScVec>),
    ScMap(Box<ScMap>),
    ScBytes(Box<ScBytes>),
    ScString(Box<ScString>),
    ScSymbol(Box<ScSymbol>),
    ScNonceKey(Box<ScNonceKey>),
    ScContractInstance(Box<ScContractInstance>),
    ScVal(Box<ScVal>),
    ScMapEntry(Box<ScMapEntry>),
    LedgerCloseMetaBatch(Box<LedgerCloseMetaBatch>),
    StoredTransactionSet(Box<StoredTransactionSet>),
    StoredDebugTransactionSet(Box<StoredDebugTransactionSet>),
    PersistedScpStateV0(Box<PersistedScpStateV0>),
    PersistedScpStateV1(Box<PersistedScpStateV1>),
    PersistedScpState(Box<PersistedScpState>),
    Thresholds(Box<Thresholds>),
    String32(Box<String32>),
    String64(Box<String64>),
    SequenceNumber(Box<SequenceNumber>),
    DataValue(Box<DataValue>),
    AssetCode4(Box<AssetCode4>),
    AssetCode12(Box<AssetCode12>),
    AssetType(Box<AssetType>),
    AssetCode(Box<AssetCode>),
    AlphaNum4(Box<AlphaNum4>),
    AlphaNum12(Box<AlphaNum12>),
    Asset(Box<Asset>),
    Price(Box<Price>),
    Liabilities(Box<Liabilities>),
    ThresholdIndexes(Box<ThresholdIndexes>),
    LedgerEntryType(Box<LedgerEntryType>),
    Signer(Box<Signer>),
    AccountFlags(Box<AccountFlags>),
    SponsorshipDescriptor(Box<SponsorshipDescriptor>),
    AccountEntryExtensionV3(Box<AccountEntryExtensionV3>),
    AccountEntryExtensionV2(Box<AccountEntryExtensionV2>),
    AccountEntryExtensionV2Ext(Box<AccountEntryExtensionV2Ext>),
    AccountEntryExtensionV1(Box<AccountEntryExtensionV1>),
    AccountEntryExtensionV1Ext(Box<AccountEntryExtensionV1Ext>),
    AccountEntry(Box<AccountEntry>),
    AccountEntryExt(Box<AccountEntryExt>),
    TrustLineFlags(Box<TrustLineFlags>),
    LiquidityPoolType(Box<LiquidityPoolType>),
    TrustLineAsset(Box<TrustLineAsset>),
    TrustLineEntryExtensionV2(Box<TrustLineEntryExtensionV2>),
    TrustLineEntryExtensionV2Ext(Box<TrustLineEntryExtensionV2Ext>),
    TrustLineEntry(Box<TrustLineEntry>),
    TrustLineEntryExt(Box<TrustLineEntryExt>),
    TrustLineEntryV1(Box<TrustLineEntryV1>),
    TrustLineEntryV1Ext(Box<TrustLineEntryV1Ext>),
    OfferEntryFlags(Box<OfferEntryFlags>),
    OfferEntry(Box<OfferEntry>),
    OfferEntryExt(Box<OfferEntryExt>),
    DataEntry(Box<DataEntry>),
    DataEntryExt(Box<DataEntryExt>),
    ClaimPredicateType(Box<ClaimPredicateType>),
    ClaimPredicate(Box<ClaimPredicate>),
    ClaimantType(Box<ClaimantType>),
    Claimant(Box<Claimant>),
    ClaimantV0(Box<ClaimantV0>),
    ClaimableBalanceFlags(Box<ClaimableBalanceFlags>),
    ClaimableBalanceEntryExtensionV1(Box<ClaimableBalanceEntryExtensionV1>),
    ClaimableBalanceEntryExtensionV1Ext(Box<ClaimableBalanceEntryExtensionV1Ext>),
    ClaimableBalanceEntry(Box<ClaimableBalanceEntry>),
    ClaimableBalanceEntryExt(Box<ClaimableBalanceEntryExt>),
    LiquidityPoolConstantProductParameters(Box<LiquidityPoolConstantProductParameters>),
    LiquidityPoolEntry(Box<LiquidityPoolEntry>),
    LiquidityPoolEntryBody(Box<LiquidityPoolEntryBody>),
    LiquidityPoolEntryConstantProduct(Box<LiquidityPoolEntryConstantProduct>),
    ContractDataDurability(Box<ContractDataDurability>),
    ContractDataEntry(Box<ContractDataEntry>),
    ContractCodeCostInputs(Box<ContractCodeCostInputs>),
    ContractCodeEntry(Box<ContractCodeEntry>),
    ContractCodeEntryExt(Box<ContractCodeEntryExt>),
    ContractCodeEntryV1(Box<ContractCodeEntryV1>),
    TtlEntry(Box<TtlEntry>),
    LedgerEntryExtensionV1(Box<LedgerEntryExtensionV1>),
    LedgerEntryExtensionV1Ext(Box<LedgerEntryExtensionV1Ext>),
    LedgerEntry(Box<LedgerEntry>),
    LedgerEntryData(Box<LedgerEntryData>),
    LedgerEntryExt(Box<LedgerEntryExt>),
    LedgerKey(Box<LedgerKey>),
    LedgerKeyAccount(Box<LedgerKeyAccount>),
    LedgerKeyTrustLine(Box<LedgerKeyTrustLine>),
    LedgerKeyOffer(Box<LedgerKeyOffer>),
    LedgerKeyData(Box<LedgerKeyData>),
    LedgerKeyClaimableBalance(Box<LedgerKeyClaimableBalance>),
    LedgerKeyLiquidityPool(Box<LedgerKeyLiquidityPool>),
    LedgerKeyContractData(Box<LedgerKeyContractData>),
    LedgerKeyContractCode(Box<LedgerKeyContractCode>),
    LedgerKeyConfigSetting(Box<LedgerKeyConfigSetting>),
    LedgerKeyTtl(Box<LedgerKeyTtl>),
    EnvelopeType(Box<EnvelopeType>),
    BucketListType(Box<BucketListType>),
    BucketEntryType(Box<BucketEntryType>),
    HotArchiveBucketEntryType(Box<HotArchiveBucketEntryType>),
    BucketMetadata(Box<BucketMetadata>),
    BucketMetadataExt(Box<BucketMetadataExt>),
    BucketEntry(Box<BucketEntry>),
    HotArchiveBucketEntry(Box<HotArchiveBucketEntry>),
    UpgradeType(Box<UpgradeType>),
    StellarValueType(Box<StellarValueType>),
    LedgerCloseValueSignature(Box<LedgerCloseValueSignature>),
    StellarValue(Box<StellarValue>),
    StellarValueExt(Box<StellarValueExt>),
    LedgerHeaderFlags(Box<LedgerHeaderFlags>),
    LedgerHeaderExtensionV1(Box<LedgerHeaderExtensionV1>),
    LedgerHeaderExtensionV1Ext(Box<LedgerHeaderExtensionV1Ext>),
    LedgerHeader(Box<LedgerHeader>),
    LedgerHeaderExt(Box<LedgerHeaderExt>),
    LedgerUpgradeType(Box<LedgerUpgradeType>),
    ConfigUpgradeSetKey(Box<ConfigUpgradeSetKey>),
    LedgerUpgrade(Box<LedgerUpgrade>),
    ConfigUpgradeSet(Box<ConfigUpgradeSet>),
    TxSetComponentType(Box<TxSetComponentType>),
    DependentTxCluster(Box<DependentTxCluster>),
    ParallelTxExecutionStage(Box<ParallelTxExecutionStage>),
    ParallelTxsComponent(Box<ParallelTxsComponent>),
    TxSetComponent(Box<TxSetComponent>),
    TxSetComponentTxsMaybeDiscountedFee(Box<TxSetComponentTxsMaybeDiscountedFee>),
    TransactionPhase(Box<TransactionPhase>),
    TransactionSet(Box<TransactionSet>),
    TransactionSetV1(Box<TransactionSetV1>),
    GeneralizedTransactionSet(Box<GeneralizedTransactionSet>),
    TransactionResultPair(Box<TransactionResultPair>),
    TransactionResultSet(Box<TransactionResultSet>),
    TransactionHistoryEntry(Box<TransactionHistoryEntry>),
    TransactionHistoryEntryExt(Box<TransactionHistoryEntryExt>),
    TransactionHistoryResultEntry(Box<TransactionHistoryResultEntry>),
    TransactionHistoryResultEntryExt(Box<TransactionHistoryResultEntryExt>),
    LedgerHeaderHistoryEntry(Box<LedgerHeaderHistoryEntry>),
    LedgerHeaderHistoryEntryExt(Box<LedgerHeaderHistoryEntryExt>),
    LedgerScpMessages(Box<LedgerScpMessages>),
    ScpHistoryEntryV0(Box<ScpHistoryEntryV0>),
    ScpHistoryEntry(Box<ScpHistoryEntry>),
    LedgerEntryChangeType(Box<LedgerEntryChangeType>),
    LedgerEntryChange(Box<LedgerEntryChange>),
    LedgerEntryChanges(Box<LedgerEntryChanges>),
    OperationMeta(Box<OperationMeta>),
    TransactionMetaV1(Box<TransactionMetaV1>),
    TransactionMetaV2(Box<TransactionMetaV2>),
    ContractEventType(Box<ContractEventType>),
    ContractEvent(Box<ContractEvent>),
    ContractEventBody(Box<ContractEventBody>),
    ContractEventV0(Box<ContractEventV0>),
    DiagnosticEvent(Box<DiagnosticEvent>),
    SorobanTransactionMetaExtV1(Box<SorobanTransactionMetaExtV1>),
    SorobanTransactionMetaExt(Box<SorobanTransactionMetaExt>),
    SorobanTransactionMeta(Box<SorobanTransactionMeta>),
    TransactionMetaV3(Box<TransactionMetaV3>),
    OperationMetaV2(Box<OperationMetaV2>),
    SorobanTransactionMetaV2(Box<SorobanTransactionMetaV2>),
    TransactionEventStage(Box<TransactionEventStage>),
    TransactionEvent(Box<TransactionEvent>),
    TransactionMetaV4(Box<TransactionMetaV4>),
    InvokeHostFunctionSuccessPreImage(Box<InvokeHostFunctionSuccessPreImage>),
    TransactionMeta(Box<TransactionMeta>),
    TransactionResultMeta(Box<TransactionResultMeta>),
    TransactionResultMetaV1(Box<TransactionResultMetaV1>),
    UpgradeEntryMeta(Box<UpgradeEntryMeta>),
    LedgerCloseMetaV0(Box<LedgerCloseMetaV0>),
    LedgerCloseMetaExtV1(Box<LedgerCloseMetaExtV1>),
    LedgerCloseMetaExt(Box<LedgerCloseMetaExt>),
    LedgerCloseMetaV1(Box<LedgerCloseMetaV1>),
    LedgerCloseMetaV2(Box<LedgerCloseMetaV2>),
    LedgerCloseMeta(Box<LedgerCloseMeta>),
    ErrorCode(Box<ErrorCode>),
    SError(Box<SError>),
    SendMore(Box<SendMore>),
    SendMoreExtended(Box<SendMoreExtended>),
    AuthCert(Box<AuthCert>),
    Hello(Box<Hello>),
    Auth(Box<Auth>),
    IpAddrType(Box<IpAddrType>),
    PeerAddress(Box<PeerAddress>),
    PeerAddressIp(Box<PeerAddressIp>),
    MessageType(Box<MessageType>),
    DontHave(Box<DontHave>),
    SurveyMessageCommandType(Box<SurveyMessageCommandType>),
    SurveyMessageResponseType(Box<SurveyMessageResponseType>),
    TimeSlicedSurveyStartCollectingMessage(Box<TimeSlicedSurveyStartCollectingMessage>),
    SignedTimeSlicedSurveyStartCollectingMessage(Box<SignedTimeSlicedSurveyStartCollectingMessage>),
    TimeSlicedSurveyStopCollectingMessage(Box<TimeSlicedSurveyStopCollectingMessage>),
    SignedTimeSlicedSurveyStopCollectingMessage(Box<SignedTimeSlicedSurveyStopCollectingMessage>),
    SurveyRequestMessage(Box<SurveyRequestMessage>),
    TimeSlicedSurveyRequestMessage(Box<TimeSlicedSurveyRequestMessage>),
    SignedTimeSlicedSurveyRequestMessage(Box<SignedTimeSlicedSurveyRequestMessage>),
    EncryptedBody(Box<EncryptedBody>),
    SurveyResponseMessage(Box<SurveyResponseMessage>),
    TimeSlicedSurveyResponseMessage(Box<TimeSlicedSurveyResponseMessage>),
    SignedTimeSlicedSurveyResponseMessage(Box<SignedTimeSlicedSurveyResponseMessage>),
    PeerStats(Box<PeerStats>),
    TimeSlicedNodeData(Box<TimeSlicedNodeData>),
    TimeSlicedPeerData(Box<TimeSlicedPeerData>),
    TimeSlicedPeerDataList(Box<TimeSlicedPeerDataList>),
    TopologyResponseBodyV2(Box<TopologyResponseBodyV2>),
    SurveyResponseBody(Box<SurveyResponseBody>),
    TxAdvertVector(Box<TxAdvertVector>),
    FloodAdvert(Box<FloodAdvert>),
    TxDemandVector(Box<TxDemandVector>),
    FloodDemand(Box<FloodDemand>),
    StellarMessage(Box<StellarMessage>),
    AuthenticatedMessage(Box<AuthenticatedMessage>),
    AuthenticatedMessageV0(Box<AuthenticatedMessageV0>),
    LiquidityPoolParameters(Box<LiquidityPoolParameters>),
    MuxedAccount(Box<MuxedAccount>),
    MuxedAccountMed25519(Box<MuxedAccountMed25519>),
    DecoratedSignature(Box<DecoratedSignature>),
    OperationType(Box<OperationType>),
    CreateAccountOp(Box<CreateAccountOp>),
    PaymentOp(Box<PaymentOp>),
    PathPaymentStrictReceiveOp(Box<PathPaymentStrictReceiveOp>),
    PathPaymentStrictSendOp(Box<PathPaymentStrictSendOp>),
    ManageSellOfferOp(Box<ManageSellOfferOp>),
    ManageBuyOfferOp(Box<ManageBuyOfferOp>),
    CreatePassiveSellOfferOp(Box<CreatePassiveSellOfferOp>),
    SetOptionsOp(Box<SetOptionsOp>),
    ChangeTrustAsset(Box<ChangeTrustAsset>),
    ChangeTrustOp(Box<ChangeTrustOp>),
    AllowTrustOp(Box<AllowTrustOp>),
    ManageDataOp(Box<ManageDataOp>),
    BumpSequenceOp(Box<BumpSequenceOp>),
    CreateClaimableBalanceOp(Box<CreateClaimableBalanceOp>),
    ClaimClaimableBalanceOp(Box<ClaimClaimableBalanceOp>),
    BeginSponsoringFutureReservesOp(Box<BeginSponsoringFutureReservesOp>),
    RevokeSponsorshipType(Box<RevokeSponsorshipType>),
    RevokeSponsorshipOp(Box<RevokeSponsorshipOp>),
    RevokeSponsorshipOpSigner(Box<RevokeSponsorshipOpSigner>),
    ClawbackOp(Box<ClawbackOp>),
    ClawbackClaimableBalanceOp(Box<ClawbackClaimableBalanceOp>),
    SetTrustLineFlagsOp(Box<SetTrustLineFlagsOp>),
    LiquidityPoolDepositOp(Box<LiquidityPoolDepositOp>),
    LiquidityPoolWithdrawOp(Box<LiquidityPoolWithdrawOp>),
    HostFunctionType(Box<HostFunctionType>),
    ContractIdPreimageType(Box<ContractIdPreimageType>),
    ContractIdPreimage(Box<ContractIdPreimage>),
    ContractIdPreimageFromAddress(Box<ContractIdPreimageFromAddress>),
    CreateContractArgs(Box<CreateContractArgs>),
    CreateContractArgsV2(Box<CreateContractArgsV2>),
    InvokeContractArgs(Box<InvokeContractArgs>),
    HostFunction(Box<HostFunction>),
    SorobanAuthorizedFunctionType(Box<SorobanAuthorizedFunctionType>),
    SorobanAuthorizedFunction(Box<SorobanAuthorizedFunction>),
    SorobanAuthorizedInvocation(Box<SorobanAuthorizedInvocation>),
    SorobanAddressCredentials(Box<SorobanAddressCredentials>),
    SorobanCredentialsType(Box<SorobanCredentialsType>),
    SorobanCredentials(Box<SorobanCredentials>),
    SorobanAuthorizationEntry(Box<SorobanAuthorizationEntry>),
    SorobanAuthorizationEntries(Box<SorobanAuthorizationEntries>),
    InvokeHostFunctionOp(Box<InvokeHostFunctionOp>),
    ExtendFootprintTtlOp(Box<ExtendFootprintTtlOp>),
    RestoreFootprintOp(Box<RestoreFootprintOp>),
    Operation(Box<Operation>),
    OperationBody(Box<OperationBody>),
    HashIdPreimage(Box<HashIdPreimage>),
    HashIdPreimageOperationId(Box<HashIdPreimageOperationId>),
    HashIdPreimageRevokeId(Box<HashIdPreimageRevokeId>),
    HashIdPreimageContractId(Box<HashIdPreimageContractId>),
    HashIdPreimageSorobanAuthorization(Box<HashIdPreimageSorobanAuthorization>),
    MemoType(Box<MemoType>),
    Memo(Box<Memo>),
    TimeBounds(Box<TimeBounds>),
    LedgerBounds(Box<LedgerBounds>),
    PreconditionsV2(Box<PreconditionsV2>),
    PreconditionType(Box<PreconditionType>),
    Preconditions(Box<Preconditions>),
    LedgerFootprint(Box<LedgerFootprint>),
    SorobanResources(Box<SorobanResources>),
    SorobanResourcesExtV0(Box<SorobanResourcesExtV0>),
    SorobanTransactionData(Box<SorobanTransactionData>),
    SorobanTransactionDataExt(Box<SorobanTransactionDataExt>),
    TransactionV0(Box<TransactionV0>),
    TransactionV0Ext(Box<TransactionV0Ext>),
    TransactionV0Envelope(Box<TransactionV0Envelope>),
    Transaction(Box<Transaction>),
    TransactionExt(Box<TransactionExt>),
    TransactionV1Envelope(Box<TransactionV1Envelope>),
    FeeBumpTransaction(Box<FeeBumpTransaction>),
    FeeBumpTransactionInnerTx(Box<FeeBumpTransactionInnerTx>),
    FeeBumpTransactionExt(Box<FeeBumpTransactionExt>),
    FeeBumpTransactionEnvelope(Box<FeeBumpTransactionEnvelope>),
    TransactionEnvelope(Box<TransactionEnvelope>),
    TransactionSignaturePayload(Box<TransactionSignaturePayload>),
    TransactionSignaturePayloadTaggedTransaction(Box<TransactionSignaturePayloadTaggedTransaction>),
    ClaimAtomType(Box<ClaimAtomType>),
    ClaimOfferAtomV0(Box<ClaimOfferAtomV0>),
    ClaimOfferAtom(Box<ClaimOfferAtom>),
    ClaimLiquidityAtom(Box<ClaimLiquidityAtom>),
    ClaimAtom(Box<ClaimAtom>),
    CreateAccountResultCode(Box<CreateAccountResultCode>),
    CreateAccountResult(Box<CreateAccountResult>),
    PaymentResultCode(Box<PaymentResultCode>),
    PaymentResult(Box<PaymentResult>),
    PathPaymentStrictReceiveResultCode(Box<PathPaymentStrictReceiveResultCode>),
    SimplePaymentResult(Box<SimplePaymentResult>),
    PathPaymentStrictReceiveResult(Box<PathPaymentStrictReceiveResult>),
    PathPaymentStrictReceiveResultSuccess(Box<PathPaymentStrictReceiveResultSuccess>),
    PathPaymentStrictSendResultCode(Box<PathPaymentStrictSendResultCode>),
    PathPaymentStrictSendResult(Box<PathPaymentStrictSendResult>),
    PathPaymentStrictSendResultSuccess(Box<PathPaymentStrictSendResultSuccess>),
    ManageSellOfferResultCode(Box<ManageSellOfferResultCode>),
    ManageOfferEffect(Box<ManageOfferEffect>),
    ManageOfferSuccessResult(Box<ManageOfferSuccessResult>),
    ManageOfferSuccessResultOffer(Box<ManageOfferSuccessResultOffer>),
    ManageSellOfferResult(Box<ManageSellOfferResult>),
    ManageBuyOfferResultCode(Box<ManageBuyOfferResultCode>),
    ManageBuyOfferResult(Box<ManageBuyOfferResult>),
    SetOptionsResultCode(Box<SetOptionsResultCode>),
    SetOptionsResult(Box<SetOptionsResult>),
    ChangeTrustResultCode(Box<ChangeTrustResultCode>),
    ChangeTrustResult(Box<ChangeTrustResult>),
    AllowTrustResultCode(Box<AllowTrustResultCode>),
    AllowTrustResult(Box<AllowTrustResult>),
    AccountMergeResultCode(Box<AccountMergeResultCode>),
    AccountMergeResult(Box<AccountMergeResult>),
    InflationResultCode(Box<InflationResultCode>),
    InflationPayout(Box<InflationPayout>),
    InflationResult(Box<InflationResult>),
    ManageDataResultCode(Box<ManageDataResultCode>),
    ManageDataResult(Box<ManageDataResult>),
    BumpSequenceResultCode(Box<BumpSequenceResultCode>),
    BumpSequenceResult(Box<BumpSequenceResult>),
    CreateClaimableBalanceResultCode(Box<CreateClaimableBalanceResultCode>),
    CreateClaimableBalanceResult(Box<CreateClaimableBalanceResult>),
    ClaimClaimableBalanceResultCode(Box<ClaimClaimableBalanceResultCode>),
    ClaimClaimableBalanceResult(Box<ClaimClaimableBalanceResult>),
    BeginSponsoringFutureReservesResultCode(Box<BeginSponsoringFutureReservesResultCode>),
    BeginSponsoringFutureReservesResult(Box<BeginSponsoringFutureReservesResult>),
    EndSponsoringFutureReservesResultCode(Box<EndSponsoringFutureReservesResultCode>),
    EndSponsoringFutureReservesResult(Box<EndSponsoringFutureReservesResult>),
    RevokeSponsorshipResultCode(Box<RevokeSponsorshipResultCode>),
    RevokeSponsorshipResult(Box<RevokeSponsorshipResult>),
    ClawbackResultCode(Box<ClawbackResultCode>),
    ClawbackResult(Box<ClawbackResult>),
    ClawbackClaimableBalanceResultCode(Box<ClawbackClaimableBalanceResultCode>),
    ClawbackClaimableBalanceResult(Box<ClawbackClaimableBalanceResult>),
    SetTrustLineFlagsResultCode(Box<SetTrustLineFlagsResultCode>),
    SetTrustLineFlagsResult(Box<SetTrustLineFlagsResult>),
    LiquidityPoolDepositResultCode(Box<LiquidityPoolDepositResultCode>),
    LiquidityPoolDepositResult(Box<LiquidityPoolDepositResult>),
    LiquidityPoolWithdrawResultCode(Box<LiquidityPoolWithdrawResultCode>),
    LiquidityPoolWithdrawResult(Box<LiquidityPoolWithdrawResult>),
    InvokeHostFunctionResultCode(Box<InvokeHostFunctionResultCode>),
    InvokeHostFunctionResult(Box<InvokeHostFunctionResult>),
    ExtendFootprintTtlResultCode(Box<ExtendFootprintTtlResultCode>),
    ExtendFootprintTtlResult(Box<ExtendFootprintTtlResult>),
    RestoreFootprintResultCode(Box<RestoreFootprintResultCode>),
    RestoreFootprintResult(Box<RestoreFootprintResult>),
    OperationResultCode(Box<OperationResultCode>),
    OperationResult(Box<OperationResult>),
    OperationResultTr(Box<OperationResultTr>),
    TransactionResultCode(Box<TransactionResultCode>),
    InnerTransactionResult(Box<InnerTransactionResult>),
    InnerTransactionResultResult(Box<InnerTransactionResultResult>),
    InnerTransactionResultExt(Box<InnerTransactionResultExt>),
    InnerTransactionResultPair(Box<InnerTransactionResultPair>),
    TransactionResult(Box<TransactionResult>),
    TransactionResultResult(Box<TransactionResultResult>),
    TransactionResultExt(Box<TransactionResultExt>),
    Hash(Box<Hash>),
    Uint256(Box<Uint256>),
    Uint32(Box<Uint32>),
    Int32(Box<Int32>),
    Uint64(Box<Uint64>),
    Int64(Box<Int64>),
    TimePoint(Box<TimePoint>),
    Duration(Box<Duration>),
    ExtensionPoint(Box<ExtensionPoint>),
    CryptoKeyType(Box<CryptoKeyType>),
    PublicKeyType(Box<PublicKeyType>),
    SignerKeyType(Box<SignerKeyType>),
    PublicKey(Box<PublicKey>),
    SignerKey(Box<SignerKey>),
    SignerKeyEd25519SignedPayload(Box<SignerKeyEd25519SignedPayload>),
    Signature(Box<Signature>),
    SignatureHint(Box<SignatureHint>),
    NodeId(Box<NodeId>),
    AccountId(Box<AccountId>),
    ContractId(Box<ContractId>),
    Curve25519Secret(Box<Curve25519Secret>),
    Curve25519Public(Box<Curve25519Public>),
    HmacSha256Key(Box<HmacSha256Key>),
    HmacSha256Mac(Box<HmacSha256Mac>),
    ShortHashSeed(Box<ShortHashSeed>),
    BinaryFuseFilterType(Box<BinaryFuseFilterType>),
    SerializedBinaryFuseFilter(Box<SerializedBinaryFuseFilter>),
    PoolId(Box<PoolId>),
    ClaimableBalanceIdType(Box<ClaimableBalanceIdType>),
    ClaimableBalanceId(Box<ClaimableBalanceId>),
}

