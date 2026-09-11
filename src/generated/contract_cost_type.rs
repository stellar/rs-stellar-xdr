#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractCostType is an XDR Enum defined as:
///
/// ```text
/// enum ContractCostType {
///     // Cost of running 1 wasm instruction
///     WasmInsnExec = 0,
///     // Cost of allocating a slice of memory (in bytes)
///     MemAlloc = 1,
///     // Cost of copying a slice of bytes into a pre-allocated memory
///     MemCpy = 2,
///     // Cost of comparing two slices of memory
///     MemCmp = 3,
///     // Cost of a host function dispatch, not including the actual work done by
///     // the function nor the cost of VM invocation machinary
///     DispatchHostFunction = 4,
///     // Cost of visiting a host object from the host object storage. Exists to
///     // make sure some baseline cost coverage, i.e. repeatly visiting objects
///     // by the guest will always incur some charges.
///     VisitObject = 5,
///     // Cost of serializing an xdr object to bytes
///     ValSer = 6,
///     // Cost of deserializing an xdr object from bytes
///     ValDeser = 7,
///     // Cost of computing the sha256 hash from bytes
///     ComputeSha256Hash = 8,
///     // Cost of computing the ed25519 pubkey from bytes
///     ComputeEd25519PubKey = 9,
///     // Cost of verifying ed25519 signature of a payload.
///     VerifyEd25519Sig = 10,
///     // Cost of instantiation a VM from wasm bytes code.
///     VmInstantiation = 11,
///     // Cost of instantiation a VM from a cached state.
///     VmCachedInstantiation = 12,
///     // Cost of invoking a function on the VM. If the function is a host function,
///     // additional cost will be covered by `DispatchHostFunction`.
///     InvokeVmFunction = 13,
///     // Cost of computing a keccak256 hash from bytes.
///     ComputeKeccak256Hash = 14,
///     // Cost of decoding an ECDSA signature computed from a 256-bit prime modulus
///     // curve (e.g. secp256k1 and secp256r1)
///     DecodeEcdsaCurve256Sig = 15,
///     // Cost of recovering an ECDSA secp256k1 key from a signature.
///     RecoverEcdsaSecp256k1Key = 16,
///     // Cost of int256 addition (`+`) and subtraction (`-`) operations
///     Int256AddSub = 17,
///     // Cost of int256 multiplication (`*`) operation
///     Int256Mul = 18,
///     // Cost of int256 division (`/`) operation
///     Int256Div = 19,
///     // Cost of int256 power (`exp`) operation
///     Int256Pow = 20,
///     // Cost of int256 shift (`shl`, `shr`) operation
///     Int256Shift = 21,
///     // Cost of drawing random bytes using a ChaCha20 PRNG
///     ChaCha20DrawBytes = 22,
///
///     // Cost of parsing wasm bytes that only encode instructions.
///     ParseWasmInstructions = 23,
///     // Cost of parsing a known number of wasm functions.
///     ParseWasmFunctions = 24,
///     // Cost of parsing a known number of wasm globals.
///     ParseWasmGlobals = 25,
///     // Cost of parsing a known number of wasm table entries.
///     ParseWasmTableEntries = 26,
///     // Cost of parsing a known number of wasm types.
///     ParseWasmTypes = 27,
///     // Cost of parsing a known number of wasm data segments.
///     ParseWasmDataSegments = 28,
///     // Cost of parsing a known number of wasm element segments.
///     ParseWasmElemSegments = 29,
///     // Cost of parsing a known number of wasm imports.
///     ParseWasmImports = 30,
///     // Cost of parsing a known number of wasm exports.
///     ParseWasmExports = 31,
///     // Cost of parsing a known number of data segment bytes.
///     ParseWasmDataSegmentBytes = 32,
///
///     // Cost of instantiating wasm bytes that only encode instructions.
///     InstantiateWasmInstructions = 33,
///     // Cost of instantiating a known number of wasm functions.
///     InstantiateWasmFunctions = 34,
///     // Cost of instantiating a known number of wasm globals.
///     InstantiateWasmGlobals = 35,
///     // Cost of instantiating a known number of wasm table entries.
///     InstantiateWasmTableEntries = 36,
///     // Cost of instantiating a known number of wasm types.
///     InstantiateWasmTypes = 37,
///     // Cost of instantiating a known number of wasm data segments.
///     InstantiateWasmDataSegments = 38,
///     // Cost of instantiating a known number of wasm element segments.
///     InstantiateWasmElemSegments = 39,
///     // Cost of instantiating a known number of wasm imports.
///     InstantiateWasmImports = 40,
///     // Cost of instantiating a known number of wasm exports.
///     InstantiateWasmExports = 41,
///     // Cost of instantiating a known number of data segment bytes.
///     InstantiateWasmDataSegmentBytes = 42,
///
///     // Cost of decoding a bytes array representing an uncompressed SEC-1 encoded
///     // point on a 256-bit elliptic curve
///     Sec1DecodePointUncompressed = 43,
///     // Cost of verifying an ECDSA Secp256r1 signature
///     VerifyEcdsaSecp256r1Sig = 44,
///
///     // Cost of encoding a BLS12-381 Fp (base field element)
///     Bls12381EncodeFp = 45,
///     // Cost of decoding a BLS12-381 Fp (base field element)
///     Bls12381DecodeFp = 46,
///     // Cost of checking a G1 point lies on the curve
///     Bls12381G1CheckPointOnCurve = 47,
///     // Cost of checking a G1 point belongs to the correct subgroup
///     Bls12381G1CheckPointInSubgroup = 48,
///     // Cost of checking a G2 point lies on the curve
///     Bls12381G2CheckPointOnCurve = 49,
///     // Cost of checking a G2 point belongs to the correct subgroup
///     Bls12381G2CheckPointInSubgroup = 50,
///     // Cost of converting a BLS12-381 G1 point from projective to affine coordinates
///     Bls12381G1ProjectiveToAffine = 51,
///     // Cost of converting a BLS12-381 G2 point from projective to affine coordinates
///     Bls12381G2ProjectiveToAffine = 52,
///     // Cost of performing BLS12-381 G1 point addition
///     Bls12381G1Add = 53,
///     // Cost of performing BLS12-381 G1 scalar multiplication
///     Bls12381G1Mul = 54,
///     // Cost of performing BLS12-381 G1 multi-scalar multiplication (MSM)
///     Bls12381G1Msm = 55,
///     // Cost of mapping a BLS12-381 Fp field element to a G1 point
///     Bls12381MapFpToG1 = 56,
///     // Cost of hashing to a BLS12-381 G1 point
///     Bls12381HashToG1 = 57,
///     // Cost of performing BLS12-381 G2 point addition
///     Bls12381G2Add = 58,
///     // Cost of performing BLS12-381 G2 scalar multiplication
///     Bls12381G2Mul = 59,
///     // Cost of performing BLS12-381 G2 multi-scalar multiplication (MSM)
///     Bls12381G2Msm = 60,
///     // Cost of mapping a BLS12-381 Fp2 field element to a G2 point
///     Bls12381MapFp2ToG2 = 61,
///     // Cost of hashing to a BLS12-381 G2 point
///     Bls12381HashToG2 = 62,
///     // Cost of performing BLS12-381 pairing operation
///     Bls12381Pairing = 63,
///     // Cost of converting a BLS12-381 scalar element from U256
///     Bls12381FrFromU256 = 64,
///     // Cost of converting a BLS12-381 scalar element to U256
///     Bls12381FrToU256 = 65,
///     // Cost of performing BLS12-381 scalar element addition/subtraction
///     Bls12381FrAddSub = 66,
///     // Cost of performing BLS12-381 scalar element multiplication
///     Bls12381FrMul = 67,
///     // Cost of performing BLS12-381 scalar element exponentiation
///     Bls12381FrPow = 68,
///     // Cost of performing BLS12-381 scalar element inversion
///     Bls12381FrInv = 69,
///
///     // Cost of encoding a BN254 Fp (base field element)
///     Bn254EncodeFp = 70,
///     // Cost of decoding a BN254 Fp (base field element)
///     Bn254DecodeFp = 71,
///     // Cost of checking a G1 point lies on the curve
///     Bn254G1CheckPointOnCurve = 72,
///     // Cost of checking a G2 point lies on the curve
///     Bn254G2CheckPointOnCurve = 73,
///     // Cost of checking a G2 point belongs to the correct subgroup
///     Bn254G2CheckPointInSubgroup = 74,
///     // Cost of converting a BN254 G1 point from projective to affine coordinates
///     Bn254G1ProjectiveToAffine = 75,
///     // Cost of performing BN254 G1 point addition
///     Bn254G1Add = 76,
///     // Cost of performing BN254 G1 scalar multiplication
///     Bn254G1Mul = 77,
///     // Cost of performing BN254 pairing operation
///     Bn254Pairing = 78,
///     // Cost of converting a BN254 scalar element from U256
///     Bn254FrFromU256 = 79,
///     // Cost of converting a BN254 scalar element to U256
///     Bn254FrToU256 = 80,
///     // // Cost of performing BN254 scalar element addition/subtraction
///     Bn254FrAddSub = 81,
///     // Cost of performing BN254 scalar element multiplication
///     Bn254FrMul = 82,
///     // Cost of performing BN254 scalar element exponentiation
///     Bn254FrPow = 83,
///      // Cost of performing BN254 scalar element inversion
///     Bn254FrInv = 84,
///     // Cost of performing BN254 G1 multi-scalar multiplication (MSM)
///     Bn254G1Msm = 85
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum ContractCostType {
    #[cfg_attr(feature = "alloc", default)]
    WasmInsnExec = 0,
    MemAlloc = 1,
    MemCpy = 2,
    MemCmp = 3,
    DispatchHostFunction = 4,
    VisitObject = 5,
    ValSer = 6,
    ValDeser = 7,
    ComputeSha256Hash = 8,
    ComputeEd25519PubKey = 9,
    VerifyEd25519Sig = 10,
    VmInstantiation = 11,
    VmCachedInstantiation = 12,
    InvokeVmFunction = 13,
    ComputeKeccak256Hash = 14,
    DecodeEcdsaCurve256Sig = 15,
    RecoverEcdsaSecp256k1Key = 16,
    Int256AddSub = 17,
    Int256Mul = 18,
    Int256Div = 19,
    Int256Pow = 20,
    Int256Shift = 21,
    ChaCha20DrawBytes = 22,
    ParseWasmInstructions = 23,
    ParseWasmFunctions = 24,
    ParseWasmGlobals = 25,
    ParseWasmTableEntries = 26,
    ParseWasmTypes = 27,
    ParseWasmDataSegments = 28,
    ParseWasmElemSegments = 29,
    ParseWasmImports = 30,
    ParseWasmExports = 31,
    ParseWasmDataSegmentBytes = 32,
    InstantiateWasmInstructions = 33,
    InstantiateWasmFunctions = 34,
    InstantiateWasmGlobals = 35,
    InstantiateWasmTableEntries = 36,
    InstantiateWasmTypes = 37,
    InstantiateWasmDataSegments = 38,
    InstantiateWasmElemSegments = 39,
    InstantiateWasmImports = 40,
    InstantiateWasmExports = 41,
    InstantiateWasmDataSegmentBytes = 42,
    Sec1DecodePointUncompressed = 43,
    VerifyEcdsaSecp256r1Sig = 44,
    Bls12381EncodeFp = 45,
    Bls12381DecodeFp = 46,
    Bls12381G1CheckPointOnCurve = 47,
    Bls12381G1CheckPointInSubgroup = 48,
    Bls12381G2CheckPointOnCurve = 49,
    Bls12381G2CheckPointInSubgroup = 50,
    Bls12381G1ProjectiveToAffine = 51,
    Bls12381G2ProjectiveToAffine = 52,
    Bls12381G1Add = 53,
    Bls12381G1Mul = 54,
    Bls12381G1Msm = 55,
    Bls12381MapFpToG1 = 56,
    Bls12381HashToG1 = 57,
    Bls12381G2Add = 58,
    Bls12381G2Mul = 59,
    Bls12381G2Msm = 60,
    Bls12381MapFp2ToG2 = 61,
    Bls12381HashToG2 = 62,
    Bls12381Pairing = 63,
    Bls12381FrFromU256 = 64,
    Bls12381FrToU256 = 65,
    Bls12381FrAddSub = 66,
    Bls12381FrMul = 67,
    Bls12381FrPow = 68,
    Bls12381FrInv = 69,
    Bn254EncodeFp = 70,
    Bn254DecodeFp = 71,
    Bn254G1CheckPointOnCurve = 72,
    Bn254G2CheckPointOnCurve = 73,
    Bn254G2CheckPointInSubgroup = 74,
    Bn254G1ProjectiveToAffine = 75,
    Bn254G1Add = 76,
    Bn254G1Mul = 77,
    Bn254Pairing = 78,
    Bn254FrFromU256 = 79,
    Bn254FrToU256 = 80,
    Bn254FrAddSub = 81,
    Bn254FrMul = 82,
    Bn254FrPow = 83,
    Bn254FrInv = 84,
    Bn254G1Msm = 85,
}

