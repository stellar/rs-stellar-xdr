#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScVec is an XDR Typedef defined as:
///
/// ```text
/// typedef SCVal SCVec<>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct ScVec(pub VecM<ScVal>);

impl From<ScVec> for VecM<ScVal> {
    #[must_use]
    fn from(x: ScVec) -> Self {
        x.0
    }
}

impl From<VecM<ScVal>> for ScVec {
    #[must_use]
    fn from(x: VecM<ScVal>) -> Self {
        ScVec(x)
    }
}

impl AsRef<VecM<ScVal>> for ScVec {
    #[must_use]
    fn as_ref(&self) -> &VecM<ScVal> {
        &self.0
    }
}

impl ReadXdr for ScVec {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<ScVal>::read_xdr(r)?;
            let v = ScVec(i);
            Ok(v)
        })
    }
}

impl ScVec {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.0 .0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for ScVec {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}

impl Deref for ScVec {
    type Target = VecM<ScVal>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScVec> for Vec<ScVal> {
    #[must_use]
    fn from(x: ScVec) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<ScVal>> for ScVec {
    type Error = Error;
    fn try_from(x: Vec<ScVal>) -> Result<Self, Error> {
        Ok(ScVec(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<ScVal>> for ScVec {
    type Error = Error;
    fn try_from(x: &Vec<ScVal>) -> Result<Self, Error> {
        Ok(ScVec(x.try_into()?))
    }
}

impl AsRef<Vec<ScVal>> for ScVec {
    #[must_use]
    fn as_ref(&self) -> &Vec<ScVal> {
        &self.0 .0
    }
}

impl AsRef<[ScVal]> for ScVec {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[ScVal] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[ScVal] {
        self.0 .0
    }
}
