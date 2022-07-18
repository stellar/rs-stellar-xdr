use crate::ScVal;

pub trait Validate {
    type Error;
    fn validate() -> Result<(), Self::Error>;
}

impl Validate for ScVal {
    type Error = ();

    fn validate() -> Result<(), Self::Error> {
        todo!()
    }
}
