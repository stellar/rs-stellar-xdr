#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpStatementPledges is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (SCPStatementType type)
///     {
///     case SCP_ST_PREPARE:
///         struct
///         {
///             Hash quorumSetHash;       // D
///             SCPBallot ballot;         // b
///             SCPBallot* prepared;      // p
///             SCPBallot* preparedPrime; // p'
///             uint32 nC;                // c.n
///             uint32 nH;                // h.n
///         } prepare;
///     case SCP_ST_CONFIRM:
///         struct
///         {
///             SCPBallot ballot;   // b
///             uint32 nPrepared;   // p.n
///             uint32 nCommit;     // c.n
///             uint32 nH;          // h.n
///             Hash quorumSetHash; // D
///         } confirm;
///     case SCP_ST_EXTERNALIZE:
///         struct
///         {
///             SCPBallot commit;         // c
///             uint32 nH;                // h.n
///             Hash commitQuorumSetHash; // D used before EXTERNALIZE
///         } externalize;
///     case SCP_ST_NOMINATE:
///         SCPNomination nominate;
///     }
/// ```
///
// union with discriminant ScpStatementType
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
pub enum ScpStatementPledges {
    Prepare(ScpStatementPrepare),
    Confirm(ScpStatementConfirm),
    Externalize(ScpStatementExternalize),
    Nominate(ScpNomination),
}

#[cfg(feature = "alloc")]
impl Default for ScpStatementPledges {
    fn default() -> Self {
        Self::Prepare(ScpStatementPrepare::default())
    }
}

impl ScpStatementPledges {
    const _VARIANTS: &[ScpStatementType] = &[
        ScpStatementType::Prepare,
        ScpStatementType::Confirm,
        ScpStatementType::Externalize,
        ScpStatementType::Nominate,
    ];
    pub const VARIANTS: [ScpStatementType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Prepare", "Confirm", "Externalize", "Nominate"];
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
            Self::Prepare(_) => "Prepare",
            Self::Confirm(_) => "Confirm",
            Self::Externalize(_) => "Externalize",
            Self::Nominate(_) => "Nominate",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScpStatementType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Prepare(_) => ScpStatementType::Prepare,
            Self::Confirm(_) => ScpStatementType::Confirm,
            Self::Externalize(_) => ScpStatementType::Externalize,
            Self::Nominate(_) => ScpStatementType::Nominate,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScpStatementType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScpStatementPledges {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScpStatementType> for ScpStatementPledges {
    #[must_use]
    fn discriminant(&self) -> ScpStatementType {
        Self::discriminant(self)
    }
}

impl Variants<ScpStatementType> for ScpStatementPledges {
    fn variants() -> slice::Iter<'static, ScpStatementType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScpStatementType> for ScpStatementPledges {}

impl ReadXdr for ScpStatementPledges {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScpStatementType = <ScpStatementType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScpStatementType::Prepare => Self::Prepare(ScpStatementPrepare::read_xdr(r)?),
                ScpStatementType::Confirm => Self::Confirm(ScpStatementConfirm::read_xdr(r)?),
                ScpStatementType::Externalize => {
                    Self::Externalize(ScpStatementExternalize::read_xdr(r)?)
                }
                ScpStatementType::Nominate => Self::Nominate(ScpNomination::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl ScpStatementPledges {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Prepare(v) => {
                v.const_write_xdr(w);
            }
            Self::Confirm(v) => {
                v.const_write_xdr(w);
            }
            Self::Externalize(v) => {
                v.const_write_xdr(w);
            }
            Self::Nominate(v) => {
                v.const_write_xdr(w);
            }
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

impl WriteXdr for ScpStatementPledges {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Prepare(v) => v.write_xdr(w)?,
                Self::Confirm(v) => v.write_xdr(w)?,
                Self::Externalize(v) => v.write_xdr(w)?,
                Self::Nominate(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
