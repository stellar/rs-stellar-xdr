#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// BucketMetadataExt is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         BucketListType bucketListType;
///     }
/// ```
///
// union with discriminant i32
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
pub enum BucketMetadataExt {
    V0,
    V1(BucketListType),
}

#[cfg(feature = "alloc")]
impl Default for BucketMetadataExt {
    fn default() -> Self {
        Self::V0
    }
}

impl BucketMetadataExt {
    const _VARIANTS: &[i32] = &[0, 1];
    pub const VARIANTS: [i32; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["V0", "V1"];
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
            Self::V0 => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }

    #[must_use]
    pub const fn variants() -> [i32; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BucketMetadataExt {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<i32> for BucketMetadataExt {
    #[must_use]
    fn discriminant(&self) -> i32 {
        Self::discriminant(self)
    }
}

impl Variants<i32> for BucketMetadataExt {
    fn variants() -> slice::Iter<'static, i32> {
        Self::VARIANTS.iter()
    }
}

impl Union<i32> for BucketMetadataExt {}

impl ReadXdr for BucketMetadataExt {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                0 => Self::V0,
                1 => Self::V1(BucketListType::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for BucketMetadataExt {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::V0 => ().write_xdr(w)?,
                Self::V1(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
