#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AuthenticatedMessageV0 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         uint64 sequence;
///         StellarMessage message;
///         HmacSha256Mac mac;
///     }
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
pub struct AuthenticatedMessageV0 {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub sequence: u64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

impl ReadXdr for AuthenticatedMessageV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                sequence: u64::read_xdr(r)?,
                message: StellarMessage::read_xdr(r)?,
                mac: HmacSha256Mac::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AuthenticatedMessageV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.sequence.write_xdr(w)?;
            self.message.write_xdr(w)?;
            self.mac.write_xdr(w)?;
            Ok(())
        })
    }
}

/// AuthenticatedMessageV0Ref is a borrowing equivalent of [`AuthenticatedMessageV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthenticatedMessageV0Ref<'a> {
    pub sequence: u64,
    pub message: StellarMessageRef<'a>,
    pub mac: HmacSha256Mac,
}

#[cfg(feature = "alloc")]
impl From<&AuthenticatedMessageV0Ref<'_>> for AuthenticatedMessageV0 {
    #[must_use]
    fn from(v: &AuthenticatedMessageV0Ref<'_>) -> Self {
        Self {
            sequence: v.sequence,
            message: (&v.message).into(),
            mac: v.mac.clone(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<AuthenticatedMessageV0Ref<'_>> for AuthenticatedMessageV0 {
    #[must_use]
    fn from(v: AuthenticatedMessageV0Ref<'_>) -> Self {
        Self::from(&v)
    }
}
