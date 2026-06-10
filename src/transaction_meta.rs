use crate::{
    LedgerEntryChange, OperationMeta, OperationMetaV2, TransactionMeta, TransactionResultMeta, VecM,
};

extern crate alloc;
use alloc::boxed::Box;

pub enum OperationsMetaRef<'a> {
    V0toV3(&'a VecM<OperationMeta>),
    V4(&'a VecM<OperationMetaV2>),
}

impl TransactionResultMeta {
    pub fn operations(&self) -> OperationsMetaRef<'_> {
        match &self.tx_apply_processing {
            TransactionMeta::V0(value) => OperationsMetaRef::V0toV3(&value),
            TransactionMeta::V1(value) => OperationsMetaRef::V0toV3(&value.operations),
            TransactionMeta::V2(value) => OperationsMetaRef::V0toV3(&value.operations),
            TransactionMeta::V3(value) => OperationsMetaRef::V0toV3(&value.operations),
            TransactionMeta::V4(value) => OperationsMetaRef::V4(&value.operations),
        }
    }

    pub fn changes(&self) -> Box<dyn Iterator<Item = &LedgerEntryChange> + '_> {
        let fee_processing = self.fee_processing.0.iter();
        let tx_apply_processing_ledgers: Box<dyn Iterator<Item = &LedgerEntryChange>> =
            match &self.tx_apply_processing {
                TransactionMeta::V0(operation_meta) => {
                    let res = operation_meta.iter().flat_map(|op| op.changes.0.iter());
                    Box::new(res)
                }

                TransactionMeta::V1(operation_meta) => {
                    let iter = operation_meta
                        .operations
                        .iter()
                        .flat_map(|op| op.changes.0.iter())
                        .chain(operation_meta.tx_changes.0.iter());
                    Box::new(iter)
                }

                TransactionMeta::V2(operation_meta) => {
                    let iter = operation_meta.tx_changes_before.0.iter().chain(
                        operation_meta
                            .operations
                            .iter()
                            .flat_map(|op| op.changes.0.iter())
                            .chain(operation_meta.tx_changes_after.0.iter()),
                    );
                    Box::new(iter)
                }

                TransactionMeta::V3(operation_meta) => {
                    let iter = operation_meta.tx_changes_before.0.iter().chain(
                        operation_meta
                            .operations
                            .iter()
                            .flat_map(|op| op.changes.0.iter())
                            .chain(operation_meta.tx_changes_after.0.iter()),
                    );
                    Box::new(iter)
                }

                TransactionMeta::V4(operation_meta) => {
                    let iter = operation_meta.tx_changes_before.0.iter().chain(
                        operation_meta
                            .operations
                            .iter()
                            .flat_map(|op| op.changes.0.iter())
                            .chain(operation_meta.tx_changes_after.0.iter()),
                    );
                    Box::new(iter)
                }
            };
        Box::new(fee_processing.chain(tx_apply_processing_ledgers))
    }
}
