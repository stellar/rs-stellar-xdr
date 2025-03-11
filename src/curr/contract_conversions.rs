use super::{
    Hash, HostFunction, InvokeContractArgs, InvokeHostFunctionOp, Operation, OperationBody,
    ScAddress, VecM,
};

#[cfg(feature = "alloc")]
mod stellar_strkey_contract {
    use super::super::{Error, ScVal};
    use super::*;
    impl From<Hash> for stellar_strkey::Contract {
        fn from(v: Hash) -> Self {
            stellar_strkey::Contract(v.0)
        }
    }
    impl From<&Hash> for stellar_strkey::Contract {
        fn from(v: &Hash) -> Self {
            stellar_strkey::Contract(v.into())
        }
    }

    impl From<stellar_strkey::Contract> for Hash {
        fn from(v: stellar_strkey::Contract) -> Self {
            Hash(v.0)
        }
    }
    impl From<&stellar_strkey::Contract> for Hash {
        fn from(stellar_strkey::Contract(bytes): &stellar_strkey::Contract) -> Self {
            Hash(*bytes)
        }
    }

    impl From<stellar_strkey::Contract> for ScAddress {
        fn from(v: stellar_strkey::Contract) -> Self {
            ScAddress::Contract(v.into())
        }
    }
    impl From<&stellar_strkey::Contract> for ScAddress {
        fn from(v: &stellar_strkey::Contract) -> Self {
            ScAddress::Contract(v.into())
        }
    }

    impl TryFrom<ScAddress> for stellar_strkey::Contract {
        type Error = Error;
        fn try_from(sc_address: ScAddress) -> Result<Self, Self::Error> {
            match sc_address {
                ScAddress::Contract(c) => Ok(c.into()),
                _ => Err(Error::Invalid),
            }
        }
    }

    impl From<stellar_strkey::Contract> for ScVal {
        fn from(contract: stellar_strkey::Contract) -> Self {
            ScVal::Address(contract.into())
        }
    }

    impl From<&stellar_strkey::Contract> for ScVal {
        fn from(contract: &stellar_strkey::Contract) -> Self {
            ScVal::Address(contract.into())
        }
    }
}

impl From<&Hash> for [u8; 32] {
    fn from(Hash(bytes): &Hash) -> Self {
        *bytes
    }
}

impl From<Hash> for ScAddress {
    fn from(hash: Hash) -> Self {
        ScAddress::Contract(hash)
    }
}

impl From<&Hash> for ScAddress {
    fn from(hash: &Hash) -> Self {
        ScAddress::Contract(hash.into())
    }
}

impl From<&Hash> for Hash {
    fn from(Hash(bytes): &Hash) -> Self {
        Hash(*bytes)
    }
}

impl From<InvokeContractArgs> for HostFunction {
    fn from(parameters: InvokeContractArgs) -> Self {
        HostFunction::InvokeContract(parameters)
    }
}

impl From<InvokeContractArgs> for Operation {
    fn from(parameters: InvokeContractArgs) -> Self {
        Operation {
            source_account: None,
            body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function: parameters.into(),
                auth: VecM::default(),
            }),
        }
    }
}

impl From<HostFunction> for Operation {
    fn from(host_function: HostFunction) -> Self {
        Operation {
            source_account: None,
            body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                host_function,
                auth: VecM::default(),
            }),
        }
    }
}
