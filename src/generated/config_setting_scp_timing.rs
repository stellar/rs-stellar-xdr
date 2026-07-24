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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

impl ConfigSettingScpTiming {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.ledger_target_close_time_milliseconds);
        w.write_u32(self.nomination_timeout_initial_milliseconds);
        w.write_u32(self.nomination_timeout_increment_milliseconds);
        w.write_u32(self.ballot_timeout_initial_milliseconds);
        w.write_u32(self.ballot_timeout_increment_milliseconds);
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

impl WriteXdr for ConfigSettingScpTiming {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_target_close_time_milliseconds.write_xdr(w)?;
            self.nomination_timeout_initial_milliseconds.write_xdr(w)?;
            self.nomination_timeout_increment_milliseconds
                .write_xdr(w)?;
            self.ballot_timeout_initial_milliseconds.write_xdr(w)?;
            self.ballot_timeout_increment_milliseconds.write_xdr(w)?;
            Ok(())
        })
    }
}
