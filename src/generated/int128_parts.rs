#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Int128Parts is an XDR Struct defined as:
///
/// ```text
/// struct Int128Parts {
///     int64 hi;
///     uint64 lo;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay)
)]
pub struct Int128Parts {
    pub hi: i64,
    pub lo: u64,
}

impl ReadXdr for Int128Parts {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hi: i64::read_xdr(r)?,
                lo: u64::read_xdr(r)?,
            })
        })
    }
}

impl Int128Parts {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i64(self.hi);
        w.write_u64(self.lo);
        w.leave_depth();
    }
}

impl WriteXdr for Int128Parts {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hi.write_xdr(w)?;
            self.lo.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for Int128Parts {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct Int128Parts {
            hi: i64,
            lo: u64,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Int128PartsOrString<'a> {
            Str(&'a str),
            String(String),
            Int128Parts(Int128Parts),
        }
        match Int128PartsOrString::deserialize(deserializer)? {
            Int128PartsOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            Int128PartsOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            Int128PartsOrString::Int128Parts(Int128Parts { hi, lo }) => {
                Ok(self::Int128Parts { hi, lo })
            }
        }
    }
}
