#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractCostParams is an XDR Typedef defined as:
///
/// ```text
/// typedef ContractCostParamEntry ContractCostParams<CONTRACT_COST_COUNT_LIMIT>;
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
pub struct ContractCostParams(pub VecM<ContractCostParamEntry, 1024>);

impl From<ContractCostParams> for VecM<ContractCostParamEntry, 1024> {
    #[must_use]
    fn from(x: ContractCostParams) -> Self {
        x.0
    }
}

impl From<VecM<ContractCostParamEntry, 1024>> for ContractCostParams {
    #[must_use]
    fn from(x: VecM<ContractCostParamEntry, 1024>) -> Self {
        ContractCostParams(x)
    }
}

impl AsRef<VecM<ContractCostParamEntry, 1024>> for ContractCostParams {
    #[must_use]
    fn as_ref(&self) -> &VecM<ContractCostParamEntry, 1024> {
        &self.0
    }
}

impl ReadXdr for ContractCostParams {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<ContractCostParamEntry, 1024>::read_xdr(r)?;
            let v = ContractCostParams(i);
            Ok(v)
        })
    }
}

impl ContractCostParams {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.0 .0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_to_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for ContractCostParams {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}

impl Deref for ContractCostParams {
    type Target = VecM<ContractCostParamEntry, 1024>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ContractCostParams> for Vec<ContractCostParamEntry> {
    #[must_use]
    fn from(x: ContractCostParams) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<ContractCostParamEntry>> for ContractCostParams {
    type Error = Error;
    fn try_from(x: Vec<ContractCostParamEntry>) -> Result<Self, Error> {
        Ok(ContractCostParams(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<ContractCostParamEntry>> for ContractCostParams {
    type Error = Error;
    fn try_from(x: &Vec<ContractCostParamEntry>) -> Result<Self, Error> {
        Ok(ContractCostParams(x.try_into()?))
    }
}

impl AsRef<Vec<ContractCostParamEntry>> for ContractCostParams {
    #[must_use]
    fn as_ref(&self) -> &Vec<ContractCostParamEntry> {
        &self.0 .0
    }
}

impl AsRef<[ContractCostParamEntry]> for ContractCostParams {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[ContractCostParamEntry] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[ContractCostParamEntry] {
        self.0 .0
    }
}
