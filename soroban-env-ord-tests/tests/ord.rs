//! Tests that the `Ord` implementation of `ScVal` in this crate, which is used
//! by `ScMap::sorted_from` and by the `Validate` implementations, orders values
//! the same way the Soroban host does.
//!
//! The host depends on its own pinned copy of `stellar-xdr`, so values are
//! converted between the two copies by round tripping through XDR. Each test
//! compares every pair of values in a corpus that covers every `ScVal` variant,
//! with values on either side of the boundaries where the host switches
//! between small and object representations of a value.
//!
//! See <https://github.com/stellar/rs-stellar-xdr/issues/117>.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use rand::{rngs::StdRng, Rng, SeedableRng};
use soroban_env_host::{
    budget::{AsBudget, Budget},
    xdr as env, Compare, Host, TryFromVal, Val,
};
use stellar_xdr::{
    AccountId, ClaimableBalanceId, ContractExecutable, ContractExecutableExternalRef, ContractId,
    Duration, Hash, Int128Parts, Int256Parts, Limits, MuxedContract, MuxedEd25519Account, PoolId,
    PublicKey, ReadXdr, ScAddress, ScBytes, ScContractInstance, ScError, ScErrorCode, ScMap,
    ScMapEntry, ScNonceKey, ScString, ScVal, ScVec, TimePoint, UInt128Parts, UInt256Parts, Uint256,
    Validate, WriteXdr,
};

fn to_env(v: &ScVal) -> env::ScVal {
    let xdr = v.to_xdr(Limits::none()).unwrap();
    <env::ScVal as env::ReadXdr>::from_xdr(xdr, env::Limits::none()).unwrap()
}

fn from_env(v: &env::ScVal) -> ScVal {
    let xdr = <env::ScVal as env::WriteXdr>::to_xdr(v, env::Limits::none()).unwrap();
    ScVal::from_xdr(xdr, Limits::none()).unwrap()
}

fn budget() -> Budget {
    let budget = Budget::default();
    budget.reset_unlimited().unwrap();
    budget
}

fn host() -> Host {
    let host = Host::default();
    host.as_budget().reset_unlimited().unwrap();
    host
}

/// Converts the value to a host `Val`, returning `None` if the host does not
/// support the value as a `Val`. Asserts that converting back to an `ScVal`
/// round trips to the original value.
fn to_val(host: &Host, v: &ScVal) -> Option<Val> {
    let val = Val::try_from_val(host, &to_env(v)).ok()?;
    let roundtrip = env::ScVal::try_from_val(host, &val).unwrap();
    assert_eq!(&from_env(&roundtrip), v, "round trip through Val");
    Some(val)
}

fn vec(vals: impl IntoIterator<Item = ScVal>) -> ScVal {
    ScVal::Vec(Some(ScVec(
        vals.into_iter().collect::<Vec<_>>().try_into().unwrap(),
    )))
}

/// Builds a map with entries in the given order, without sorting them.
fn map(entries: impl IntoIterator<Item = (ScVal, ScVal)>) -> ScVal {
    ScVal::Map(Some(ScMap(
        entries
            .into_iter()
            .map(|(key, val)| ScMapEntry { key, val })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    )))
}

fn sym(s: &str) -> ScVal {
    ScVal::Symbol(s.try_into().unwrap())
}

