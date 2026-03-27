#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryChangeType is an XDR Enum defined as:
///
/// ```text
/// enum LedgerEntryChangeType
/// {
///     LEDGER_ENTRY_CREATED = 0, // entry was added to the ledger
///     LEDGER_ENTRY_UPDATED = 1, // entry was modified in the ledger
///     LEDGER_ENTRY_REMOVED = 2, // entry was removed from the ledger
///     LEDGER_ENTRY_STATE    = 3, // value of the entry
///     LEDGER_ENTRY_RESTORED = 4  // archived entry was restored in the ledger
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum LedgerEntryChangeType {
    #[cfg_attr(feature = "alloc", default)]
    Created = 0,
    Updated = 1,
    Removed = 2,
    State = 3,
    Restored = 4,
}

impl LedgerEntryChangeType {
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
            Self::Created => "Created",
            Self::Updated => "Updated",
            Self::Removed => "Removed",
            Self::State => "State",
            Self::Restored => "Restored",
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerEntryChangeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerEntryChangeType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<LedgerEntryChangeType> for LedgerEntryChangeType {
    fn variants() -> slice::Iter<'static, LedgerEntryChangeType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for LedgerEntryChangeType {}

impl fmt::Display for LedgerEntryChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerEntryChangeType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => LedgerEntryChangeType::Created,
            1 => LedgerEntryChangeType::Updated,
            2 => LedgerEntryChangeType::Removed,
            3 => LedgerEntryChangeType::State,
            4 => LedgerEntryChangeType::Restored,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerEntryChangeType> for i32 {
    #[must_use]
    fn from(e: LedgerEntryChangeType) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerEntryChangeType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerEntryChangeType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
