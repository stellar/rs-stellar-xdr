#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScContractInstance is an XDR Struct defined as:
///
/// ```text
/// struct SCContractInstance {
///     ContractExecutable executable;
///     SCMap* storage;
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
pub struct ScContractInstance {
    pub executable: ContractExecutable,
    pub storage: Option<ScMap>,
}

impl ReadXdr for ScContractInstance {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                executable: ContractExecutable::read_xdr(r)?,
                storage: Option::<ScMap>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScContractInstance {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.executable.write_xdr(w)?;
            self.storage.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScContractInstanceView is a borrowing equivalent of [`ScContractInstance`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScContractInstanceView<'a> {
    pub executable: ContractExecutableView<'a>,
    pub storage: Option<ScMapView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScContractInstanceView<'_> {
    type Owned = ScContractInstance;
    fn into_owned(&self) -> ScContractInstance {
        ScContractInstance {
            executable: IntoOwned::into_owned(&self.executable),
            storage: IntoOwned::into_owned(&self.storage),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScContractInstanceView<'_>> for ScContractInstance {
    #[must_use]
    fn from(v: &ScContractInstanceView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<ScContractInstanceView<'_>> for ScContractInstance {
    #[must_use]
    fn from(v: ScContractInstanceView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for ScContractInstanceView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.executable.write_xdr(w)?;
            self.storage.write_xdr(w)?;
            Ok(())
        })
    }
}
