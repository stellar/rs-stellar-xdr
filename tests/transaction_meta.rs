#![cfg(feature = "alloc")]

use stellar_xdr::transaction_meta::OperationsMetaRef;
use stellar_xdr::LedgerEntryChange;
use stellar_xdr::{
    ContractEvent, DiagnosticEvent, ExtensionPoint, LedgerEntry, LedgerEntryChanges,
    LedgerEntryData, LedgerEntryExt, OperationMeta, OperationMetaV2, SorobanTransactionMeta,
    SorobanTransactionMetaV2, TransactionEvent, TransactionMeta, TransactionMetaV1,
    TransactionMetaV2, TransactionMetaV3, TransactionMetaV4, TransactionResultMeta,
    TransactionResultPair, VecM,
};

fn create_ledger_entry(seq: u32) -> LedgerEntry {
    LedgerEntry {
        last_modified_ledger_seq: seq,
        data: LedgerEntryData::default(),
        ext: LedgerEntryExt::V0,
    }
}

fn create_ledger_entry_change_created(ledger_entry: LedgerEntry) -> LedgerEntryChange {
    LedgerEntryChange::Created(ledger_entry)
}

fn create_ledger_entry_change_updated(ledger_entry: LedgerEntry) -> LedgerEntryChange {
    LedgerEntryChange::Updated(ledger_entry)
}

fn create_ledger_entry_change_restored(ledger_entry: LedgerEntry) -> LedgerEntryChange {
    LedgerEntryChange::Restored(ledger_entry)
}

fn create_ledger_entry_changes(ledger_entry_change: Vec<LedgerEntryChange>) -> LedgerEntryChanges {
    LedgerEntryChanges(VecM::try_from(ledger_entry_change).unwrap())
}

fn create_operation_meta(ledger_entry_changes: LedgerEntryChanges) -> OperationMeta {
    OperationMeta {
        changes: ledger_entry_changes,
    }
}

fn create_operation_meta_v2(ledger_entry_changes: LedgerEntryChanges) -> OperationMetaV2 {
    OperationMetaV2 {
        ext: ExtensionPoint::default(),
        changes: ledger_entry_changes,
        events: VecM::try_from(vec![ContractEvent::default()]).unwrap(),
    }
}

fn create_transaction_meta_v1(
    tx_changes: LedgerEntryChanges,
    operations: Vec<OperationMeta>,
) -> TransactionMetaV1 {
    TransactionMetaV1 {
        tx_changes,
        operations: VecM::try_from(operations).unwrap(),
    }
}

#[allow(dead_code)]
fn create_transaction_meta_v2(
    tx_changes_before: LedgerEntryChanges,
    operations: Vec<OperationMeta>,
    tx_changes_after: LedgerEntryChanges,
) -> TransactionMetaV2 {
    TransactionMetaV2 {
        tx_changes_before,
        operations: VecM::try_from(operations).unwrap(),
        tx_changes_after,
    }
}

#[allow(dead_code)]
fn create_transaction_meta_v3(
    tx_changes_before: LedgerEntryChanges,
    operations: Vec<OperationMeta>,
    tx_changes_after: LedgerEntryChanges,
) -> TransactionMetaV3 {
    TransactionMetaV3 {
        ext: ExtensionPoint::default(),
        tx_changes_before,
        operations: VecM::try_from(operations).unwrap(),
        tx_changes_after,
        soroban_meta: Some(SorobanTransactionMeta::default()),
    }
}

fn create_transaction_meta_v4(
    tx_changes_before: LedgerEntryChanges,
    operations: Vec<OperationMetaV2>,
    tx_changes_after: LedgerEntryChanges,
) -> TransactionMetaV4 {
    TransactionMetaV4 {
        ext: ExtensionPoint::default(),
        tx_changes_before,
        operations: VecM::try_from(operations).unwrap(),
        tx_changes_after,
        soroban_meta: Some(SorobanTransactionMetaV2::default()),
        events: VecM::try_from(vec![TransactionEvent::default()]).unwrap(),
        diagnostic_events: VecM::try_from(vec![DiagnosticEvent::default()]).unwrap(),
    }
}

fn create_transaction_result_meta(
    result: TransactionResultPair,
    fee_processing: LedgerEntryChanges,
    tx_apply_processing: TransactionMeta,
) -> TransactionResultMeta {
    TransactionResultMeta {
        result,
        fee_processing,
        tx_apply_processing,
    }
}

#[test]
fn test_transaction_meta_v0_default_operations() {
    let transaction_result_meta = TransactionResultMeta {
        result: TransactionResultPair::default(),
        fee_processing: LedgerEntryChanges::default(),
        tx_apply_processing: TransactionMeta::default(),
    };

    match transaction_result_meta.operations() {
        OperationsMetaRef::V0toV3(ops) => {
            assert_eq!(ops.len(), 0);
        }
        OperationsMetaRef::V4(_) => panic!("unexpected V4 in test"),
    }
}

