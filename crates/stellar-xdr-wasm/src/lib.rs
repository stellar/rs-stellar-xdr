mod utils;

use std::{
    cmp,
    convert::TryInto,
    io::{self, Read},
    str::FromStr,
};

use stellar_xdr::{
    curr::{self, TypeVariant, WriteXdr},
    schema::Schema,
};
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

// #[cfg(feature = "schema")]
#[wasm_bindgen]
pub fn schema(type_variant: &str) -> String {
    let Schema(json_schema): Schema = type_variant.parse().unwrap();
    serde_json::to_string(&json_schema).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

#[wasm_bindgen]
pub fn transaction() -> String {
    let Schema(json_schema): Schema = TypeVariant::Transaction.try_into().unwrap();
    serde_json::to_string(&json_schema).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

#[wasm_bindgen]
pub fn from_xdr(xdr_base64: String, variant: Option<String>) -> String {
    let mut f = curr::Limited::new(ResetRead::new(xdr_base64.as_bytes()), curr::Limits::none());
    for variant in variant
        .map(|v| Some(vec![v.parse::<crate::curr::TypeVariant>().ok()?]))
        .unwrap_or_else(|| Some(TypeVariant::VARIANTS.to_vec()))
        .unwrap()
    {
        f.inner.reset();
        if let Ok(res) = curr::Type::read_xdr_to_end(variant, &mut f) {
            return serde_json::to_string(&res)
                .unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"));
        }
    }
    "{\"error\": \"unknown type\"}".to_string()
}

#[wasm_bindgen]
pub fn to_xdr(json_string: String, variant: String) -> String {
    let r#type = TypeVariant::from_str(&variant).unwrap();
    let t = curr::Type::read_json(r#type, json_string.as_bytes()).unwrap();
    t.to_xdr_base64(curr::Limits::none()).unwrap()
}

struct ResetRead<R: Read> {
    read: R,
    buf: Vec<u8>,
    cursor: usize,
}

impl<R: Read> ResetRead<R> {
    fn new(r: R) -> Self {
        Self {
            read: r,
            buf: Vec::new(),
            cursor: 0,
        }
    }

    fn reset(&mut self) {
        self.cursor = 0;
    }
}

impl<R: Read> Read for ResetRead<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Read from the buffer first into buf.
        let n = cmp::min(self.buf.len() - self.cursor, buf.len());
        buf[..n].copy_from_slice(&self.buf[self.cursor..self.cursor + n]);
        // Read from the reader and cache the result in the buf if the buf is consumed.
        if n < buf.len() {
            let read_n = self.read.read(buf)?;
            self.buf.extend_from_slice(&buf[n..n + read_n]);
            self.cursor += n + read_n;
            Ok(n + read_n)
        } else {
            self.cursor += n;
            Ok(n)
        }
    }
}
