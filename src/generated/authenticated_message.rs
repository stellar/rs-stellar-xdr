#[allow(unused_imports)]
use super::*;
/// AuthenticatedMessage is an XDR Union defined as:
///
/// ```text
/// union AuthenticatedMessage switch (uint32 v)
/// {
/// case 0:
///     struct
///     {
///         uint64 sequence;
///         StellarMessage message;
///         HmacSha256Mac mac;
///     } v0;
/// };
/// ```
///
// union with discriminant u32
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
pub enum AuthenticatedMessage {
    V0(AuthenticatedMessageV0),
}

#[cfg(feature = "alloc")]
impl Default for AuthenticatedMessage {
    fn default() -> Self {
        Self::V0(AuthenticatedMessageV0::default())
    }
}

impl AuthenticatedMessage {
    const _VARIANTS: &[u32] = &[0];
    pub const VARIANTS: [u32; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["V0"];
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
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> u32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
        }
    }

    #[must_use]
    pub const fn variants() -> [u32; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AuthenticatedMessage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<u32> for AuthenticatedMessage {
    #[must_use]
    fn discriminant(&self) -> u32 {
        Self::discriminant(self)
    }
}

impl Variants<u32> for AuthenticatedMessage {
    fn variants() -> slice::Iter<'static, u32> {
        Self::VARIANTS.iter()
    }
}

impl Union<u32> for AuthenticatedMessage {}

impl ReadXdr for AuthenticatedMessage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: u32 = <u32 as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                0 => Self::V0(AuthenticatedMessageV0::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for AuthenticatedMessage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::V0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