#[test]
fn test_transaction_meta_v1_operations() {
    let entry1 = create_ledger_entry(1);
    let entry2 = create_ledger_entry(2);
    let ledger_entry_created = create_ledger_entry_change_created(entry1);
    let ledger_entry_updated = create_ledger_entry_change_updated(entry2);
    let ledger_entry_changes =
        create_ledger_entry_changes(vec![ledger_entry_created, ledger_entry_updated]);
    let operation_meta = create_operation_meta(ledger_entry_changes.clone());
    let transaction_meta =
        create_transaction_meta_v1(ledger_entry_changes.clone(), vec![operation_meta]);
    let transaction_result_meta_v1 = create_transaction_result_meta(
        TransactionResultPair::default(),
        ledger_entry_changes,
        TransactionMeta::V1(transaction_meta),
    );
    match transaction_result_meta_v1.operations() {
        OperationsMetaRef::V0toV3(ops) => {
            assert_eq!(ops.len(), 1);

            let ledger_changes = &ops.get(0).unwrap().changes;
            let first_ledger_name = ledger_changes.get(0).unwrap().name();
            assert_eq!(first_ledger_name, "Created");

            let second_ledger_name = ledger_changes.get(1).unwrap().name();
            assert_eq!(second_ledger_name, "Updated");
        }
        OperationsMetaRef::V4(_) => panic!("unexpected V4 in test"),
    }
}

#[test]
fn test_transaction_meta_v4_operations() {
    let ledger_entry_created = create_ledger_entry_change_created(create_ledger_entry(1));
    let ledger_entry_updated = create_ledger_entry_change_updated(create_ledger_entry(2));
    let ledger_entry_restored = create_ledger_entry_change_restored(create_ledger_entry(3));
    let ledger_entry_fee_processing = create_ledger_entry_change_restored(create_ledger_entry(4));
    let tx_changes_before = create_ledger_entry_changes(vec![ledger_entry_created]);
    let tx_changes_after = create_ledger_entry_changes(vec![ledger_entry_updated]);
    let tx_restored = create_ledger_entry_changes(vec![ledger_entry_restored]);
    let fee_processing = create_ledger_entry_changes(vec![ledger_entry_fee_processing]);
    let operation1 = create_operation_meta_v2(tx_restored.clone());
    let operation2 = create_operation_meta_v2(tx_changes_before.clone());
    let operation3 = create_operation_meta_v2(tx_changes_after.clone());
    let transaction_meta_v4 = create_transaction_meta_v4(
        tx_changes_before,
        vec![operation1, operation2, operation3],
        tx_changes_after,
    );

    let transaction_result_meta_v4 = create_transaction_result_meta(
        TransactionResultPair::default(),
        fee_processing,
        TransactionMeta::V4(transaction_meta_v4),
    );
    match transaction_result_meta_v4.operations() {
        OperationsMetaRef::V0toV3(_) => panic!("unexpected V0-V3 in test"),

        OperationsMetaRef::V4(ops) => {
            assert_eq!(ops.len(), 3);

            let first_ledger_name = &ops.get(0).unwrap().changes.get(0).unwrap().name();
            assert_eq!(*first_ledger_name, "Restored");

            let second_ledger_name = &ops.get(1).unwrap().changes.get(0).unwrap().name();
            assert_eq!(*second_ledger_name, "Created");

            let third_ledger_name = &ops.get(2).unwrap().changes.get(0).unwrap().name();
            assert_eq!(*third_ledger_name, "Updated");
        }
    }
}

#[test]
fn test_transaction_meta_v1_changes() {
    let entry1 = create_ledger_entry(1);
    let entry2 = create_ledger_entry(2);
    let ledger_entry_created = create_ledger_entry_change_created(entry1);
    let ledger_entry_updated = create_ledger_entry_change_updated(entry2);
    let ledger_entry_fee_processing = create_ledger_entry_change_restored(create_ledger_entry(3));
    let ledger_entry_changes =
        create_ledger_entry_changes(vec![ledger_entry_created, ledger_entry_updated]);
    let operation_meta = create_operation_meta(ledger_entry_changes.clone()); //2
    let fee_processing = create_ledger_entry_changes(vec![ledger_entry_fee_processing]); //1
    let transaction_meta =
        create_transaction_meta_v1(ledger_entry_changes.clone(), vec![operation_meta]); //4
    let transaction_result_meta_v1 = create_transaction_result_meta(
        TransactionResultPair::default(),
        fee_processing,
        TransactionMeta::V1(transaction_meta),
    );

    let changes: Vec<&LedgerEntryChange> = transaction_result_meta_v1.changes().collect();
    assert_eq!(changes.len(), 5);
}

#[test]
fn test_transaction_meta_v4_changes() {
    let ledger_entry_created = create_ledger_entry_change_created(create_ledger_entry(1));
    let ledger_entry_updated = create_ledger_entry_change_updated(create_ledger_entry(2));
    let ledger_entry_restored = create_ledger_entry_change_restored(create_ledger_entry(3));
    let ledger_entry_fee_processing = create_ledger_entry_change_restored(create_ledger_entry(4));
    let tx_changes_before = create_ledger_entry_changes(vec![ledger_entry_created]);
    let tx_changes_after = create_ledger_entry_changes(vec![ledger_entry_updated]);
    let tx_restored = create_ledger_entry_changes(vec![ledger_entry_restored]);
    let fee_processing = create_ledger_entry_changes(vec![ledger_entry_fee_processing]);
    let operation1 = create_operation_meta_v2(tx_restored.clone());
    let operation2 = create_operation_meta_v2(tx_changes_before.clone());
    let operation3 = create_operation_meta_v2(tx_changes_after.clone());
    let transaction_meta_v4 = create_transaction_meta_v4(
        tx_changes_before,
        vec![operation1, operation2, operation3],
        tx_changes_after,
    );

    let transaction_result_meta_v4 = create_transaction_result_meta(
        TransactionResultPair::default(),
        fee_processing,
        TransactionMeta::V4(transaction_meta_v4),
    );

    let changes: Vec<&LedgerEntryChange> = transaction_result_meta_v4.changes().collect();
    assert_eq!(changes.len(), 6);
}
