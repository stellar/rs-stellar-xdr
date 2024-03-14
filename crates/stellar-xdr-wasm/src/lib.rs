mod utils;

use std::convert::TryInto;

use stellar_xdr::schema::Schema;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn schema() -> String {
    let Schema(json_schema): Schema = stellar_xdr::curr::TypeVariant::Transaction.try_into().unwrap();
    let s = match serde_json::to_string(&json_schema) {
        Ok(s) => s,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    s
}
