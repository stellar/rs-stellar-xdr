use super::{BytesM, ScVal};
impl From<BytesM> for ScVal {
    fn from(value: BytesM) -> Self {
        ScVal::Bytes(value.into())
    }
}