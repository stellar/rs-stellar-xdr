#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// EncodedLedgerKey is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque EncodedLedgerKey<>;
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
pub struct EncodedLedgerKey(pub BytesM);

impl From<EncodedLedgerKey> for BytesM {
    #[must_use]
    fn from(x: EncodedLedgerKey) -> Self {
        x.0
    }
}

impl From<BytesM> for EncodedLedgerKey {
    #[must_use]
    fn from(x: BytesM) -> Self {
        EncodedLedgerKey(x)
    }
}

impl AsRef<BytesM> for EncodedLedgerKey {
    #[must_use]
    fn as_ref(&self) -> &BytesM {
        &self.0
    }
}

impl ReadXdr for EncodedLedgerKey {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::read_xdr(r)?;
            let v = EncodedLedgerKey(i);
            Ok(v)
        })
    }
}

impl WriteXdr for EncodedLedgerKey {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for EncodedLedgerKey {
    type Target = BytesM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EncodedLedgerKey> for Vec<u8> {
    #[must_use]
    fn from(x: EncodedLedgerKey) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for EncodedLedgerKey {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(EncodedLedgerKey(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for EncodedLedgerKey {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(EncodedLedgerKey(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for EncodedLedgerKey {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for EncodedLedgerKey {
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

/// EncodedLedgerKeyRef is a borrowing equivalent of [`EncodedLedgerKey`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncodedLedgerKeyRef<'a>(pub BytesMRef<'a>);

#[cfg(feature = "alloc")]
impl IntoOwned for EncodedLedgerKeyRef<'_> {
    type Owned = EncodedLedgerKey;
    fn into_owned(self) -> EncodedLedgerKey {
        EncodedLedgerKey(self.0.into_owned())
    }
}

#[cfg(feature = "alloc")]
impl From<&EncodedLedgerKeyRef<'_>> for EncodedLedgerKey {
    #[must_use]
    fn from(v: &EncodedLedgerKeyRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<EncodedLedgerKeyRef<'_>> for EncodedLedgerKey {
    #[must_use]
    fn from(v: EncodedLedgerKeyRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for EncodedLedgerKeyRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "const")]
impl EncodedLedgerKeyRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_encoded_ledger_key(self);
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
        w.write_type_encoded_ledger_key(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`EncodedLedgerKey`], mirroring `<EncodedLedgerKey as WriteXdr>::write_xdr`.
    pub const fn write_type_encoded_ledger_key(&mut self, v: &EncodedLedgerKeyRef<'_>) {
        self.write_var_opaque(v.0.as_slice());
    }

    /// Serializes a variable-length array of [`EncodedLedgerKey`], mirroring `<VecM<EncodedLedgerKey, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_encoded_ledger_key<const MAX: u32>(
        &mut self,
        v: &VecMRef<'_, EncodedLedgerKeyRef<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_encoded_ledger_key(&s[i]);
            i += 1;
        }
    }
}
