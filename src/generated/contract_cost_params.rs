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
pub struct ContractCostParams(pub VecM<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT>);

impl From<ContractCostParams> for VecM<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT> {
    #[must_use]
    fn from(x: ContractCostParams) -> Self {
        x.0
    }
}

impl From<VecM<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT>> for ContractCostParams {
    #[must_use]
    fn from(x: VecM<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT>) -> Self {
        ContractCostParams(x)
    }
}

impl AsRef<VecM<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT>> for ContractCostParams {
    #[must_use]
    fn as_ref(&self) -> &VecM<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT> {
        &self.0
    }
}

impl ReadXdr for ContractCostParams {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT>::read_xdr(r)?;
            let v = ContractCostParams(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ContractCostParams {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ContractCostParams {
    type Target = VecM<ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT>;
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

/// ContractCostParamsView is a borrowing equivalent of [`ContractCostParams`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCostParamsView<'a>(
    pub VecMView<'a, ContractCostParamEntry, CONTRACT_COST_COUNT_LIMIT>,
);

#[cfg(feature = "alloc")]
impl From<&ContractCostParamsView<'_>> for ContractCostParams {
    #[must_use]
    fn from(v: &ContractCostParamsView<'_>) -> Self {
        Self(v.0.to_vecm())
    }
}

#[cfg(feature = "alloc")]
impl From<ContractCostParamsView<'_>> for ContractCostParams {
    #[must_use]
    fn from(v: ContractCostParamsView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ContractCostParamsView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}
