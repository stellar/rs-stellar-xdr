#[allow(unused_imports)]
use super::*;
/// ClaimableBalanceId is an XDR Union defined as:
///
/// ```text
/// union ClaimableBalanceID switch (ClaimableBalanceIDType type)
/// {
/// case CLAIMABLE_BALANCE_ID_TYPE_V0:
///     Hash v0;
/// };
/// ```
///
// union with discriminant ClaimableBalanceIdType
#[cfg_eval::cfg_eval]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimableBalanceId {
    ClaimableBalanceIdTypeV0(Hash),
}

#[cfg(feature = "alloc")]
impl Default for ClaimableBalanceId {
    fn default() -> Self {
        Self::ClaimableBalanceIdTypeV0(Hash::default())
    }
}

impl ClaimableBalanceId {
    const _VARIANTS: &[ClaimableBalanceIdType] =
        &[ClaimableBalanceIdType::ClaimableBalanceIdTypeV0];
    pub const VARIANTS: [ClaimableBalanceIdType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["ClaimableBalanceIdTypeV0"];
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
            Self::ClaimableBalanceIdTypeV0(_) => "ClaimableBalanceIdTypeV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClaimableBalanceIdType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceIdTypeV0(_) => ClaimableBalanceIdType::ClaimableBalanceIdTypeV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimableBalanceIdType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimableBalanceId {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClaimableBalanceIdType> for ClaimableBalanceId {
    #[must_use]
    fn discriminant(&self) -> ClaimableBalanceIdType {
        Self::discriminant(self)
    }
}

impl Variants<ClaimableBalanceIdType> for ClaimableBalanceId {
    fn variants() -> slice::Iter<'static, ClaimableBalanceIdType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClaimableBalanceIdType> for ClaimableBalanceId {}

impl ReadXdr for ClaimableBalanceId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClaimableBalanceIdType = <ClaimableBalanceIdType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClaimableBalanceIdType::ClaimableBalanceIdTypeV0 => {
                    Self::ClaimableBalanceIdTypeV0(Hash::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimableBalanceId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ClaimableBalanceIdTypeV0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
