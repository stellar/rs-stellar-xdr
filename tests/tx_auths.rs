#![cfg(all(
    feature = "alloc",
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{
    Asset, ContractId, Error, FeeBumpTransaction, FeeBumpTransactionEnvelope,
    FeeBumpTransactionExt, FeeBumpTransactionInnerTx, Hash, HostFunction, InvokeContractArgs,
    InvokeHostFunctionOp, Memo, MuxedAccount, Operation, OperationBody, PaymentOp, Preconditions,
    ScAddress, ScSymbol, ScVal, SequenceNumber, SorobanAuthorizationEntry,
    SorobanAuthorizedFunction, SorobanAuthorizedInvocation, SorobanCredentials, Transaction,
    TransactionEnvelope, TransactionExt, TransactionV0, TransactionV0Envelope, TransactionV0Ext,
    TransactionV1Envelope, Uint256,
};

#[test]
fn txv0() -> Result<(), Error> {
    let auth1 = create_soroban_auth_entry(1);
    let auth2 = create_soroban_auth_entry(2);
    let auth3 = create_soroban_auth_entry(3);

    let invoke_op_some = create_invoke_host_op(&[auth1.clone(), auth2.clone()]);
    let invoke_op_more = create_invoke_host_op(&[auth3.clone()]);
    let invoke_op_none = create_invoke_host_op(&[]);
    let other_op = create_payment_op();

    // Mix of ops, expect auths returned is a chain of auths across invoke ops.
    let mut tx = TransactionEnvelope::TxV0(TransactionV0Envelope {
        tx: TransactionV0 {
            source_account_ed25519: Uint256([0; 32]),
            fee: 100,
            seq_num: SequenceNumber(1),
            time_bounds: None,
            memo: Memo::None,
            operations: [invoke_op_some, invoke_op_none, other_op, invoke_op_more]
                .to_vec()
                .try_into()?,
            ext: TransactionV0Ext::V0,
        },
        signatures: [].to_vec().try_into()?,
    });

    // Check .auths()
    let tx_auths: Vec<_> = tx.auths().collect();
    assert_eq!(tx_auths, [&auth1, &auth2, &auth3]);

    // Check .auths_mut()
    #[allow(unused_mut)]
    let mut tx_auths: Vec<_> = tx.auths_mut().collect();
    assert_eq!(tx_auths, [&auth1, &auth2, &auth3]);

    Ok(())
}

#[test]
fn txv1() -> Result<(), Error> {
    let auth1 = create_soroban_auth_entry(1);
    let auth2 = create_soroban_auth_entry(2);
    let auth3 = create_soroban_auth_entry(3);

    let invoke_op_some = create_invoke_host_op(&[auth1.clone(), auth2.clone()]);
    let invoke_op_more = create_invoke_host_op(&[auth3.clone()]);
    let invoke_op_none = create_invoke_host_op(&[]);
    let other_op = create_payment_op();

    // Mix of ops, expect auths returned is a chain of auths across invoke ops.
    let mut tx = TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 100,
            seq_num: SequenceNumber(1),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: [invoke_op_some, invoke_op_none, other_op, invoke_op_more]
                .to_vec()
                .try_into()?,
            ext: TransactionExt::V0,
        },
        signatures: [].to_vec().try_into()?,
    });

    // Check .auths()
    let tx_auths: Vec<_> = tx.auths().collect();
    assert_eq!(tx_auths, [&auth1, &auth2, &auth3]);

    // Check .auths_mut()
    #[allow(unused_mut)]
    let mut tx_auths: Vec<_> = tx.auths_mut().collect();
    assert_eq!(tx_auths, [&auth1, &auth2, &auth3]);

    Ok(())
}

#[test]
fn txfeebump() -> Result<(), Error> {
    let auth1 = create_soroban_auth_entry(1);
    let auth2 = create_soroban_auth_entry(2);
    let auth3 = create_soroban_auth_entry(3);

    let invoke_op_some = create_invoke_host_op(&[auth1.clone(), auth2.clone()]);
    let invoke_op_more = create_invoke_host_op(&[auth3.clone()]);
    let invoke_op_none = create_invoke_host_op(&[]);
    let other_op = create_payment_op();

    // Mix of ops, expect auths returned is a chain of auths across invoke ops.
    let mut tx = TransactionEnvelope::TxFeeBump(FeeBumpTransactionEnvelope {
        tx: FeeBumpTransaction {
            fee_source: MuxedAccount::Ed25519(Uint256([0; 32])),
            fee: 200,
            inner_tx: FeeBumpTransactionInnerTx::Tx(TransactionV1Envelope {
                tx: Transaction {
                    source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
                    fee: 100,
                    seq_num: SequenceNumber(1),
                    cond: Preconditions::None,
                    memo: Memo::None,
                    operations: [invoke_op_some, invoke_op_none, other_op, invoke_op_more]
                        .to_vec()
                        .try_into()?,
                    ext: TransactionExt::V0,
                },
                signatures: [].to_vec().try_into()?,
            }),
            ext: FeeBumpTransactionExt::V0,
        },
        signatures: [].to_vec().try_into()?,
    });

    // Check .auths()
    let tx_auths: Vec<_> = tx.auths().collect();
    assert_eq!(tx_auths, [&auth1, &auth2, &auth3]);

    // Check .auths_mut()
    #[allow(unused_mut)]
    let mut tx_auths: Vec<_> = tx.auths_mut().collect();
    assert_eq!(tx_auths, [&auth1, &auth2, &auth3]);

    Ok(())
}

fn create_soroban_auth_entry(id: u64) -> SorobanAuthorizationEntry {
    SorobanAuthorizationEntry {
        credentials: SorobanCredentials::SourceAccount,
        root_invocation: SorobanAuthorizedInvocation {
            function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                contract_address: ScAddress::Contract(ContractId(Hash([0; 32]))),
                function_name: ScSymbol("test".try_into().unwrap()),
                args: [ScVal::U64(id)].to_vec().try_into().unwrap(),
            }),
            sub_invocations: [].to_vec().try_into().unwrap(),
        },
    }
}

fn create_invoke_host_op(auth_entries: &[SorobanAuthorizationEntry]) -> Operation {
    Operation {
        source_account: None,
        body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
            host_function: HostFunction::InvokeContract(InvokeContractArgs {
                contract_address: ScAddress::Contract(ContractId(Hash([0; 32]))),
                function_name: ScSymbol("test".try_into().unwrap()),
                args: [].to_vec().try_into().unwrap(),
            }),
            auth: auth_entries.try_into().unwrap(),
        }),
    }
}

fn create_payment_op() -> Operation {
    Operation {
        source_account: None,
        body: OperationBody::Payment(PaymentOp {
            asset: Asset::Native,
            destination: MuxedAccount::Ed25519(Uint256([0; 32])),
            amount: 1,
        }),
    }
}