#[allow(clippy::cast_possible_truncation)]
fn u128(v: u128) -> ScVal {
    ScVal::U128(UInt128Parts {
        hi: (v >> 64) as u64,
        lo: v as u64,
    })
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn i128(v: i128) -> ScVal {
    ScVal::I128(Int128Parts {
        hi: (v >> 64) as i64,
        lo: v as u64,
    })
}

fn u256(hi_hi: u64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> ScVal {
    ScVal::U256(UInt256Parts {
        hi_hi,
        hi_lo,
        lo_hi,
        lo_lo,
    })
}

fn i256(hi_hi: i64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> ScVal {
    ScVal::I256(Int256Parts {
        hi_hi,
        hi_lo,
        lo_hi,
        lo_lo,
    })
}

/// Values covering every `ScVal` variant. Where the host has both a small and
/// an object representation of a type, values either side of the boundary are
/// included.
#[allow(clippy::too_many_lines)]
fn corpus() -> Vec<ScVal> {
    // The host stores 64-bit values that fit in 56 bits as small values.
    const U56: u64 = 1 << 56;
    const I56: i64 = 1 << 55;
    let u64s = [0, 1, U56 - 1, U56, u64::MAX];
    let i64s = [i64::MIN, -I56 - 1, -I56, -1, 0, 1, I56 - 1, I56, i64::MAX];

    let mut vals = vec![
        ScVal::Bool(false),
        ScVal::Bool(true),
        ScVal::Void,
        ScVal::LedgerKeyContractInstance,
    ];
    vals.extend(
        [
            ScError::Contract(0),
            ScError::Contract(u32::MAX),
            ScError::WasmVm(ScErrorCode::ArithDomain),
            ScError::WasmVm(ScErrorCode::UnexpectedSize),
            ScError::Context(ScErrorCode::InternalError),
            ScError::Storage(ScErrorCode::MissingValue),
            ScError::Object(ScErrorCode::IndexBounds),
            ScError::Crypto(ScErrorCode::InvalidInput),
            ScError::Events(ScErrorCode::ExistingValue),
            ScError::Budget(ScErrorCode::ExceededLimit),
            ScError::Value(ScErrorCode::UnexpectedType),
            ScError::Auth(ScErrorCode::InvalidAction),
        ]
        .map(ScVal::Error),
    );
    vals.extend([0, 1, u32::MAX].map(ScVal::U32));
    vals.extend([i32::MIN, -1, 0, 1, i32::MAX].map(ScVal::I32));
    vals.extend(u64s.map(ScVal::U64));
    vals.extend(i64s.map(ScVal::I64));
    vals.extend(u64s.map(|v| ScVal::Timepoint(TimePoint(v))));
    vals.extend(u64s.map(|v| ScVal::Duration(Duration(v))));
    vals.extend(
        [
            0,
            1,
            u128::from(U56) - 1,
            u128::from(U56),
            u128::from(u64::MAX),
            1 << 64,
            u128::MAX,
        ]
        .map(u128),
    );
    vals.extend(
        [
            i128::MIN,
            -(1 << 64),
            -i128::from(I56) - 1,
            -i128::from(I56),
            -1,
            0,
            1,
            i128::from(I56) - 1,
            i128::from(I56),
            1 << 64,
            i128::MAX,
        ]
        .map(i128),
    );
    vals.extend([
        u256(0, 0, 0, 0),
        u256(0, 0, 0, 1),
        u256(0, 0, 0, U56 - 1),
        u256(0, 0, 0, U56),
        u256(0, 0, 1, 0),
        u256(0, 1, 0, 0),
        u256(1, 0, 0, 0),
        u256(u64::MAX, u64::MAX, u64::MAX, u64::MAX),
    ]);
    #[allow(clippy::cast_sign_loss)]
    vals.extend([
        i256(i64::MIN, 0, 0, 0),
        i256(-1, 0, 0, 0),
        i256(-1, u64::MAX, u64::MAX, (-I56 - 1) as u64),
        i256(-1, u64::MAX, u64::MAX, -I56 as u64),
        i256(-1, u64::MAX, u64::MAX, u64::MAX),
        i256(0, 0, 0, 0),
        i256(0, 0, 0, 1),
        i256(0, 0, 0, I56 as u64 - 1),
        i256(0, 0, 0, I56 as u64),
        i256(0, 0, 1, 0),
        i256(0, 1, 0, 0),
        i256(1, 0, 0, 0),
        i256(i64::MAX, u64::MAX, u64::MAX, u64::MAX),
    ]);
    let bytes: [&[u8]; 7] = [
        b"",
        b"\x00",
        b"\x00\x00",
        b"\x01",
        b"\x01\x00",
        b"\xff",
        b"a",
    ];
    vals.extend(bytes.map(|b| ScVal::Bytes(ScBytes(b.try_into().unwrap()))));
    vals.extend(
        ["", "\0", "a", "ab", "b", "\u{ff}"]
            .map(|s| ScVal::String(ScString(s.try_into().unwrap()))),
    );
    // Symbols of up to 9 characters are small values in the host.
    vals.extend(
        [
            "",
            "0",
            "9",
            "A",
            "Z",
            "_",
            "a",
            "z",
            "ab",
            "a_bcdefgh",
            "zzzzzzzzz",
            "a_bcdefghi",
            "zzzzzzzzzz",
            "a_very_long_symbol_0123456789",
        ]
        .map(sym),
    );
    vals.extend([
        vec([]),
        vec([ScVal::U32(0)]),
        vec([ScVal::U32(0), ScVal::U32(0)]),
        vec([ScVal::U32(0), ScVal::U32(1)]),
        vec([ScVal::U32(1)]),
        vec([ScVal::I32(0)]),
        vec([vec([])]),
        vec([map([])]),
    ]);
    vals.extend([
        map([]),
        map([(ScVal::U32(0), ScVal::U32(0))]),
        map([(ScVal::U32(0), ScVal::U32(1))]),
        map([(ScVal::U32(1), ScVal::U32(0))]),
        map([
            (ScVal::U32(0), ScVal::U32(0)),
            (ScVal::U32(1), ScVal::U32(1)),
        ]),
        map([(ScVal::I32(0), ScVal::U32(0))]),
        map([(vec([]), map([]))]),
    ]);
    let account =
        |b| ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([b; 32]))));
    let contract = |b| ContractId(Hash([b; 32]));
    let addresses = [
        account(0),
        account(0xff),
        ScAddress::Contract(contract(0)),
        ScAddress::Contract(contract(0xff)),
        ScAddress::MuxedAccount(MuxedEd25519Account {
            id: 0,
            ed25519: Uint256([0; 32]),
        }),
        ScAddress::MuxedAccount(MuxedEd25519Account {
            id: 1,
            ed25519: Uint256([0; 32]),
        }),
        ScAddress::MuxedAccount(MuxedEd25519Account {
            id: 0,
            ed25519: Uint256([0xff; 32]),
        }),
        ScAddress::ClaimableBalance(ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([0; 32]))),
        ScAddress::LiquidityPool(PoolId(Hash([0; 32]))),
        ScAddress::MuxedContract(MuxedContract {
            id: 0,
            contract_id: contract(0),
        }),
        ScAddress::MuxedContract(MuxedContract {
            id: 1,
            contract_id: contract(0),
        }),
    ];
    vals.extend(addresses.clone().map(ScVal::Address));
    let executables = [
        ContractExecutable::Wasm(Hash([0; 32])),
        ContractExecutable::Wasm(Hash([0xff; 32])),
        ContractExecutable::StellarAsset,
        ContractExecutable::ExternalRef(ContractExecutableExternalRef {
            executable_owner: addresses[2].clone(),
            tag: ScString("a".try_into().unwrap()),
        }),
    ];
    for executable in executables {
        for storage in [
            None,
            Some(ScMap::default()),
            Some(ScMap::sorted_from([(ScVal::U32(0), ScVal::Void)]).unwrap()),
        ] {
            vals.push(ScVal::ContractInstance(ScContractInstance {
                executable: executable.clone(),
                storage,
            }));
        }
    }
    vals.extend(
        [i64::MIN, -1, 0, 1, i64::MAX].map(|nonce| ScVal::LedgerKeyNonce(ScNonceKey { nonce })),
    );
    vals.extend(
        ["", "a", "ab", "b"].map(|s| ScVal::ExecutableTag(ScString(s.try_into().unwrap()))),
    );

    for v in &vals {
        assert_eq!(v.validate(), Ok(()), "corpus value is invalid: {v:?}");
    }
    vals
}

