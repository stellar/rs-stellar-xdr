#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// RevokeSponsorshipOp is an XDR Union defined as:
///
/// ```text
/// union RevokeSponsorshipOp switch (RevokeSponsorshipType type)
/// {
/// case REVOKE_SPONSORSHIP_LEDGER_ENTRY:
///     LedgerKey ledgerKey;
/// case REVOKE_SPONSORSHIP_SIGNER:
///     struct
///     {
///         AccountID accountID;
///         SignerKey signerKey;
///     } signer;
/// };
/// ```
///
// union with discriminant RevokeSponsorshipType
#[cfg_eval::cfg_eval]
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
pub enum RevokeSponsorshipOp {
    LedgerEntry(LedgerKey),
    Signer(RevokeSponsorshipOpSigner),
}

#[cfg(feature = "alloc")]
impl Default for RevokeSponsorshipOp {
    fn default() -> Self {
        Self::LedgerEntry(LedgerKey::default())
    }
}

impl RevokeSponsorshipOp {
    const _VARIANTS: &[RevokeSponsorshipType] = &[
        RevokeSponsorshipType::LedgerEntry,
        RevokeSponsorshipType::Signer,
    ];
    pub const VARIANTS: [RevokeSponsorshipType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["LedgerEntry", "Signer"];
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
            Self::LedgerEntry(_) => "LedgerEntry",
            Self::Signer(_) => "Signer",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> RevokeSponsorshipType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LedgerEntry(_) => RevokeSponsorshipType::LedgerEntry,
            Self::Signer(_) => RevokeSponsorshipType::Signer,
        }
    }

    #[must_use]
    pub const fn variants() -> [RevokeSponsorshipType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for RevokeSponsorshipOp {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<RevokeSponsorshipType> for RevokeSponsorshipOp {
    #[must_use]
    fn discriminant(&self) -> RevokeSponsorshipType {
        Self::discriminant(self)
    }
}

impl Variants<RevokeSponsorshipType> for RevokeSponsorshipOp {
    fn variants() -> slice::Iter<'static, RevokeSponsorshipType> {
        Self::VARIANTS.iter()
    }
}

impl Union<RevokeSponsorshipType> for RevokeSponsorshipOp {}

impl ReadXdr for RevokeSponsorshipOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: RevokeSponsorshipType = <RevokeSponsorshipType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                RevokeSponsorshipType::LedgerEntry => Self::LedgerEntry(LedgerKey::read_xdr(r)?),
                RevokeSponsorshipType::Signer => {
                    Self::Signer(RevokeSponsorshipOpSigner::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for RevokeSponsorshipOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::LedgerEntry(v) => v.write_xdr(w)?,
                Self::Signer(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
