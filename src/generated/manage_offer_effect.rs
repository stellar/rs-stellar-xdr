#[allow(unused_imports)]
use super::*;
/// ManageOfferEffect is an XDR Enum defined as:
///
/// ```text
/// enum ManageOfferEffect
/// {
///     MANAGE_OFFER_CREATED = 0,
///     MANAGE_OFFER_UPDATED = 1,
///     MANAGE_OFFER_DELETED = 2
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
pub enum ManageOfferEffect {
    #[cfg_attr(feature = "alloc", default)]
    Created = 0,
    Updated = 1,
    Deleted = 2,
}

impl ManageOfferEffect {
    const _VARIANTS: &[ManageOfferEffect] = &[
        ManageOfferEffect::Created,
        ManageOfferEffect::Updated,
        ManageOfferEffect::Deleted,
    ];
    pub const VARIANTS: [ManageOfferEffect; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Created", "Updated", "Deleted"];
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
            Self::Created => "Created",
            Self::Updated => "Updated",
            Self::Deleted => "Deleted",
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageOfferEffect; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageOfferEffect {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ManageOfferEffect> for ManageOfferEffect {
    fn variants() -> slice::Iter<'static, ManageOfferEffect> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ManageOfferEffect {}

impl fmt::Display for ManageOfferEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageOfferEffect {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ManageOfferEffect::Created,
            1 => ManageOfferEffect::Updated,
            2 => ManageOfferEffect::Deleted,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageOfferEffect> for i32 {
    #[must_use]
    fn from(e: ManageOfferEffect) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageOfferEffect {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ManageOfferEffect {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
