#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryChange is an XDR Union defined as:
///
/// ```text
/// union LedgerEntryChange switch (LedgerEntryChangeType type)
/// {
/// case LEDGER_ENTRY_CREATED:
///     LedgerEntry created;
/// case LEDGER_ENTRY_UPDATED:
///     LedgerEntry updated;
/// case LEDGER_ENTRY_REMOVED:
///     LedgerKey removed;
/// case LEDGER_ENTRY_STATE:
///     LedgerEntry state;
/// case LEDGER_ENTRY_RESTORED:
///     LedgerEntry restored;
/// };
/// ```
///
// union with discriminant LedgerEntryChangeType
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
pub enum LedgerEntryChange {
    Created(LedgerEntry),
    Updated(LedgerEntry),
    Removed(LedgerKey),
    State(LedgerEntry),
    Restored(LedgerEntry),
}

#[cfg(feature = "alloc")]
impl Default for LedgerEntryChange {
    fn default() -> Self {
        Self::Created(LedgerEntry::default())
    }
}

impl LedgerEntryChange {
    const _VARIANTS: &[LedgerEntryChangeType] = &[
        LedgerEntryChangeType::Created,
        LedgerEntryChangeType::Updated,
        LedgerEntryChangeType::Removed,
        LedgerEntryChangeType::State,
        LedgerEntryChangeType::Restored,
    ];
    pub const VARIANTS: [LedgerEntryChangeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Created", "Updated", "Removed", "State", "Restored"];
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
            Self::Created(_) => "Created",
            Self::Updated(_) => "Updated",
            Self::Removed(_) => "Removed",
            Self::State(_) => "State",
            Self::Restored(_) => "Restored",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> LedgerEntryChangeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => LedgerEntryChangeType::Created,
            Self::Updated(_) => LedgerEntryChangeType::Updated,
            Self::Removed(_) => LedgerEntryChangeType::Removed,
            Self::State(_) => LedgerEntryChangeType::State,
            Self::Restored(_) => LedgerEntryChangeType::Restored,
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerEntryChangeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerEntryChange {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<LedgerEntryChangeType> for LedgerEntryChange {
    #[must_use]
    fn discriminant(&self) -> LedgerEntryChangeType {
        Self::discriminant(self)
    }
}

impl Variants<LedgerEntryChangeType> for LedgerEntryChange {
    fn variants() -> slice::Iter<'static, LedgerEntryChangeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<LedgerEntryChangeType> for LedgerEntryChange {}

impl ReadXdr for LedgerEntryChange {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: LedgerEntryChangeType = <LedgerEntryChangeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                LedgerEntryChangeType::Created => Self::Created(LedgerEntry::read_xdr(r)?),
                LedgerEntryChangeType::Updated => Self::Updated(LedgerEntry::read_xdr(r)?),
                LedgerEntryChangeType::Removed => Self::Removed(LedgerKey::read_xdr(r)?),
                LedgerEntryChangeType::State => Self::State(LedgerEntry::read_xdr(r)?),
                LedgerEntryChangeType::Restored => Self::Restored(LedgerEntry::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerEntryChange {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Created(v) => v.write_xdr(w)?,
                Self::Updated(v) => v.write_xdr(w)?,
                Self::Removed(v) => v.write_xdr(w)?,
                Self::State(v) => v.write_xdr(w)?,
                Self::Restored(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
