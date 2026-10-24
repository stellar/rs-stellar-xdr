#![cfg(feature = "cli")]

use crate::cli::encode;

#[test]
fn test_encode_json_with_schema_at_root_enum() {
    let json_without_schema = r#"{"tx":{"tx":{"source_account":"GDYESST5BMEZLFHTPDJGFF6URVR57XF2BZTBZX5ECLK2FNWHU6MFKOMS","fee":100,"seq_num":"4999668350058497","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"create_account":{"destination":"GASMZEZZ6VHQH2HREIQZZK7HZLV72K3P3JNDUV7U2TMH6KEA5325UNN3","starting_balance":"100000000"}}}],"ext":"v0"},"signatures":[]}}"#;

    // $schema at the beginning of the root object
    let json_schema_start = r#"{"$schema":"https://json-schema.org/draft/2019-09/schema","tx":{"tx":{"source_account":"GDYESST5BMEZLFHTPDJGFF6URVR57XF2BZTBZX5ECLK2FNWHU6MFKOMS","fee":100,"seq_num":"4999668350058497","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"create_account":{"destination":"GASMZEZZ6VHQH2HREIQZZK7HZLV72K3P3JNDUV7U2TMH6KEA5325UNN3","starting_balance":"100000000"}}}],"ext":"v0"},"signatures":[]}}"#;

    // $schema at the end of the root object
    let json_schema_end = r#"{"tx":{"tx":{"source_account":"GDYESST5BMEZLFHTPDJGFF6URVR57XF2BZTBZX5ECLK2FNWHU6MFKOMS","fee":100,"seq_num":"4999668350058497","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"create_account":{"destination":"GASMZEZZ6VHQH2HREIQZZK7HZLV72K3P3JNDUV7U2TMH6KEA5325UNN3","starting_balance":"100000000"}}}],"ext":"v0"},"signatures":[]},"$schema":"https://json-schema.org/draft/2019-09/schema"}"#;

    let r#type = crate::TypeVariant::TransactionEnvelope;

    let (res_clean, ignored_clean) =
        encode::from_json_ignoring_schema(r#type, json_without_schema.as_bytes()).unwrap();
    assert!(ignored_clean.is_empty());

    let (res_start, ignored_start) =
        encode::from_json_ignoring_schema(r#type, json_schema_start.as_bytes()).unwrap();
    assert!(ignored_start.is_empty());
    assert_eq!(res_start, res_clean);

    let (res_end, ignored_end) =
        encode::from_json_ignoring_schema(r#type, json_schema_end.as_bytes()).unwrap();
    assert!(ignored_end.is_empty());
    assert_eq!(res_end, res_clean);
}

#[test]
fn test_encode_json_with_schema_at_root_struct() {
    let json_without_schema = r#"{"source_account":"GDYESST5BMEZLFHTPDJGFF6URVR57XF2BZTBZX5ECLK2FNWHU6MFKOMS","fee":100,"seq_num":"4999668350058497","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"create_account":{"destination":"GASMZEZZ6VHQH2HREIQZZK7HZLV72K3P3JNDUV7U2TMH6KEA5325UNN3","starting_balance":"100000000"}}}],"ext":"v0"}"#;

    let json_with_schema = r#"{"$schema":"https://json-schema.org/draft/2019-09/schema","source_account":"GDYESST5BMEZLFHTPDJGFF6URVR57XF2BZTBZX5ECLK2FNWHU6MFKOMS","fee":100,"seq_num":"4999668350058497","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"create_account":{"destination":"GASMZEZZ6VHQH2HREIQZZK7HZLV72K3P3JNDUV7U2TMH6KEA5325UNN3","starting_balance":"100000000"}}}],"ext":"v0"}"#;

    let r#type = crate::TypeVariant::Transaction;

    let (res_clean, ignored_clean) =
        encode::from_json_ignoring_schema(r#type, json_without_schema.as_bytes()).unwrap();
    assert!(ignored_clean.is_empty());

    let (res_schema, ignored_schema) =
        encode::from_json_ignoring_schema(r#type, json_with_schema.as_bytes()).unwrap();
    assert!(ignored_schema.is_empty());
    assert_eq!(res_schema, res_clean);
}

#[test]
fn test_encode_json_with_unknown_fields_still_rejected() {
    let json_unknown = r#"{"$schema":"https://json-schema.org/draft/2019-09/schema","unknown_extra_field":"bad","source_account":"GDYESST5BMEZLFHTPDJGFF6URVR57XF2BZTBZX5ECLK2FNWHU6MFKOMS","fee":100,"seq_num":"4999668350058497","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"create_account":{"destination":"GASMZEZZ6VHQH2HREIQZZK7HZLV72K3P3JNDUV7U2TMH6KEA5325UNN3","starting_balance":"100000000"}}}],"ext":"v0"}"#;

    let r#type = crate::TypeVariant::Transaction;
    let (_, ignored) = encode::from_json_ignoring_schema(r#type, json_unknown.as_bytes()).unwrap();

    let check_res = encode::check_ignored_fields(&ignored);
    assert!(check_res.is_err());
    match check_res {
        Err(encode::Error::UnknownFields(fields)) => {
            assert_eq!(fields, vec!["unknown_extra_field".to_string()]);
        }
        _ => panic!("expected UnknownFields error"),
    }
}
