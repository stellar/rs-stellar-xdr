#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageDataOp is an XDR Struct defined as:
///
/// ```text
/// struct ManageDataOp
/// {
///     string64 dataName;
///     DataValue* dataValue; // set to null to clear
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
pub struct ManageDataOp {
    pub data_name: String64,
    pub data_value: Option<DataValue>,
}

impl ReadXdr for ManageDataOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                data_name: String64::read_xdr(r)?,
                data_value: Option::<DataValue>::read_xdr(r)?,
            })
        })
    }
}

impl ManageDataOp {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.data_name.const_write_xdr(w);
        {
            w.enter_depth();
            match &self.data_value {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_write_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for ManageDataOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.data_name.write_xdr(w)?;
            self.data_value.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
