use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, Cursor, Error as IoError, Read};
use std::path::Path;

/// A reader of an input, along with the limits to use when decoding XDR from
/// it.
pub type Input = (Box<dyn Read>, crate::Limits);

pub fn parse_input(input: &[OsString]) -> Result<Vec<Input>, IoError> {
    if input.is_empty() {
        Ok(vec![(Box::new(stdin()), stdin_limits())])
    } else {
        Ok(input
            .iter()
            .map(|input| {
                let exist = Path::new(input).try_exists();
                if let Ok(true) = exist {
                    let file = File::open(input)?;
                    let limits = file_limits(&file);
                    Ok::<_, IoError>((Box::new(file) as Box<dyn Read>, limits))
                } else {
                    let bytes = input.clone().into_encoded_bytes();
                    let limits = limits(u64::try_from(bytes.len()).ok());
                    Ok((Box::new(Cursor::new(bytes)) as Box<dyn Read>, limits))
                }
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
}

/// Returns the limits to use when decoding XDR from the file.
pub fn file_limits(file: &File) -> crate::Limits {
    limits(file_len(file))
}

/// Returns the limits to use when decoding XDR from stdin.
pub fn stdin_limits() -> crate::Limits {
    limits(stdin_len())
}

/// Returns the limits to use when decoding XDR from an input of the given size.
///
/// Decoded XDR is never larger than the input it is decoded from, so a known
/// size is used as the length limit, causing lengths declared in the XDR that
/// exceed the input to be rejected before anything is allocated.
fn limits(len: Option<u64>) -> crate::Limits {
    len.map_or_else(crate::Limits::none, crate::Limits::len)
}

/// Returns the size of the file if it is a regular file.
///
/// Pipes, FIFOs, and other special files have no known size and return
/// `None`. Regular files reporting a zero size also return `None`, because
/// some special filesystems (e.g. procfs) report zero for non-empty files.
fn file_len(file: &File) -> Option<u64> {
    let metadata = file.metadata().ok()?;
    (metadata.is_file() && metadata.len() > 0).then_some(metadata.len())
}

/// Returns the size of stdin if it is a regular file, such as when it is
/// redirected from a file.
fn stdin_len() -> Option<u64> {
    #[cfg(any(unix, windows))]
    {
        handle_len(&stdin())
    }
    #[cfg(not(any(unix, windows)))]
    {
        None
    }
}

/// Returns the size of the file behind the handle if it is a regular file.
#[cfg(unix)]
fn handle_len(handle: &impl std::os::fd::AsFd) -> Option<u64> {
    let fd = handle.as_fd().try_clone_to_owned().ok()?;
    file_len(&File::from(fd))
}

/// Returns the size of the file behind the handle if it is a regular file.
#[cfg(windows)]
fn handle_len(handle: &impl std::os::windows::io::AsHandle) -> Option<u64> {
    let handle = handle.as_handle().try_clone_to_owned().ok()?;
    file_len(&File::from(handle))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(unix, windows))]
    fn assert_handle_len_of_file_and_pipe() {
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, [1, 2, 3, 4, 5]).unwrap();
        let file = File::open(&path).unwrap();
        assert_eq!(handle_len(&file), Some(5));
        drop(file);
        std::fs::write(&path, []).unwrap();
        let file = File::open(&path).unwrap();
        assert_eq!(handle_len(&file), None);
        drop(file);
        std::fs::remove_file(path).unwrap();

        let (reader, _writer) = std::io::pipe().unwrap();
        assert_eq!(handle_len(&reader), None);
    }

    #[cfg(unix)]
    #[test]
    fn handle_len_unix() {
        assert_handle_len_of_file_and_pipe();
    }

    #[cfg(windows)]
    #[test]
    fn handle_len_windows() {
        assert_handle_len_of_file_and_pipe();
    }
}
