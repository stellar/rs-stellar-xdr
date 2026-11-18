use crate::naming::{field_json_rename, field_name, mod_name, set_single_words, type_name};

/// The single words configured by the Makefile. Every test that depends on them
/// calls this; the list can only be set once per test process.
fn single_words() {
    set_single_words(vec!["IPv".to_string()]);
}

#[test]
fn test_type_name_snake_case() {
    assert_eq!(type_name("public_key"), "PublicKey");
}

#[test]
fn test_type_name_screaming_snake_case() {
    assert_eq!(type_name("PUBLIC_KEY_TYPE_ED25519"), "PublicKeyTypeEd25519");
}

#[test]
fn test_type_name_camel_case() {
    assert_eq!(type_name("publicKey"), "PublicKey");
}

#[test]
fn test_type_name_single_word() {
    assert_eq!(type_name("hash"), "Hash");
}

#[test]
fn test_type_name_already_upper_camel() {
    assert_eq!(type_name("PublicKey"), "PublicKey");
}

#[test]
fn test_type_name_escape_type() {
    assert_eq!(type_name("type"), "Type");
}

#[test]
fn test_type_name_escape_error() {
    assert_eq!(type_name("Error"), "SError");
}

#[test]
fn test_type_name_with_digits() {
    assert_eq!(type_name("UINT256"), "Uint256");
    assert_eq!(type_name("ed25519"), "Ed25519");
}

#[test]
fn test_field_name_camel_case() {
    assert_eq!(field_name("publicKey"), "public_key");
}

#[test]
fn test_field_name_already_snake_case() {
    assert_eq!(field_name("public_key"), "public_key");
}

#[test]
fn test_field_name_escape_type() {
    assert_eq!(field_name("type"), "type_");
}

#[test]
fn test_field_json_rename_type() {
    assert_eq!(field_json_rename("type"), Some("type".to_string()));
}

#[test]
fn test_field_json_rename_non_keyword() {
    assert_eq!(field_json_rename("public_key"), None);
}

#[test]
fn test_field_name_upper_camel_case() {
    assert_eq!(field_name("PublicKey"), "public_key");
}

#[test]
fn test_field_name_screaming_snake_case() {
    assert_eq!(field_name("PUBLIC_KEY"), "public_key");
}

#[test]
fn test_field_name_single_word() {
    assert_eq!(field_name("hash"), "hash");
}

#[test]
fn test_field_name_with_digits() {
    assert_eq!(field_name("ed25519"), "ed25519");
}

#[test]
fn test_type_name_trailing_id_uppercase() {
    // XDR types like NodeID, AccountID, PoolID — heck treats "ID" as two words
    assert_eq!(type_name("NodeID"), "NodeId");
    assert_eq!(type_name("AccountID"), "AccountId");
    assert_eq!(type_name("PoolID"), "PoolId");
    assert_eq!(type_name("ConfigSettingID"), "ConfigSettingId");
}

#[test]
fn test_type_name_trailing_id_mixed_case() {
    assert_eq!(type_name("accountId"), "AccountId");
    assert_eq!(type_name("contractID"), "ContractId");
}

#[test]
fn test_type_name_id_in_middle() {
    assert_eq!(
        type_name("ClaimableBalanceIDType"),
        "ClaimableBalanceIdType"
    );
}

#[test]
fn test_field_name_trailing_id() {
    assert_eq!(field_name("nodeID"), "node_id");
    assert_eq!(field_name("accountID"), "account_id");
    assert_eq!(field_name("offerID"), "offer_id");
    assert_eq!(field_name("contractID"), "contract_id");
}

// https://github.com/stellar/rs-stellar-xdr/issues/471
// heck alone splits "IPv4" into ["I", "Pv4"]
#[test]
fn test_type_name_ip_address_variants() {
    single_words();
    assert_eq!(type_name("IPv4"), "Ipv4");
    assert_eq!(type_name("IPv6"), "Ipv6");
    assert_eq!(type_name("IPAddrType"), "IpAddrType");
}

// https://github.com/stellar/rs-stellar-xdr/issues/471
#[test]
fn test_field_name_ip_address_variants() {
    single_words();
    assert_eq!(field_name("ipv4"), "ipv4");
    assert_eq!(field_name("ipv6"), "ipv6");
    assert_eq!(field_name("IPv4"), "ipv4");
    assert_eq!(field_name("IPv6"), "ipv6");
}

#[test]
fn test_mod_name_ip_address_variants() {
    single_words();
    assert_eq!(mod_name("IPv4"), "ipv4");
}

// heck alone splits "IDs" into ["I", "Ds"]
#[test]
fn test_field_name_plural_acronym() {
    assert_eq!(field_name("signerSponsoringIDs"), "signer_sponsoring_ids");
    assert_eq!(field_name("IDs"), "ids");
    assert_eq!(field_name("nodeIDsList"), "node_ids_list");
    assert_eq!(field_name("nodeIDs2"), "node_ids2");
}

#[test]
fn test_type_name_plural_acronym() {
    assert_eq!(type_name("signerSponsoringIDs"), "SignerSponsoringIds");
}

#[test]
fn test_field_json_rename_plural_acronym() {
    assert_eq!(field_json_rename("signerSponsoringIDs"), None);
}

// Acronyms followed by a capitalised word, or a lowercase word that doesn't
// start with "s", are unaffected.
#[test]
fn test_acronym_fixes_do_not_affect_other_names() {
    assert_eq!(type_name("SCPStatement"), "ScpStatement");
    assert_eq!(type_name("SCPStatements"), "ScpStatements");
    assert_eq!(type_name("IPAddrType"), "IpAddrType");
    assert_eq!(type_name("PUBLIC_KEY_TYPE_ED25519"), "PublicKeyTypeEd25519");
    assert_eq!(field_name("SIGNERS"), "signers");
    assert_eq!(field_name("hashIDPreimage"), "hash_id_preimage");
}
