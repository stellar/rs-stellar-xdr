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

impl WriteXdr for ScVec {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
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

/// ScVecConst is a borrowing equivalent of [`ScVec`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScVecConst(pub VecMConst<ScValConst>);

#[cfg(feature = "const")]
impl ScVecConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_vec(self);
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
        w.write_type_sc_vec(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScVec`], mirroring `<ScVec as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_vec(&mut self, v: &ScVecConst) {
        self.write_type_vec_sc_val(&v.0);
    }

    /// Serializes an optional [`ScVec`], mirroring `<Option<ScVec> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_sc_vec(&mut self, v: &Option<ScVecConst>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_sc_vec(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
