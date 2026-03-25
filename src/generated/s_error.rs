#[allow(unused_imports)]
use super::*;
/// SError is an XDR Struct defined as:
///
/// ```text
/// struct Error
/// {
///     ErrorCode code;
///     string msg<100>;
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
pub struct SError {
    pub code: ErrorCode,
    pub msg: StringM<100>,
}

impl ReadXdr for SError {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                code: ErrorCode::read_xdr(r)?,
                msg: StringM::<100>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SError {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.code.write_xdr(w)?;
            self.msg.write_xdr(w)?;
            Ok(())
        })
    }
}
