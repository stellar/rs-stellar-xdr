#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    stellar_xdr_fuzz::assert_same_encoding!(
        TransactionEnvelope,
        write_type_transaction_envelope,
        data
    );
});
