#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PersistedScpStateV0 is an XDR Struct defined as:
///
/// ```text
/// struct PersistedSCPStateV0
/// {
/// 	SCPEnvelope scpEnvelopes<>;
/// 	SCPQuorumSet quorumSets<>;
/// 	StoredTransactionSet txSets<>;
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
pub struct PersistedScpStateV0 {
    pub scp_envelopes: VecM<ScpEnvelope>,
    pub quorum_sets: VecM<ScpQuorumSet>,
    pub tx_sets: VecM<StoredTransactionSet>,
}

impl ReadXdr for PersistedScpStateV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                scp_envelopes: VecM::<ScpEnvelope>::read_xdr(r)?,
                quorum_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
                tx_sets: VecM::<StoredTransactionSet>::read_xdr(r)?,
            })
        })
    }
}

impl PersistedScpStateV0 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.scp_envelopes.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            let __s0 = self.quorum_sets.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            let __s0 = self.tx_sets.0.as_slice();
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

impl WriteXdr for PersistedScpStateV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.scp_envelopes.write_xdr(w)?;
            self.quorum_sets.write_xdr(w)?;
            self.tx_sets.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
