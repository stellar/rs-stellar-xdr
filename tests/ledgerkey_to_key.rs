#![cfg(all(
    any(feature = "curr", feature = "next"),
    not(all(feature = "curr", feature = "next"))
))]
#![cfg(feature = "std")]

#[cfg(feature = "curr")]
use stellar_xdr::curr as stellar_xdr;
#[cfg(feature = "next")]
use stellar_xdr::next as stellar_xdr;

use stellar_xdr::{
    AccountEntry, AccountEntryExt, AccountId, AlphaNum4, Asset, AssetCode4, ClaimableBalanceEntry,
    ClaimableBalanceEntryExt, ClaimableBalanceId, ConfigSettingContractBandwidthV0,
    ConfigSettingEntry, ConfigSettingId, ContractCodeEntry, ContractCodeEntryExt,
    ContractDataDurability, ContractDataEntry, DataEntry, DataEntryExt, ExtensionPoint, Hash,
    LedgerEntry, LedgerEntryData, LedgerEntryExt, LedgerKey, LedgerKeyAccount,
    LedgerKeyClaimableBalance, LedgerKeyConfigSetting, LedgerKeyContractCode,
    LedgerKeyContractData, LedgerKeyData, LedgerKeyLiquidityPool, LedgerKeyOffer,
    LedgerKeyTrustLine, LedgerKeyTtl, LiquidityPoolConstantProductParameters, LiquidityPoolEntry,
    LiquidityPoolEntryBody, LiquidityPoolEntryConstantProduct, OfferEntry, OfferEntryExt, PoolId,
    Price, PublicKey, ScAddress, ScVal, SequenceNumber, String32, String64, Thresholds,
    TrustLineAsset, TrustLineEntry, TrustLineEntryExt, TtlEntry, Uint256,
};

