#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionMeta is an XDR Union defined as:
///
/// ```text
/// union TransactionMeta switch (int v)
/// {
/// case 0:
///     OperationMeta operations<>;
/// case 1:
///     TransactionMetaV1 v1;
/// case 2:
///     TransactionMetaV2 v2;
/// case 3:
///     TransactionMetaV3 v3;
/// case 4:
///     TransactionMetaV4 v4;
/// };
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
pub enum TransactionMeta {
    V0(VecM<OperationMeta>),
    V1(TransactionMetaV1),
    V2(TransactionMetaV2),
    V3(TransactionMetaV3),
    V4(TransactionMetaV4),
}

#[cfg(feature = "alloc")]
impl Default for TransactionMeta {
    fn default() -> Self {
        Self::V0(VecM::<OperationMeta>::default())
    }
}

impl TransactionMeta {
    const _VARIANTS: &[i32] = &[0, 1, 2, 3, 4];
    pub const VARIANTS: [i32; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["V0", "V1", "V2", "V3", "V4"];
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
            Self::V0(_) => "V0",
            Self::V1(_) => "V1",
            Self::V2(_) => "V2",
            Self::V3(_) => "V3",
            Self::V4(_) => "V4",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
            Self::V1(_) => 1,
            Self::V2(_) => 2,
            Self::V3(_) => 3,
            Self::V4(_) => 4,
        }
    }

    #[must_use]
    pub const fn variants() -> [i32; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TransactionMeta {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<i32> for TransactionMeta {
    #[must_use]
    fn discriminant(&self) -> i32 {
        Self::discriminant(self)
    }
}

impl Variants<i32> for TransactionMeta {
    fn variants() -> slice::Iter<'static, i32> {
        Self::VARIANTS.iter()
    }
}

impl Union<i32> for TransactionMeta {}

impl ReadXdr for TransactionMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                0 => Self::V0(VecM::<OperationMeta>::read_xdr(r)?),
                1 => Self::V1(TransactionMetaV1::read_xdr(r)?),
                2 => Self::V2(TransactionMetaV2::read_xdr(r)?),
                3 => Self::V3(TransactionMetaV3::read_xdr(r)?),
                4 => Self::V4(TransactionMetaV4::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for TransactionMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::V0(v) => v.write_xdr(w)?,
                Self::V1(v) => v.write_xdr(w)?,
                Self::V2(v) => v.write_xdr(w)?,
                Self::V3(v) => v.write_xdr(w)?,
                Self::V4(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
