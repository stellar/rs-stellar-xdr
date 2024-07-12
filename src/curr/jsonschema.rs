#![cfg(feature = "schemars")]
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};

macro_rules! impl_json_schema_string {
    ($type:ident) => {
        impl JsonSchema for super::$type {
            fn schema_name() -> String {
                stringify!($type).to_string()
            }

            fn json_schema(gen: &mut SchemaGenerator) -> Schema {
                String::json_schema(gen)
            }
        }
    };
}

impl_json_schema_string!(PublicKey);
impl_json_schema_string!(AccountId);
impl_json_schema_string!(MuxedAccount);
impl_json_schema_string!(MuxedAccountMed25519);
impl_json_schema_string!(SignerKey);
impl_json_schema_string!(SignerKeyEd25519SignedPayload);
impl_json_schema_string!(NodeId);
impl_json_schema_string!(ScAddress);
impl_json_schema_string!(AssetCode);
impl_json_schema_string!(AssetCode4);
impl_json_schema_string!(AssetCode12);
impl_json_schema_string!(ClaimableBalanceId);
