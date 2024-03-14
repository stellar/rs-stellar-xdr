mod utils;

use stellar_xdr::{curr::TypeVariant, schema::Schema};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn type_variants() -> String {
    format!(
        "[{}]",
        TypeVariant::VARIANTS_STR
            .iter()
            .map(|s| format!("{s:?}"))
            .collect::<Vec<String>>()
            .join(", ")
    )
}

#[wasm_bindgen]
pub fn schema(s: &str) -> String {
    let Schema(json_schema): Schema = s.parse().unwrap();
    let s = match serde_json::to_string(&json_schema) {
        Ok(s) => s,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    s
}