#[test]
fn test_ttl_entry_to_key() {
    let entry = TtlEntry {
        key_hash: Hash([1u8; 32]),
        live_until_ledger_seq: 12345,
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyTtl {
            key_hash: Hash([1u8; 32]),
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::Ttl(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::Ttl(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_account_entry_to_key() {
    let account_id = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([2u8; 32])));
    let entry = AccountEntry {
        account_id: account_id.clone(),
        balance: 1000,
        seq_num: SequenceNumber(1),
        num_sub_entries: 0,
        inflation_dest: None,
        flags: 0,
        home_domain: String32::default(),
        thresholds: Thresholds([0u8; 4]),
        signers: vec![].try_into().unwrap(),
        ext: AccountEntryExt::V0,
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyAccount {
            account_id: account_id.clone(),
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::Account(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::Account(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_trustline_entry_to_key() {
    let account_id = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([3u8; 32])));
    let asset = TrustLineAsset::CreditAlphanum4(AlphaNum4 {
        asset_code: AssetCode4(*b"USDC"),
        issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([4u8; 32]))),
    });

    let entry = TrustLineEntry {
        account_id: account_id.clone(),
        asset: asset.clone(),
        balance: 2000,
        limit: 10000,
        flags: 1,
        ext: TrustLineEntryExt::V0,
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyTrustLine {
            account_id: account_id.clone(),
            asset: asset.clone(),
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::Trustline(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::Trustline(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_offer_entry_to_key() {
    let seller_id = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([5u8; 32])));
    let entry = OfferEntry {
        seller_id: seller_id.clone(),
        offer_id: 12345,
        selling: Asset::Native,
        buying: Asset::CreditAlphanum4(AlphaNum4 {
            asset_code: AssetCode4(*b"USDC"),
            issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([6u8; 32]))),
        }),
        amount: 5000,
        price: Price { n: 1, d: 1 },
        flags: 0,
        ext: OfferEntryExt::V0,
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyOffer {
            seller_id: seller_id.clone(),
            offer_id: 12345,
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::Offer(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::Offer(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_data_entry_to_key() {
    let account_id = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([7u8; 32])));
    let data_name: String64 = "test_data".as_bytes().to_vec().try_into().unwrap();

    let entry = DataEntry {
        account_id: account_id.clone(),
        data_name: data_name.clone(),
        data_value: vec![8u8; 10].try_into().unwrap(),
        ext: DataEntryExt::V0,
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyData {
            account_id: account_id.clone(),
            data_name: data_name.clone(),
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::Data(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::Data(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_claimable_balance_entry_to_key() {
    let balance_id = ClaimableBalanceId::ClaimableBalanceIdTypeV0(Hash([9u8; 32]));

    let entry = ClaimableBalanceEntry {
        balance_id: balance_id.clone(),
        claimants: vec![].try_into().unwrap(),
        asset: Asset::Native,
        amount: 3000,
        ext: ClaimableBalanceEntryExt::V0,
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyClaimableBalance {
            balance_id: balance_id.clone(),
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::ClaimableBalance(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::ClaimableBalance(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_liquidity_pool_entry_to_key() {
    let pool_id = PoolId(Hash([10u8; 32]));

    let entry = LiquidityPoolEntry {
        liquidity_pool_id: pool_id.clone(),
        body: LiquidityPoolEntryBody::LiquidityPoolConstantProduct(
            LiquidityPoolEntryConstantProduct {
                params: LiquidityPoolConstantProductParameters {
                    asset_a: Asset::Native,
                    asset_b: Asset::CreditAlphanum4(AlphaNum4 {
                        asset_code: AssetCode4(*b"USDC"),
                        issuer: AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([11u8; 32]))),
                    }),
                    fee: 30,
                },
                reserve_a: 1000,
                reserve_b: 1000,
                total_pool_shares: 1000,
                pool_shares_trust_line_count: 1,
            },
        ),
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyLiquidityPool {
            liquidity_pool_id: pool_id.clone(),
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::LiquidityPool(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::LiquidityPool(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_contract_data_entry_to_key() {
    let contract = ScAddress::Account(AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(
        [12u8; 32],
    ))));
    let key_data = ScVal::String("key".as_bytes().to_vec().try_into().unwrap());

    let entry = ContractDataEntry {
        contract: contract.clone(),
        key: key_data.clone(),
        val: ScVal::I32(42),
        durability: ContractDataDurability::Persistent,
        ext: ExtensionPoint::V0,
    };

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyContractData {
            contract: contract.clone(),
            key: key_data.clone(),
            durability: ContractDataDurability::Persistent,
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::ContractData(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::ContractData(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_contract_code_entry_to_key() {
    let hash = Hash([13u8; 32]);

    let entry = ContractCodeEntry {
        hash: hash.clone(),
        ext: ContractCodeEntryExt::V0,
        code: vec![].try_into().unwrap(),
    };

    let key = entry.to_key();
    assert_eq!(key, LedgerKeyContractCode { hash: hash.clone() });

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::ContractCode(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::ContractCode(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}

#[test]
fn test_config_setting_entry_to_key() {
    let entry = ConfigSettingEntry::ContractBandwidthV0(ConfigSettingContractBandwidthV0 {
        ledger_max_txs_size_bytes: 100 * 1024,
        tx_max_size_bytes: 100 * 1024,
        fee_tx_size1_kb: 100,
    });

    let key = entry.to_key();
    assert_eq!(
        key,
        LedgerKeyConfigSetting {
            config_setting_id: ConfigSettingId::ContractBandwidthV0,
        }
    );

    let ledger_key: LedgerKey = key.clone().into();
    assert_eq!(ledger_key, LedgerKey::ConfigSetting(key));

    let ledger_entry = LedgerEntry {
        data: LedgerEntryData::ConfigSetting(entry),
        ext: LedgerEntryExt::V0,
        last_modified_ledger_seq: 0,
    };
    assert_eq!(ledger_entry.to_key(), ledger_key);
}
