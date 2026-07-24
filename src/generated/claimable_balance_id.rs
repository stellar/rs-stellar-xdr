#[allow(unused_imports, clippy::wildcard_imports)]
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

impl ClaimableBalanceId {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceIdTypeV0(v) => {
                v.const_write_xdr(w);
            }
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
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
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
