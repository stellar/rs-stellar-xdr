use crate::naming::{field_name, type_name};

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
// heck splits "IPv4" into ["I", "Pv4"] producing surprising results
#[test]
fn test_type_name_ip_address_variants() {
    assert_eq!(type_name("IPv4"), "IPv4");
    assert_eq!(type_name("IPv6"), "IPv6");
    assert_eq!(type_name("IPAddrType"), "IpAddrType");
}

// https://github.com/stellar/rs-stellar-xdr/issues/471
#[test]
fn test_field_name_ip_address_variants() {
    assert_eq!(field_name("ipv4"), "ipv4");
    assert_eq!(field_name("ipv6"), "ipv6");
    assert_eq!(field_name("IPv4"), "i_pv4");
    assert_eq!(field_name("IPv6"), "i_pv6");
}
