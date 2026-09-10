#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpEnvelope is an XDR Struct defined as:
///
/// ```text
/// struct SCPEnvelope
/// {
///     SCPStatement statement;
///     Signature signature;
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
pub struct ScpEnvelope {
    pub statement: ScpStatement,
    pub signature: Signature,
}

impl ReadXdr for ScpEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                statement: ScpStatement::read_xdr(r)?,
                signature: Signature::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.statement.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpEnvelopeConst is a borrowing equivalent of [`ScpEnvelope`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpEnvelopeConst {
    pub statement: ScpStatementConst,
    pub signature: SignatureConst,
}

#[cfg(feature = "const")]
impl ScpEnvelopeConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_scp_envelope(self);
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
        w.write_type_scp_envelope(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScpEnvelope`], mirroring `<ScpEnvelope as WriteXdr>::write_xdr`.
    pub const fn write_type_scp_envelope(&mut self, v: &ScpEnvelopeConst) {
        self.write_type_scp_statement(&v.statement);
        self.write_type_signature(&v.signature);
    }

    /// Serializes a variable-length array of [`ScpEnvelope`], mirroring `<VecM<ScpEnvelope, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_scp_envelope<const MAX: u32>(
        &mut self,
        v: &VecMConst<ScpEnvelopeConst, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_scp_envelope(&s[i]);
            i += 1;
        }
    }
}