/// Asserts that the ordering of every pair of values matches the ordering of
/// the host.
fn assert_pairs_consistent(vals: &[ScVal], env_cmp: impl Fn(&ScVal, &ScVal) -> Ordering) {
    for a in vals {
        for b in vals {
            assert_eq!(a.cmp(b), env_cmp(a, b), "\na: {a:?}\nb: {b:?}");
        }
    }
}

/// The `ScVal`s that the host does not represent as `Val`s.
fn is_val(v: &ScVal) -> bool {
    !matches!(
        v,
        ScVal::ContractInstance(_)
            | ScVal::LedgerKeyContractInstance
            | ScVal::LedgerKeyNonce(_)
            | ScVal::Address(ScAddress::ClaimableBalance(_) | ScAddress::LiquidityPool(_))
    )
}

/// Returns the corpus values that the host represents as `Val`s, along with
/// their `Val`s. Asserts that exactly the expected values are supported.
fn corpus_vals(host: &Host) -> Vec<(ScVal, Val)> {
    corpus()
        .into_iter()
        .filter_map(|v| {
            let val = to_val(host, &v);
            assert_eq!(val.is_some(), is_val(&v), "Val support for {v:?}");
            Some((v, val?))
        })
        .collect()
}

/// The host's comparison of `ScVal`s matches this crate's.
#[test]
fn scval_cmp_matches_host_scval_compare() {
    let budget = budget();
    assert_pairs_consistent(&corpus(), |a, b| {
        budget.compare(&to_env(a), &to_env(b)).unwrap()
    });
}

/// The host's comparison of `Val`s, which may be small values or objects,
/// matches this crate's comparison of the equivalent `ScVal`s.
#[test]
fn scval_cmp_matches_host_val_compare() {
    let host = host();
    let vals = corpus_vals(&host);
    for (a, a_val) in &vals {
        for (b, b_val) in &vals {
            let cmp = host.compare(a_val, b_val).unwrap();
            assert_eq!(a.cmp(b), cmp, "\na: {a:?}\nb: {b:?}");
        }
    }
}

type Wrap = dyn Fn(&ScVal, &ScVal) -> ScVal;

