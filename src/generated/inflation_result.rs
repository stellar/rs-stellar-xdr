#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// InflationResult is an XDR Union defined as:
///
/// ```text
/// union InflationResult switch (InflationResultCode code)
/// {
/// case INFLATION_SUCCESS:
///     InflationPayout payouts<>;
/// case INFLATION_NOT_TIME:
///     void;
/// };
/// ```
///
// union with discriminant InflationResultCode
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
pub enum InflationResult {
    Success(VecM<InflationPayout>),
    NotTime,
}

#[cfg(feature = "alloc")]
impl Default for InflationResult {
    fn default() -> Self {
        Self::Success(VecM::<InflationPayout>::default())
    }
}

impl InflationResult {
    const _VARIANTS: &[InflationResultCode] =
        &[InflationResultCode::Success, InflationResultCode::NotTime];
    pub const VARIANTS: [InflationResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "NotTime"];
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
            Self::Success(_) => "Success",
            Self::NotTime => "NotTime",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> InflationResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => InflationResultCode::Success,
            Self::NotTime => InflationResultCode::NotTime,
        }
    }

    #[must_use]
    pub const fn variants() -> [InflationResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for InflationResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<InflationResultCode> for InflationResult {
    #[must_use]
    fn discriminant(&self) -> InflationResultCode {
        Self::discriminant(self)
    }
}

impl Variants<InflationResultCode> for InflationResult {
    fn variants() -> slice::Iter<'static, InflationResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<InflationResultCode> for InflationResult {}

impl ReadXdr for InflationResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: InflationResultCode = <InflationResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                InflationResultCode::Success => {
                    Self::Success(VecM::<InflationPayout>::read_xdr(r)?)
                }
                InflationResultCode::NotTime => Self::NotTime,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for InflationResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::NotTime => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// InflationResultConst is a borrowing equivalent of [`InflationResult`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum InflationResultConst {
    Success(VecMConst<InflationPayout>),
    NotTime,
}

#[cfg(feature = "const")]
impl InflationResultConst {
    #[must_use]
    pub const fn discriminant(&self) -> InflationResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => InflationResultCode::Success,
            Self::NotTime => InflationResultCode::NotTime,
        }
    }
}

#[cfg(feature = "const")]
impl InflationResultConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_inflation_result(self);
        w.len()
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
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_inflation_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`InflationResult`], mirroring `<InflationResult as WriteXdr>::write_xdr`.
    pub const fn write_type_inflation_result(&mut self, v: &InflationResultConst) {
        let d = v.discriminant();
        self.write_type_inflation_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            InflationResultConst::Success(value) => {
                self.write_type_vec_inflation_payout(value);
            }
            InflationResultConst::NotTime => {}
        }
    }
}
