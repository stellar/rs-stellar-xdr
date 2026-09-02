#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// InvokeHostFunctionResult is an XDR Union defined as:
///
/// ```text
/// union InvokeHostFunctionResult switch (InvokeHostFunctionResultCode code)
/// {
/// case INVOKE_HOST_FUNCTION_SUCCESS:
///     Hash success; // sha256(InvokeHostFunctionSuccessPreImage)
/// case INVOKE_HOST_FUNCTION_MALFORMED:
/// case INVOKE_HOST_FUNCTION_TRAPPED:
/// case INVOKE_HOST_FUNCTION_RESOURCE_LIMIT_EXCEEDED:
/// case INVOKE_HOST_FUNCTION_ENTRY_ARCHIVED:
/// case INVOKE_HOST_FUNCTION_INSUFFICIENT_REFUNDABLE_FEE:
///     void;
/// };
/// ```
///
// union with discriminant InvokeHostFunctionResultCode
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
pub enum InvokeHostFunctionResult {
    Success(Hash),
    Malformed,
    Trapped,
    ResourceLimitExceeded,
    EntryArchived,
    InsufficientRefundableFee,
}

#[cfg(feature = "alloc")]
impl Default for InvokeHostFunctionResult {
    fn default() -> Self {
        Self::Success(Hash::default())
    }
}

impl InvokeHostFunctionResult {
    const _VARIANTS: &[InvokeHostFunctionResultCode] = &[
        InvokeHostFunctionResultCode::Success,
        InvokeHostFunctionResultCode::Malformed,
        InvokeHostFunctionResultCode::Trapped,
        InvokeHostFunctionResultCode::ResourceLimitExceeded,
        InvokeHostFunctionResultCode::EntryArchived,
        InvokeHostFunctionResultCode::InsufficientRefundableFee,
    ];
    pub const VARIANTS: [InvokeHostFunctionResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "Trapped",
        "ResourceLimitExceeded",
        "EntryArchived",
        "InsufficientRefundableFee",
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
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::Trapped => "Trapped",
            Self::ResourceLimitExceeded => "ResourceLimitExceeded",
            Self::EntryArchived => "EntryArchived",
            Self::InsufficientRefundableFee => "InsufficientRefundableFee",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> InvokeHostFunctionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => InvokeHostFunctionResultCode::Success,
            Self::Malformed => InvokeHostFunctionResultCode::Malformed,
            Self::Trapped => InvokeHostFunctionResultCode::Trapped,
            Self::ResourceLimitExceeded => InvokeHostFunctionResultCode::ResourceLimitExceeded,
            Self::EntryArchived => InvokeHostFunctionResultCode::EntryArchived,
            Self::InsufficientRefundableFee => {
                InvokeHostFunctionResultCode::InsufficientRefundableFee
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [InvokeHostFunctionResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for InvokeHostFunctionResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<InvokeHostFunctionResultCode> for InvokeHostFunctionResult {
    #[must_use]
    fn discriminant(&self) -> InvokeHostFunctionResultCode {
        Self::discriminant(self)
    }
}

impl Variants<InvokeHostFunctionResultCode> for InvokeHostFunctionResult {
    fn variants() -> slice::Iter<'static, InvokeHostFunctionResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<InvokeHostFunctionResultCode> for InvokeHostFunctionResult {}

impl ReadXdr for InvokeHostFunctionResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: InvokeHostFunctionResultCode =
                <InvokeHostFunctionResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                InvokeHostFunctionResultCode::Success => Self::Success(Hash::read_xdr(r)?),
                InvokeHostFunctionResultCode::Malformed => Self::Malformed,
                InvokeHostFunctionResultCode::Trapped => Self::Trapped,
                InvokeHostFunctionResultCode::ResourceLimitExceeded => Self::ResourceLimitExceeded,
                InvokeHostFunctionResultCode::EntryArchived => Self::EntryArchived,
                InvokeHostFunctionResultCode::InsufficientRefundableFee => {
                    Self::InsufficientRefundableFee
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for InvokeHostFunctionResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Trapped => ().write_xdr(w)?,
                Self::ResourceLimitExceeded => ().write_xdr(w)?,
                Self::EntryArchived => ().write_xdr(w)?,
                Self::InsufficientRefundableFee => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for InvokeHostFunctionResult {
    type Owned = InvokeHostFunctionResult;
    fn into_owned(self) -> InvokeHostFunctionResult {
        self
    }
}

#[cfg(feature = "const")]
impl InvokeHostFunctionResult {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_invoke_host_function_result(self);
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
        w.write_type_invoke_host_function_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`InvokeHostFunctionResult`], mirroring `<InvokeHostFunctionResult as WriteXdr>::write_xdr`.
    pub const fn write_type_invoke_host_function_result(&mut self, v: &InvokeHostFunctionResult) {
        let d = v.discriminant();
        self.write_type_invoke_host_function_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            InvokeHostFunctionResult::Success(value) => {
                self.write_type_hash(value);
            }
            InvokeHostFunctionResult::Malformed => {}
            InvokeHostFunctionResult::Trapped => {}
            InvokeHostFunctionResult::ResourceLimitExceeded => {}
            InvokeHostFunctionResult::EntryArchived => {}
            InvokeHostFunctionResult::InsufficientRefundableFee => {}
        }
    }
}
