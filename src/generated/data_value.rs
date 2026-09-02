#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DataValue is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque DataValue<64>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct DataValue(pub BytesM<64>);

impl From<DataValue> for BytesM<64> {
    #[must_use]
    fn from(x: DataValue) -> Self {
        x.0
    }
}

impl From<BytesM<64>> for DataValue {
    #[must_use]
    fn from(x: BytesM<64>) -> Self {
        DataValue(x)
    }
}

impl AsRef<BytesM<64>> for DataValue {
    #[must_use]
    fn as_ref(&self) -> &BytesM<64> {
        &self.0
    }
}

impl ReadXdr for DataValue {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::<64>::read_xdr(r)?;
            let v = DataValue(i);
            Ok(v)
        })
    }
}

impl WriteXdr for DataValue {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for DataValue {
    type Target = BytesM<64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DataValue> for Vec<u8> {
    #[must_use]
    fn from(x: DataValue) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for DataValue {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(DataValue(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for DataValue {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(DataValue(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for DataValue {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for DataValue {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

/// DataValueView is a borrowing equivalent of [`DataValue`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataValueView<'a>(pub BytesMView<'a, 64>);

#[cfg(feature = "alloc")]
impl From<&DataValueView<'_>> for DataValue {
    #[must_use]
    fn from(v: &DataValueView<'_>) -> Self {
        Self(v.0.to_bytesm())
    }
}

#[cfg(feature = "alloc")]
impl From<DataValueView<'_>> for DataValue {
    #[must_use]
    fn from(v: DataValueView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for DataValueView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "const")]
impl DataValueView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_data_value(self);
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
        w.write_type_data_value(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`DataValue`], mirroring `<DataValue as WriteXdr>::write_xdr`.
    pub const fn write_type_data_value(&mut self, v: &DataValueView<'_>) {
        self.write_var_opaque(v.0.as_slice());
    }

    /// Serializes an optional [`DataValue`], mirroring `<Option<DataValue> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_data_value(&mut self, v: &Option<DataValueView<'_>>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_data_value(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
