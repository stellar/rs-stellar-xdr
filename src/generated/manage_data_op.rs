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

impl WriteXdr for ManageDataOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.data_name.write_xdr(w)?;
            self.data_value.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ManageDataOpView is a borrowing equivalent of [`ManageDataOp`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManageDataOpView<'a> {
    pub data_name: String64View<'a>,
    pub data_value: Option<DataValueView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&ManageDataOpView<'_>> for ManageDataOp {
    #[must_use]
    fn from(v: &ManageDataOpView<'_>) -> Self {
        Self {
            data_name: (&v.data_name).into(),
            data_value: v.data_value.as_ref().map(Into::into),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ManageDataOpView<'_>> for ManageDataOp {
    #[must_use]
    fn from(v: ManageDataOpView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ManageDataOpView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.data_name.write_xdr(w)?;
            self.data_value.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ManageDataOpView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_manage_data_op(self);
        w.len()
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
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_manage_data_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ManageDataOp`], mirroring `<ManageDataOp as WriteXdr>::write_xdr`.
    pub const fn write_type_manage_data_op(&mut self, v: &ManageDataOpView<'_>) {
        self.write_type_string64(&v.data_name);
        self.write_type_option_data_value(&v.data_value);
    }
}
