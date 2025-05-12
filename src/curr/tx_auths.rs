use super::{
    FeeBumpTransaction, FeeBumpTransactionEnvelope, FeeBumpTransactionInnerTx,
    InvokeHostFunctionOp, Operation, OperationBody, SorobanAuthorizationEntry, Transaction,
    TransactionEnvelope, TransactionV0, TransactionV0Envelope, TransactionV1Envelope, VecM,
};

impl TransactionEnvelope {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        match self {
            TransactionEnvelope::TxV0(e) => e.auths(),
            TransactionEnvelope::Tx(e) => e.auths(),
            TransactionEnvelope::TxFeeBump(e) => e.auths(),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        match self {
            TransactionEnvelope::TxV0(e) => e.auths_mut(),
            TransactionEnvelope::Tx(e) => e.auths_mut(),
            TransactionEnvelope::TxFeeBump(e) => e.auths_mut(),
        }
    }
}

impl FeeBumpTransactionEnvelope {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        self.tx.auths()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        self.tx.auths_mut()
    }
}

impl FeeBumpTransaction {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        self.inner_tx.auths()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        self.inner_tx.auths_mut()
    }
}

impl FeeBumpTransactionInnerTx {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        match self {
            FeeBumpTransactionInnerTx::Tx(e) => e.auths(),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        match self {
            FeeBumpTransactionInnerTx::Tx(e) => e.auths_mut(),
        }
    }
}

impl TransactionV1Envelope {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        self.tx.auths()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        self.tx.auths_mut()
    }
}

impl TransactionV0Envelope {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        self.tx.auths()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        self.tx.auths_mut()
    }
}

impl Transaction {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        self.operations.auths()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        self.operations.auths_mut()
    }
}

impl TransactionV0 {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        self.operations.auths()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        self.operations.auths_mut()
    }
}

type FlatMapOperationToSorobanAuthorizationEntryIter<'a> = core::iter::FlatMap<
    core::slice::Iter<'a, Operation>,
    core::slice::Iter<'a, SorobanAuthorizationEntry>,
    fn(&'a Operation) -> core::slice::Iter<'a, SorobanAuthorizationEntry>,
>;

#[cfg(feature = "alloc")]
type FlatMapOperationToSorobanAuthorizationEntryIterMut<'a> = core::iter::FlatMap<
    core::slice::IterMut<'a, Operation>,
    core::slice::IterMut<'a, SorobanAuthorizationEntry>,
    fn(&'a mut Operation) -> core::slice::IterMut<'a, SorobanAuthorizationEntry>,
>;

impl<const MAX: u32> VecM<Operation, MAX> {
    pub fn auths(&self) -> FlatMapOperationToSorobanAuthorizationEntryIter<'_> {
        self.iter().flat_map(Operation::auths)
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> FlatMapOperationToSorobanAuthorizationEntryIterMut<'_> {
        self.iter_mut().flat_map(Operation::auths_mut)
    }
}

impl Operation {
    pub fn auths(&self) -> core::slice::Iter<'_, SorobanAuthorizationEntry> {
        self.body.auths()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> core::slice::IterMut<'_, SorobanAuthorizationEntry> {
        self.body.auths_mut()
    }
}

impl OperationBody {
    pub fn auths(&self) -> core::slice::Iter<'_, SorobanAuthorizationEntry> {
        match self {
            OperationBody::InvokeHostFunction(op) => op.auths(),
            _ => [].iter(),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> core::slice::IterMut<'_, SorobanAuthorizationEntry> {
        match self {
            OperationBody::InvokeHostFunction(op) => op.auths_mut(),
            _ => [].iter_mut(),
        }
    }
}

impl InvokeHostFunctionOp {
    pub fn auths(&self) -> core::slice::Iter<'_, SorobanAuthorizationEntry> {
        self.auth.iter()
    }

    #[cfg(feature = "alloc")]
    pub fn auths_mut(&mut self) -> core::slice::IterMut<'_, SorobanAuthorizationEntry> {
        self.auth.iter_mut()
    }
}
