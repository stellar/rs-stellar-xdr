#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeTuple is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeTuple
/// {
///     SCSpecTypeDef valueTypes<12>;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ScSpecTypeTuple {
    pub value_types: VecM<ScSpecTypeDef, 12>,
}

impl ReadXdr for ScSpecTypeTuple {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                value_types: VecM::<ScSpecTypeDef, 12>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeTuple {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value_types.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecTypeTupleRef is a borrowing equivalent of [`ScSpecTypeTuple`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeTupleRef<'a> {
    pub value_types: VecMRef<'a, ScSpecTypeDefRef<'a>, 12>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeTupleRef<'_>> for ScSpecTypeTuple {
    #[must_use]
    fn from(v: &ScSpecTypeTupleRef<'_>) -> Self {
        Self {
            value_types: v.value_types.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeTupleRef<'_>> for ScSpecTypeTuple {
    #[must_use]
    fn from(v: ScSpecTypeTupleRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecTypeTupleRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value_types.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(feature = "const")]
impl ScSpecTypeTupleRef<'_> {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to the owned type's
    /// [`WriteXdr::write_xdr`].
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.value_types.as_slice();
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
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
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
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}
