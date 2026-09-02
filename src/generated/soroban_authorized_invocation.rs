#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAuthorizedInvocation is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAuthorizedInvocation
/// {
///     SorobanAuthorizedFunction function;
///     SorobanAuthorizedInvocation subInvocations<>;
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
pub struct SorobanAuthorizedInvocation {
    pub function: SorobanAuthorizedFunction,
    pub sub_invocations: VecM<SorobanAuthorizedInvocation>,
}

impl ReadXdr for SorobanAuthorizedInvocation {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                function: SorobanAuthorizedFunction::read_xdr(r)?,
                sub_invocations: VecM::<SorobanAuthorizedInvocation>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanAuthorizedInvocation {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.function.write_xdr(w)?;
            self.sub_invocations.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanAuthorizedInvocationView is a borrowing equivalent of [`SorobanAuthorizedInvocation`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAuthorizedInvocationView<'a> {
    pub function: SorobanAuthorizedFunctionView<'a>,
    pub sub_invocations: VecMView<'a, SorobanAuthorizedInvocationView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&SorobanAuthorizedInvocationView<'_>> for SorobanAuthorizedInvocation {
    #[must_use]
    fn from(v: &SorobanAuthorizedInvocationView<'_>) -> Self {
        Self {
            function: (&v.function).into(),
            sub_invocations: v.sub_invocations.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAuthorizedInvocationView<'_>> for SorobanAuthorizedInvocation {
    #[must_use]
    fn from(v: SorobanAuthorizedInvocationView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SorobanAuthorizedInvocationView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.function.write_xdr(w)?;
            self.sub_invocations.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SorobanAuthorizedInvocationView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_authorized_invocation(self);
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
        w.write_type_soroban_authorized_invocation(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanAuthorizedInvocation`], mirroring `<SorobanAuthorizedInvocation as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_authorized_invocation(
        &mut self,
        v: &SorobanAuthorizedInvocationView<'_>,
    ) {
        self.write_type_soroban_authorized_function(&v.function);
        self.write_type_vec_soroban_authorized_invocation(&v.sub_invocations);
    }

    /// Serializes a variable-length array of [`SorobanAuthorizedInvocation`], mirroring `<VecM<SorobanAuthorizedInvocation, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_soroban_authorized_invocation<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, SorobanAuthorizedInvocationView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_soroban_authorized_invocation(&s[i]);
            i += 1;
        }
    }
}
