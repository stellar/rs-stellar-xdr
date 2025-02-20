use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, Cursor, Read};
use std::path::Path;

pub fn parse_input<E>(input: &Vec<OsString>) -> Result<Vec<Box<dyn Read>>, E>
where
    E: From<std::io::Error>,
{
    if input.is_empty() {
        Ok(vec![Box::new(stdin())])
    } else {
        Ok(input
            .iter()
            .map(|input| {
                let exist = Path::new(input).try_exists();
                if let Ok(true) = exist {
                    let file = File::open(input)?;
                    Ok::<Box<dyn Read>, E>(Box::new(file) as Box<dyn Read>)
                } else {
                    Ok(Box::new(Cursor::new(input.clone().into_encoded_bytes())) as Box<dyn Read>)
                }
            })
            .collect::<Result<Vec<_>, _>>()?)
    }
}
