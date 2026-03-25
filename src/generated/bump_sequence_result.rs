#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// BumpSequenceResult is an XDR Union defined as:
///
/// ```text
/// union BumpSequenceResult switch (BumpSequenceResultCode code)
/// {
/// case BUMP_SEQUENCE_SUCCESS:
///     void;
/// case BUMP_SEQUENCE_BAD_SEQ:
///     void;
/// };
/// ```
///
// union with discriminant BumpSequenceResultCode
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
pub enum BumpSequenceResult {
    Success,
    BadSeq,
}

#[cfg(feature = "alloc")]
impl Default for BumpSequenceResult {
    fn default() -> Self {
        Self::Success
    }
}

impl BumpSequenceResult {
    const _VARIANTS: &[BumpSequenceResultCode] = &[
        BumpSequenceResultCode::Success,
        BumpSequenceResultCode::BadSeq,
    ];
    pub const VARIANTS: [BumpSequenceResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "BadSeq"];
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
            Self::BadSeq => "BadSeq",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> BumpSequenceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => BumpSequenceResultCode::Success,
            Self::BadSeq => BumpSequenceResultCode::BadSeq,
        }
    }

    #[must_use]
    pub const fn variants() -> [BumpSequenceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BumpSequenceResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<BumpSequenceResultCode> for BumpSequenceResult {
    #[must_use]
    fn discriminant(&self) -> BumpSequenceResultCode {
        Self::discriminant(self)
    }
}

impl Variants<BumpSequenceResultCode> for BumpSequenceResult {
    fn variants() -> slice::Iter<'static, BumpSequenceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<BumpSequenceResultCode> for BumpSequenceResult {}

impl ReadXdr for BumpSequenceResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: BumpSequenceResultCode = <BumpSequenceResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                BumpSequenceResultCode::Success => Self::Success,
                BumpSequenceResultCode::BadSeq => Self::BadSeq,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for BumpSequenceResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::BadSeq => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
