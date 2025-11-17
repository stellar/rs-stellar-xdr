use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, Cursor, Error as IoError, Read};
use std::path::Path;

pub fn parse_input(input: &[OsString]) -> Result<Vec<Box<dyn Read>>, IoError> {
    if input.is_empty() {
        Ok(vec![Box::new(stdin())])
    } else {
        Ok(input
            .iter()
            .map(|input| {
                let exist = Path::new(input).try_exists();
                if let Ok(true) = exist {
                    let file = File::open(input)?;
                    Ok::<Box<dyn Read>, IoError>(Box::new(file) as Box<dyn Read>)
                } else {
                    Ok(Box::new(Cursor::new(input.clone().into_encoded_bytes())) as Box<dyn Read>)
                }
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
}

pub fn serde_json_value_to_text(v: serde_json::Value) -> Option<String> {
    let s = match v {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(number) => number.to_string(),
        serde_json::Value::String(s) => s,
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => None?,
    };
    Some(s)
}
