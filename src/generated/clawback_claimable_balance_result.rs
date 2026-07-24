#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ClawbackClaimableBalanceResult is an XDR Union defined as:
///
/// ```text
/// union ClawbackClaimableBalanceResult switch (
///     ClawbackClaimableBalanceResultCode code)
/// {
/// case CLAWBACK_CLAIMABLE_BALANCE_SUCCESS:
///     void;
/// case CLAWBACK_CLAIMABLE_BALANCE_DOES_NOT_EXIST:
/// case CLAWBACK_CLAIMABLE_BALANCE_NOT_ISSUER:
/// case CLAWBACK_CLAIMABLE_BALANCE_NOT_CLAWBACK_ENABLED:
///     void;
/// };
/// ```
///
// union with discriminant ClawbackClaimableBalanceResultCode
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
pub enum ClawbackClaimableBalanceResult {
    Success,
    DoesNotExist,
    NotIssuer,
    NotClawbackEnabled,
}

#[cfg(feature = "alloc")]
impl Default for ClawbackClaimableBalanceResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ClawbackClaimableBalanceResult {
    const _VARIANTS: &[ClawbackClaimableBalanceResultCode] = &[
        ClawbackClaimableBalanceResultCode::Success,
        ClawbackClaimableBalanceResultCode::DoesNotExist,
        ClawbackClaimableBalanceResultCode::NotIssuer,
        ClawbackClaimableBalanceResultCode::NotClawbackEnabled,
    ];
    pub const VARIANTS: [ClawbackClaimableBalanceResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "DoesNotExist", "NotIssuer", "NotClawbackEnabled"];
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
            Self::Success => "Success",
            Self::DoesNotExist => "DoesNotExist",
            Self::NotIssuer => "NotIssuer",
            Self::NotClawbackEnabled => "NotClawbackEnabled",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClawbackClaimableBalanceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ClawbackClaimableBalanceResultCode::Success,
            Self::DoesNotExist => ClawbackClaimableBalanceResultCode::DoesNotExist,
            Self::NotIssuer => ClawbackClaimableBalanceResultCode::NotIssuer,
            Self::NotClawbackEnabled => ClawbackClaimableBalanceResultCode::NotClawbackEnabled,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClawbackClaimableBalanceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClawbackClaimableBalanceResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClawbackClaimableBalanceResultCode> for ClawbackClaimableBalanceResult {
    #[must_use]
    fn discriminant(&self) -> ClawbackClaimableBalanceResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ClawbackClaimableBalanceResultCode> for ClawbackClaimableBalanceResult {
    fn variants() -> slice::Iter<'static, ClawbackClaimableBalanceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClawbackClaimableBalanceResultCode> for ClawbackClaimableBalanceResult {}

impl ReadXdr for ClawbackClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClawbackClaimableBalanceResultCode =
                <ClawbackClaimableBalanceResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClawbackClaimableBalanceResultCode::Success => Self::Success,
                ClawbackClaimableBalanceResultCode::DoesNotExist => Self::DoesNotExist,
                ClawbackClaimableBalanceResultCode::NotIssuer => Self::NotIssuer,
                ClawbackClaimableBalanceResultCode::NotClawbackEnabled => Self::NotClawbackEnabled,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl ClawbackClaimableBalanceResult {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => {}
            Self::DoesNotExist => {}
            Self::NotIssuer => {}
            Self::NotClawbackEnabled => {}
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
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
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for ClawbackClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::DoesNotExist => ().write_xdr(w)?,
                Self::NotIssuer => ().write_xdr(w)?,
                Self::NotClawbackEnabled => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