/// The ordering is consistent when the values are nested inside vecs and maps,
/// both by key and by value.
#[test]
fn scval_cmp_matches_host_val_compare_nested() {
    let host = host();
    let vals: Vec<ScVal> = corpus_vals(&host).into_iter().map(|(v, _)| v).collect();
    let budget = budget();
    let wrappers: [&Wrap; 5] = [
        &|a, _| vec([a.clone()]),
        &|a, b| vec([a.clone(), b.clone()]),
        &|a, _| vec([ScVal::Void, a.clone()]),
        &|a, _| map([(a.clone(), ScVal::Void)]),
        &|a, _| map([(ScVal::Void, a.clone())]),
    ];
    for wrap in wrappers {
        let wrapped: Vec<ScVal> = vals
            .iter()
            .flat_map(|a| vals.iter().take(8).map(move |b| wrap(a, b)))
            .collect();
        let wrapped_vals: Vec<Val> = wrapped.iter().map(|v| to_val(&host, v).unwrap()).collect();
        for (a, a_val) in wrapped.iter().zip(&wrapped_vals) {
            for (b, b_val) in wrapped.iter().zip(&wrapped_vals) {
                let cmp = a.cmp(b);
                assert_eq!(
                    cmp,
                    host.compare(a_val, b_val).unwrap(),
                    "\na: {a:?}\nb: {b:?}"
                );
                assert_eq!(cmp, budget.compare(&to_env(a), &to_env(b)).unwrap());
            }
        }
    }
}

/// A map validates in this crate if and only if the host accepts it, which
/// requires the keys to be sorted and unique.
#[test]
fn scmap_validate_matches_host() {
    let host = host();
    let vals: Vec<ScVal> = corpus_vals(&host).into_iter().map(|(v, _)| v).collect();
    for a in &vals {
        for b in &vals {
            let m = map([(a.clone(), ScVal::Void), (b.clone(), ScVal::Void)]);
            let valid = m.validate().is_ok();
            assert_eq!(valid, a < b);
            assert_eq!(
                valid,
                Val::try_from_val(&host, &to_env(&m)).is_ok(),
                "\na: {a:?}\nb: {b:?}"
            );
        }
    }
}

/// `ScMap::sorted_from` produces maps the host accepts, with entries in the
/// same order as the host stores them.
#[test]
fn scmap_sorted_from_matches_host() {
    let host = host();
    let vals: Vec<ScVal> = corpus_vals(&host).into_iter().map(|(v, _)| v).collect();
    let m = ScMap::sorted_from(vals.iter().rev().map(|v| (v.clone(), v.clone()))).unwrap();
    let keys: Vec<&ScVal> = m.iter().map(|e| &e.key).collect();
    let mut expected: Vec<&ScVal> = vals.iter().collect();
    expected.sort();
    assert_eq!(keys, expected);
    let m = ScVal::Map(Some(m));
    assert_eq!(m.validate(), Ok(()));
    to_val(&host, &m).expect("host accepts map");
}

/// Builds a random value from the leaves, nesting vecs and maps.
fn random(rng: &mut StdRng, leaves: &[ScVal], depth: u32) -> ScVal {
    match rng.random_range(0..if depth == 0 { 1 } else { 4 }) {
        0 | 1 => leaves[rng.random_range(0..leaves.len())].clone(),
        2 => vec((0..rng.random_range(0..4)).map(|_| random(rng, leaves, depth - 1))),
        _ => {
            // The map is built in the order of this crate's ordering, which
            // the host checks when it accepts the map.
            let m: BTreeMap<ScVal, ScVal> = (0..rng.random_range(0..4))
                .map(|_| {
                    (
                        random(rng, leaves, depth - 1),
                        random(rng, leaves, depth - 1),
                    )
                })
                .collect();
            map(m)
        }
    }
}

/// The ordering is consistent for randomly generated nested values.
#[test]
fn scval_cmp_matches_host_val_compare_random() {
    let host = host();
    let leaves: Vec<ScVal> = corpus_vals(&host).into_iter().map(|(v, _)| v).collect();
    let budget = budget();
    let mut rng = StdRng::seed_from_u64(0);
    let vals: Vec<(ScVal, Val)> = (0..300)
        .map(|_| {
            let v = random(&mut rng, &leaves, 3);
            let val = to_val(&host, &v).expect("host accepts value");
            (v, val)
        })
        .collect();
    for (a, a_val) in &vals {
        for (b, b_val) in &vals {
            let cmp = a.cmp(b);
            assert_eq!(
                cmp,
                host.compare(a_val, b_val).unwrap(),
                "\na: {a:?}\nb: {b:?}"
            );
            assert_eq!(cmp, budget.compare(&to_env(a), &to_env(b)).unwrap());
        }
    }
}
