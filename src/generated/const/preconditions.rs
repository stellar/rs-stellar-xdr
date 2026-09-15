#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Preconditions is a borrowing equivalent of [`Preconditions`](super::super::Preconditions)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum Preconditions {
    None,
    Time(TimeBounds),
    V2(PreconditionsV2),
}

impl Preconditions {
    #[must_use]
    pub const fn discriminant(&self) -> PreconditionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => PreconditionType::None,
            Self::Time(_) => PreconditionType::Time,
            Self::V2(_) => PreconditionType::V2,
        }
    }
}

impl Preconditions {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_preconditions(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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
        w.write_type_preconditions(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`Preconditions`], mirroring `<Preconditions as WriteXdr>::write_xdr`.
    pub const fn write_type_preconditions(&mut self, v: &Preconditions) {
        let d = v.discriminant();
        self.write_type_precondition_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            Preconditions::None => {}
            Preconditions::Time(value) => {
                self.write_type_time_bounds(value);
            }
            Preconditions::V2(value) => {
                self.write_type_preconditions_v2(value);
            }
        }
    }
}
