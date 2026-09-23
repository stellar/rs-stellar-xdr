#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ClaimPredicate is a borrowing equivalent of [`ClaimPredicate`](super::super::ClaimPredicate)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum ClaimPredicate {
    Unconditional,
    And(VecM<ClaimPredicate, 2>),
    Or(VecM<ClaimPredicate, 2>),
    Not(
        #[cfg_attr(feature = "arbitrary", arbitrary(with = arbitrary_option_ref::<ClaimPredicate>))]
         Option<&'static ClaimPredicate>,
    ),
    BeforeAbsoluteTime(i64),
    BeforeRelativeTime(i64),
}

impl ClaimPredicate {
    #[must_use]
    pub const fn discriminant(&self) -> ClaimPredicateType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Unconditional => ClaimPredicateType::Unconditional,
            Self::And(_) => ClaimPredicateType::And,
            Self::Or(_) => ClaimPredicateType::Or,
            Self::Not(_) => ClaimPredicateType::Not,
            Self::BeforeAbsoluteTime(_) => ClaimPredicateType::BeforeAbsoluteTime,
            Self::BeforeRelativeTime(_) => ClaimPredicateType::BeforeRelativeTime,
        }
    }
}

impl ClaimPredicate {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_claim_predicate(self);
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
        w.write_type_claim_predicate(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`ClaimPredicate`], mirroring `<ClaimPredicate as WriteXdr>::write_xdr`.
    pub const fn write_type_claim_predicate(&mut self, v: &ClaimPredicate) {
        let d = v.discriminant();
        self.write_type_claim_predicate_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ClaimPredicate::Unconditional => {}
            ClaimPredicate::And(value) => {
                self.write_type_vec_claim_predicate(value);
            }
            ClaimPredicate::Or(value) => {
                self.write_type_vec_claim_predicate(value);
            }
            ClaimPredicate::Not(value) => {
                self.write_type_option_ref_claim_predicate(*value);
            }
            ClaimPredicate::BeforeAbsoluteTime(value) => {
                self.write_i64(*value);
            }
            ClaimPredicate::BeforeRelativeTime(value) => {
                self.write_i64(*value);
            }
        }
    }

    /// Serializes an optional [`ClaimPredicate`], mirroring `<Option<Box<ClaimPredicate>> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_ref_claim_predicate(
        &mut self,
        v: Option<&'static ClaimPredicate>,
    ) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_claim_predicate(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes a variable-length array of [`ClaimPredicate`], mirroring `<VecM<ClaimPredicate, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_claim_predicate<const MAX: u32>(
        &mut self,
        v: &VecM<ClaimPredicate, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_claim_predicate(&s[i]);
            i += 1;
        }
    }
}
