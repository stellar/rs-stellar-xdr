#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigSettingScpTiming is an XDR Struct defined as:
///
/// ```text
/// struct ConfigSettingSCPTiming {
///     uint32 ledgerTargetCloseTimeMilliseconds;
///     uint32 nominationTimeoutInitialMilliseconds;
///     uint32 nominationTimeoutIncrementMilliseconds;
///     uint32 ballotTimeoutInitialMilliseconds;
///     uint32 ballotTimeoutIncrementMilliseconds;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ConfigSettingScpTiming {
    pub ledger_target_close_time_milliseconds: u32,
    pub nomination_timeout_initial_milliseconds: u32,
    pub nomination_timeout_increment_milliseconds: u32,
    pub ballot_timeout_initial_milliseconds: u32,
    pub ballot_timeout_increment_milliseconds: u32,
}

impl ReadXdr for ConfigSettingScpTiming {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_target_close_time_milliseconds: u32::read_xdr(r)?,
                nomination_timeout_initial_milliseconds: u32::read_xdr(r)?,
                nomination_timeout_increment_milliseconds: u32::read_xdr(r)?,
                ballot_timeout_initial_milliseconds: u32::read_xdr(r)?,
                ballot_timeout_increment_milliseconds: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ConfigSettingScpTiming {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_target_close_time_milliseconds.write_xdr(w)?;
            self.nomination_timeout_initial_milliseconds.write_xdr(w)?;
            self.nomination_timeout_increment_milliseconds.write_xdr(w)?;
            self.ballot_timeout_initial_milliseconds.write_xdr(w)?;
            self.ballot_timeout_increment_milliseconds.write_xdr(w)?;
            Ok(())
        })
    }
}