impl ContractCostType {
    const _VARIANTS: &[ContractCostType] = &[
        ContractCostType::WasmInsnExec,
        ContractCostType::MemAlloc,
        ContractCostType::MemCpy,
        ContractCostType::MemCmp,
        ContractCostType::DispatchHostFunction,
        ContractCostType::VisitObject,
        ContractCostType::ValSer,
        ContractCostType::ValDeser,
        ContractCostType::ComputeSha256Hash,
        ContractCostType::ComputeEd25519PubKey,
        ContractCostType::VerifyEd25519Sig,
        ContractCostType::VmInstantiation,
        ContractCostType::VmCachedInstantiation,
        ContractCostType::InvokeVmFunction,
        ContractCostType::ComputeKeccak256Hash,
        ContractCostType::DecodeEcdsaCurve256Sig,
        ContractCostType::RecoverEcdsaSecp256k1Key,
        ContractCostType::Int256AddSub,
        ContractCostType::Int256Mul,
        ContractCostType::Int256Div,
        ContractCostType::Int256Pow,
        ContractCostType::Int256Shift,
        ContractCostType::ChaCha20DrawBytes,
        ContractCostType::ParseWasmInstructions,
        ContractCostType::ParseWasmFunctions,
        ContractCostType::ParseWasmGlobals,
        ContractCostType::ParseWasmTableEntries,
        ContractCostType::ParseWasmTypes,
        ContractCostType::ParseWasmDataSegments,
        ContractCostType::ParseWasmElemSegments,
        ContractCostType::ParseWasmImports,
        ContractCostType::ParseWasmExports,
        ContractCostType::ParseWasmDataSegmentBytes,
        ContractCostType::InstantiateWasmInstructions,
        ContractCostType::InstantiateWasmFunctions,
        ContractCostType::InstantiateWasmGlobals,
        ContractCostType::InstantiateWasmTableEntries,
        ContractCostType::InstantiateWasmTypes,
        ContractCostType::InstantiateWasmDataSegments,
        ContractCostType::InstantiateWasmElemSegments,
        ContractCostType::InstantiateWasmImports,
        ContractCostType::InstantiateWasmExports,
        ContractCostType::InstantiateWasmDataSegmentBytes,
        ContractCostType::Sec1DecodePointUncompressed,
        ContractCostType::VerifyEcdsaSecp256r1Sig,
        ContractCostType::Bls12381EncodeFp,
        ContractCostType::Bls12381DecodeFp,
        ContractCostType::Bls12381G1CheckPointOnCurve,
        ContractCostType::Bls12381G1CheckPointInSubgroup,
        ContractCostType::Bls12381G2CheckPointOnCurve,
        ContractCostType::Bls12381G2CheckPointInSubgroup,
        ContractCostType::Bls12381G1ProjectiveToAffine,
        ContractCostType::Bls12381G2ProjectiveToAffine,
        ContractCostType::Bls12381G1Add,
        ContractCostType::Bls12381G1Mul,
        ContractCostType::Bls12381G1Msm,
        ContractCostType::Bls12381MapFpToG1,
        ContractCostType::Bls12381HashToG1,
        ContractCostType::Bls12381G2Add,
        ContractCostType::Bls12381G2Mul,
        ContractCostType::Bls12381G2Msm,
        ContractCostType::Bls12381MapFp2ToG2,
        ContractCostType::Bls12381HashToG2,
        ContractCostType::Bls12381Pairing,
        ContractCostType::Bls12381FrFromU256,
        ContractCostType::Bls12381FrToU256,
        ContractCostType::Bls12381FrAddSub,
        ContractCostType::Bls12381FrMul,
        ContractCostType::Bls12381FrPow,
        ContractCostType::Bls12381FrInv,
        ContractCostType::Bn254EncodeFp,
        ContractCostType::Bn254DecodeFp,
        ContractCostType::Bn254G1CheckPointOnCurve,
        ContractCostType::Bn254G2CheckPointOnCurve,
        ContractCostType::Bn254G2CheckPointInSubgroup,
        ContractCostType::Bn254G1ProjectiveToAffine,
        ContractCostType::Bn254G1Add,
        ContractCostType::Bn254G1Mul,
        ContractCostType::Bn254Pairing,
        ContractCostType::Bn254FrFromU256,
        ContractCostType::Bn254FrToU256,
        ContractCostType::Bn254FrAddSub,
        ContractCostType::Bn254FrMul,
        ContractCostType::Bn254FrPow,
        ContractCostType::Bn254FrInv,
        ContractCostType::Bn254G1Msm,
    ];
    pub const VARIANTS: [ContractCostType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "WasmInsnExec",
        "MemAlloc",
        "MemCpy",
        "MemCmp",
        "DispatchHostFunction",
        "VisitObject",
        "ValSer",
        "ValDeser",
        "ComputeSha256Hash",
        "ComputeEd25519PubKey",
        "VerifyEd25519Sig",
        "VmInstantiation",
        "VmCachedInstantiation",
        "InvokeVmFunction",
        "ComputeKeccak256Hash",
        "DecodeEcdsaCurve256Sig",
        "RecoverEcdsaSecp256k1Key",
        "Int256AddSub",
        "Int256Mul",
        "Int256Div",
        "Int256Pow",
        "Int256Shift",
        "ChaCha20DrawBytes",
        "ParseWasmInstructions",
        "ParseWasmFunctions",
        "ParseWasmGlobals",
        "ParseWasmTableEntries",
        "ParseWasmTypes",
        "ParseWasmDataSegments",
        "ParseWasmElemSegments",
        "ParseWasmImports",
        "ParseWasmExports",
        "ParseWasmDataSegmentBytes",
        "InstantiateWasmInstructions",
        "InstantiateWasmFunctions",
        "InstantiateWasmGlobals",
        "InstantiateWasmTableEntries",
        "InstantiateWasmTypes",
        "InstantiateWasmDataSegments",
        "InstantiateWasmElemSegments",
        "InstantiateWasmImports",
        "InstantiateWasmExports",
        "InstantiateWasmDataSegmentBytes",
        "Sec1DecodePointUncompressed",
        "VerifyEcdsaSecp256r1Sig",
        "Bls12381EncodeFp",
        "Bls12381DecodeFp",
        "Bls12381G1CheckPointOnCurve",
        "Bls12381G1CheckPointInSubgroup",
        "Bls12381G2CheckPointOnCurve",
        "Bls12381G2CheckPointInSubgroup",
        "Bls12381G1ProjectiveToAffine",
        "Bls12381G2ProjectiveToAffine",
        "Bls12381G1Add",
        "Bls12381G1Mul",
        "Bls12381G1Msm",
        "Bls12381MapFpToG1",
        "Bls12381HashToG1",
        "Bls12381G2Add",
        "Bls12381G2Mul",
        "Bls12381G2Msm",
        "Bls12381MapFp2ToG2",
        "Bls12381HashToG2",
        "Bls12381Pairing",
        "Bls12381FrFromU256",
        "Bls12381FrToU256",
        "Bls12381FrAddSub",
        "Bls12381FrMul",
        "Bls12381FrPow",
        "Bls12381FrInv",
        "Bn254EncodeFp",
        "Bn254DecodeFp",
        "Bn254G1CheckPointOnCurve",
        "Bn254G2CheckPointOnCurve",
        "Bn254G2CheckPointInSubgroup",
        "Bn254G1ProjectiveToAffine",
        "Bn254G1Add",
        "Bn254G1Mul",
        "Bn254Pairing",
        "Bn254FrFromU256",
        "Bn254FrToU256",
        "Bn254FrAddSub",
        "Bn254FrMul",
        "Bn254FrPow",
        "Bn254FrInv",
        "Bn254G1Msm",
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
            Self::WasmInsnExec => "WasmInsnExec",
            Self::MemAlloc => "MemAlloc",
            Self::MemCpy => "MemCpy",
            Self::MemCmp => "MemCmp",
            Self::DispatchHostFunction => "DispatchHostFunction",
            Self::VisitObject => "VisitObject",
            Self::ValSer => "ValSer",
            Self::ValDeser => "ValDeser",
            Self::ComputeSha256Hash => "ComputeSha256Hash",
            Self::ComputeEd25519PubKey => "ComputeEd25519PubKey",
            Self::VerifyEd25519Sig => "VerifyEd25519Sig",
            Self::VmInstantiation => "VmInstantiation",
            Self::VmCachedInstantiation => "VmCachedInstantiation",
            Self::InvokeVmFunction => "InvokeVmFunction",
            Self::ComputeKeccak256Hash => "ComputeKeccak256Hash",
            Self::DecodeEcdsaCurve256Sig => "DecodeEcdsaCurve256Sig",
            Self::RecoverEcdsaSecp256k1Key => "RecoverEcdsaSecp256k1Key",
            Self::Int256AddSub => "Int256AddSub",
            Self::Int256Mul => "Int256Mul",
            Self::Int256Div => "Int256Div",
            Self::Int256Pow => "Int256Pow",
            Self::Int256Shift => "Int256Shift",
            Self::ChaCha20DrawBytes => "ChaCha20DrawBytes",
            Self::ParseWasmInstructions => "ParseWasmInstructions",
            Self::ParseWasmFunctions => "ParseWasmFunctions",
            Self::ParseWasmGlobals => "ParseWasmGlobals",
            Self::ParseWasmTableEntries => "ParseWasmTableEntries",
            Self::ParseWasmTypes => "ParseWasmTypes",
            Self::ParseWasmDataSegments => "ParseWasmDataSegments",
            Self::ParseWasmElemSegments => "ParseWasmElemSegments",
            Self::ParseWasmImports => "ParseWasmImports",
            Self::ParseWasmExports => "ParseWasmExports",
            Self::ParseWasmDataSegmentBytes => "ParseWasmDataSegmentBytes",
            Self::InstantiateWasmInstructions => "InstantiateWasmInstructions",
            Self::InstantiateWasmFunctions => "InstantiateWasmFunctions",
            Self::InstantiateWasmGlobals => "InstantiateWasmGlobals",
            Self::InstantiateWasmTableEntries => "InstantiateWasmTableEntries",
            Self::InstantiateWasmTypes => "InstantiateWasmTypes",
            Self::InstantiateWasmDataSegments => "InstantiateWasmDataSegments",
            Self::InstantiateWasmElemSegments => "InstantiateWasmElemSegments",
            Self::InstantiateWasmImports => "InstantiateWasmImports",
            Self::InstantiateWasmExports => "InstantiateWasmExports",
            Self::InstantiateWasmDataSegmentBytes => "InstantiateWasmDataSegmentBytes",
            Self::Sec1DecodePointUncompressed => "Sec1DecodePointUncompressed",
            Self::VerifyEcdsaSecp256r1Sig => "VerifyEcdsaSecp256r1Sig",
            Self::Bls12381EncodeFp => "Bls12381EncodeFp",
            Self::Bls12381DecodeFp => "Bls12381DecodeFp",
            Self::Bls12381G1CheckPointOnCurve => "Bls12381G1CheckPointOnCurve",
            Self::Bls12381G1CheckPointInSubgroup => "Bls12381G1CheckPointInSubgroup",
            Self::Bls12381G2CheckPointOnCurve => "Bls12381G2CheckPointOnCurve",
            Self::Bls12381G2CheckPointInSubgroup => "Bls12381G2CheckPointInSubgroup",
            Self::Bls12381G1ProjectiveToAffine => "Bls12381G1ProjectiveToAffine",
            Self::Bls12381G2ProjectiveToAffine => "Bls12381G2ProjectiveToAffine",
            Self::Bls12381G1Add => "Bls12381G1Add",
            Self::Bls12381G1Mul => "Bls12381G1Mul",
            Self::Bls12381G1Msm => "Bls12381G1Msm",
            Self::Bls12381MapFpToG1 => "Bls12381MapFpToG1",
            Self::Bls12381HashToG1 => "Bls12381HashToG1",
            Self::Bls12381G2Add => "Bls12381G2Add",
            Self::Bls12381G2Mul => "Bls12381G2Mul",
            Self::Bls12381G2Msm => "Bls12381G2Msm",
            Self::Bls12381MapFp2ToG2 => "Bls12381MapFp2ToG2",
            Self::Bls12381HashToG2 => "Bls12381HashToG2",
            Self::Bls12381Pairing => "Bls12381Pairing",
            Self::Bls12381FrFromU256 => "Bls12381FrFromU256",
            Self::Bls12381FrToU256 => "Bls12381FrToU256",
            Self::Bls12381FrAddSub => "Bls12381FrAddSub",
            Self::Bls12381FrMul => "Bls12381FrMul",
            Self::Bls12381FrPow => "Bls12381FrPow",
            Self::Bls12381FrInv => "Bls12381FrInv",
            Self::Bn254EncodeFp => "Bn254EncodeFp",
            Self::Bn254DecodeFp => "Bn254DecodeFp",
            Self::Bn254G1CheckPointOnCurve => "Bn254G1CheckPointOnCurve",
            Self::Bn254G2CheckPointOnCurve => "Bn254G2CheckPointOnCurve",
            Self::Bn254G2CheckPointInSubgroup => "Bn254G2CheckPointInSubgroup",
            Self::Bn254G1ProjectiveToAffine => "Bn254G1ProjectiveToAffine",
            Self::Bn254G1Add => "Bn254G1Add",
            Self::Bn254G1Mul => "Bn254G1Mul",
            Self::Bn254Pairing => "Bn254Pairing",
            Self::Bn254FrFromU256 => "Bn254FrFromU256",
            Self::Bn254FrToU256 => "Bn254FrToU256",
            Self::Bn254FrAddSub => "Bn254FrAddSub",
            Self::Bn254FrMul => "Bn254FrMul",
            Self::Bn254FrPow => "Bn254FrPow",
            Self::Bn254FrInv => "Bn254FrInv",
            Self::Bn254G1Msm => "Bn254G1Msm",
        }
    }

    #[must_use]
    pub const fn variants() -> [ContractCostType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ContractCostType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ContractCostType> for ContractCostType {
    fn variants() -> slice::Iter<'static, ContractCostType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ContractCostType {}

impl fmt::Display for ContractCostType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ContractCostType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ContractCostType::WasmInsnExec,
            1 => ContractCostType::MemAlloc,
            2 => ContractCostType::MemCpy,
            3 => ContractCostType::MemCmp,
            4 => ContractCostType::DispatchHostFunction,
            5 => ContractCostType::VisitObject,
            6 => ContractCostType::ValSer,
            7 => ContractCostType::ValDeser,
            8 => ContractCostType::ComputeSha256Hash,
            9 => ContractCostType::ComputeEd25519PubKey,
            10 => ContractCostType::VerifyEd25519Sig,
            11 => ContractCostType::VmInstantiation,
            12 => ContractCostType::VmCachedInstantiation,
            13 => ContractCostType::InvokeVmFunction,
            14 => ContractCostType::ComputeKeccak256Hash,
            15 => ContractCostType::DecodeEcdsaCurve256Sig,
            16 => ContractCostType::RecoverEcdsaSecp256k1Key,
            17 => ContractCostType::Int256AddSub,
            18 => ContractCostType::Int256Mul,
            19 => ContractCostType::Int256Div,
            20 => ContractCostType::Int256Pow,
            21 => ContractCostType::Int256Shift,
            22 => ContractCostType::ChaCha20DrawBytes,
            23 => ContractCostType::ParseWasmInstructions,
            24 => ContractCostType::ParseWasmFunctions,
            25 => ContractCostType::ParseWasmGlobals,
            26 => ContractCostType::ParseWasmTableEntries,
            27 => ContractCostType::ParseWasmTypes,
            28 => ContractCostType::ParseWasmDataSegments,
            29 => ContractCostType::ParseWasmElemSegments,
            30 => ContractCostType::ParseWasmImports,
            31 => ContractCostType::ParseWasmExports,
            32 => ContractCostType::ParseWasmDataSegmentBytes,
            33 => ContractCostType::InstantiateWasmInstructions,
            34 => ContractCostType::InstantiateWasmFunctions,
            35 => ContractCostType::InstantiateWasmGlobals,
            36 => ContractCostType::InstantiateWasmTableEntries,
            37 => ContractCostType::InstantiateWasmTypes,
            38 => ContractCostType::InstantiateWasmDataSegments,
            39 => ContractCostType::InstantiateWasmElemSegments,
            40 => ContractCostType::InstantiateWasmImports,
            41 => ContractCostType::InstantiateWasmExports,
            42 => ContractCostType::InstantiateWasmDataSegmentBytes,
            43 => ContractCostType::Sec1DecodePointUncompressed,
            44 => ContractCostType::VerifyEcdsaSecp256r1Sig,
            45 => ContractCostType::Bls12381EncodeFp,
            46 => ContractCostType::Bls12381DecodeFp,
            47 => ContractCostType::Bls12381G1CheckPointOnCurve,
            48 => ContractCostType::Bls12381G1CheckPointInSubgroup,
            49 => ContractCostType::Bls12381G2CheckPointOnCurve,
            50 => ContractCostType::Bls12381G2CheckPointInSubgroup,
            51 => ContractCostType::Bls12381G1ProjectiveToAffine,
            52 => ContractCostType::Bls12381G2ProjectiveToAffine,
            53 => ContractCostType::Bls12381G1Add,
            54 => ContractCostType::Bls12381G1Mul,
            55 => ContractCostType::Bls12381G1Msm,
            56 => ContractCostType::Bls12381MapFpToG1,
            57 => ContractCostType::Bls12381HashToG1,
            58 => ContractCostType::Bls12381G2Add,
            59 => ContractCostType::Bls12381G2Mul,
            60 => ContractCostType::Bls12381G2Msm,
            61 => ContractCostType::Bls12381MapFp2ToG2,
            62 => ContractCostType::Bls12381HashToG2,
            63 => ContractCostType::Bls12381Pairing,
            64 => ContractCostType::Bls12381FrFromU256,
            65 => ContractCostType::Bls12381FrToU256,
            66 => ContractCostType::Bls12381FrAddSub,
            67 => ContractCostType::Bls12381FrMul,
            68 => ContractCostType::Bls12381FrPow,
            69 => ContractCostType::Bls12381FrInv,
            70 => ContractCostType::Bn254EncodeFp,
            71 => ContractCostType::Bn254DecodeFp,
            72 => ContractCostType::Bn254G1CheckPointOnCurve,
            73 => ContractCostType::Bn254G2CheckPointOnCurve,
            74 => ContractCostType::Bn254G2CheckPointInSubgroup,
            75 => ContractCostType::Bn254G1ProjectiveToAffine,
            76 => ContractCostType::Bn254G1Add,
            77 => ContractCostType::Bn254G1Mul,
            78 => ContractCostType::Bn254Pairing,
            79 => ContractCostType::Bn254FrFromU256,
            80 => ContractCostType::Bn254FrToU256,
            81 => ContractCostType::Bn254FrAddSub,
            82 => ContractCostType::Bn254FrMul,
            83 => ContractCostType::Bn254FrPow,
            84 => ContractCostType::Bn254FrInv,
            85 => ContractCostType::Bn254G1Msm,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ContractCostType> for i32 {
    #[must_use]
    fn from(e: ContractCostType) -> Self {
        e as Self
    }
}

impl ReadXdr for ContractCostType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ContractCostType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
