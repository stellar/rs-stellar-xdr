#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanDelegateSignature is an XDR Struct defined as:
///
/// ```text
/// struct SorobanDelegateSignature
/// {
///     SCAddress address;
///     SCVal signature;
///     SorobanDelegateSignature nestedDelegates<>;
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
pub struct SorobanDelegateSignature {
    pub address: ScAddress,
    pub signature: ScVal,
    pub nested_delegates: VecM<SorobanDelegateSignature>,
}

impl ReadXdr for SorobanDelegateSignature {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address: ScAddress::read_xdr(r)?,
                signature: ScVal::read_xdr(r)?,
                nested_delegates: VecM::<SorobanDelegateSignature>::read_xdr(r)?,
            })
        })
    }
}

impl SorobanDelegateSignature {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.address.const_write_xdr(w);
        self.signature.const_write_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.nested_delegates.0.as_slice();
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

impl WriteXdr for SorobanDelegateSignature {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            self.nested_delegates.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
