#[allow(unused_imports)]
use super::*;
/// CreateAccountResult is an XDR Union defined as:
///
/// ```text
/// union CreateAccountResult switch (CreateAccountResultCode code)
/// {
/// case CREATE_ACCOUNT_SUCCESS:
///     void;
/// case CREATE_ACCOUNT_MALFORMED:
/// case CREATE_ACCOUNT_UNDERFUNDED:
/// case CREATE_ACCOUNT_LOW_RESERVE:
/// case CREATE_ACCOUNT_ALREADY_EXIST:
///     void;
/// };
/// ```
///
// union with discriminant CreateAccountResultCode
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
pub enum CreateAccountResult {
    Success,
    Malformed,
    Underfunded,
    LowReserve,
    AlreadyExist,
}

#[cfg(feature = "alloc")]
impl Default for CreateAccountResult {
    fn default() -> Self {
        Self::Success
    }
}

impl CreateAccountResult {
    const _VARIANTS: &[CreateAccountResultCode] = &[
        CreateAccountResultCode::Success,
        CreateAccountResultCode::Malformed,
        CreateAccountResultCode::Underfunded,
        CreateAccountResultCode::LowReserve,
        CreateAccountResultCode::AlreadyExist,
    ];
    pub const VARIANTS: [CreateAccountResultCode; Self::_VARIANTS.len()] = {
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
        "Underfunded",
        "LowReserve",
        "AlreadyExist",
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
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::LowReserve => "LowReserve",
            Self::AlreadyExist => "AlreadyExist",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> CreateAccountResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => CreateAccountResultCode::Success,
            Self::Malformed => CreateAccountResultCode::Malformed,
            Self::Underfunded => CreateAccountResultCode::Underfunded,
            Self::LowReserve => CreateAccountResultCode::LowReserve,
            Self::AlreadyExist => CreateAccountResultCode::AlreadyExist,
        }
    }

    #[must_use]
    pub const fn variants() -> [CreateAccountResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CreateAccountResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<CreateAccountResultCode> for CreateAccountResult {
    #[must_use]
    fn discriminant(&self) -> CreateAccountResultCode {
        Self::discriminant(self)
    }
}

impl Variants<CreateAccountResultCode> for CreateAccountResult {
    fn variants() -> slice::Iter<'static, CreateAccountResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<CreateAccountResultCode> for CreateAccountResult {}

impl ReadXdr for CreateAccountResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: CreateAccountResultCode = <CreateAccountResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                CreateAccountResultCode::Success => Self::Success,
                CreateAccountResultCode::Malformed => Self::Malformed,
                CreateAccountResultCode::Underfunded => Self::Underfunded,
                CreateAccountResultCode::LowReserve => Self::LowReserve,
                CreateAccountResultCode::AlreadyExist => Self::AlreadyExist,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for CreateAccountResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::AlreadyExist => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
