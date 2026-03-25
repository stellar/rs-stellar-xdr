#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// BeginSponsoringFutureReservesResult is an XDR Union defined as:
///
/// ```text
/// union BeginSponsoringFutureReservesResult switch (
///     BeginSponsoringFutureReservesResultCode code)
/// {
/// case BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS:
///     void;
/// case BEGIN_SPONSORING_FUTURE_RESERVES_MALFORMED:
/// case BEGIN_SPONSORING_FUTURE_RESERVES_ALREADY_SPONSORED:
/// case BEGIN_SPONSORING_FUTURE_RESERVES_RECURSIVE:
///     void;
/// };
/// ```
///
// union with discriminant BeginSponsoringFutureReservesResultCode
#[cfg_eval::cfg_eval]
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
pub enum BeginSponsoringFutureReservesResult {
    Success,
    Malformed,
    AlreadySponsored,
    Recursive,
}

#[cfg(feature = "alloc")]
impl Default for BeginSponsoringFutureReservesResult {
    fn default() -> Self {
        Self::Success
    }
}

impl BeginSponsoringFutureReservesResult {
    const _VARIANTS: &[BeginSponsoringFutureReservesResultCode] = &[
        BeginSponsoringFutureReservesResultCode::Success,
        BeginSponsoringFutureReservesResultCode::Malformed,
        BeginSponsoringFutureReservesResultCode::AlreadySponsored,
        BeginSponsoringFutureReservesResultCode::Recursive,
    ];
    pub const VARIANTS: [BeginSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "Malformed", "AlreadySponsored", "Recursive"];
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
            Self::Malformed => "Malformed",
            Self::AlreadySponsored => "AlreadySponsored",
            Self::Recursive => "Recursive",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> BeginSponsoringFutureReservesResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => BeginSponsoringFutureReservesResultCode::Success,
            Self::Malformed => BeginSponsoringFutureReservesResultCode::Malformed,
            Self::AlreadySponsored => BeginSponsoringFutureReservesResultCode::AlreadySponsored,
            Self::Recursive => BeginSponsoringFutureReservesResultCode::Recursive,
        }
    }

    #[must_use]
    pub const fn variants() -> [BeginSponsoringFutureReservesResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BeginSponsoringFutureReservesResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<BeginSponsoringFutureReservesResultCode> for BeginSponsoringFutureReservesResult {
    #[must_use]
    fn discriminant(&self) -> BeginSponsoringFutureReservesResultCode {
        Self::discriminant(self)
    }
}

impl Variants<BeginSponsoringFutureReservesResultCode> for BeginSponsoringFutureReservesResult {
    fn variants() -> slice::Iter<'static, BeginSponsoringFutureReservesResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<BeginSponsoringFutureReservesResultCode> for BeginSponsoringFutureReservesResult {}

impl ReadXdr for BeginSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: BeginSponsoringFutureReservesResultCode =
                <BeginSponsoringFutureReservesResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                BeginSponsoringFutureReservesResultCode::Success => Self::Success,
                BeginSponsoringFutureReservesResultCode::Malformed => Self::Malformed,
                BeginSponsoringFutureReservesResultCode::AlreadySponsored => Self::AlreadySponsored,
                BeginSponsoringFutureReservesResultCode::Recursive => Self::Recursive,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for BeginSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::AlreadySponsored => ().write_xdr(w)?,
                Self::Recursive => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
