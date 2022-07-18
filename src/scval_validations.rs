use crate::ScVal;

pub trait Validate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

impl Validate for ScVal {
    type Error = ();

    fn validate(&self) -> Result<(), Self::Error> {
        // u63, bitset, symbol, obj has to be some, ScMap
        match self {
            ScVal::U63(i) => {
                if *i >= 0 {
                    Ok(())
                } else {
                    Err(())
                }
            }
            ScVal::Symbol(s) => {
                // Symbol is defined as valid per https://github.com/stellar/rs-stellar-contract-env/blob/94c1717516c8fad4ad65caa148183b9fcbc408db/stellar-contract-env-common/src/symbol.rs#L107-L111.
                if s.iter()
                    .all(|c| matches!(*c as char, '_' | '0'..='9' | 'A'..='Z' | 'a'..='z'))
                {
                    Ok(())
                } else {
                    Err(())
                }
            }
            ScVal::Bitset(b) => {
                // Bitset is defined as valid per https://github.com/stellar/rs-stellar-contract-env/blob/94c1717516c8fad4ad65caa148183b9fcbc408db/stellar-contract-env-common/src/bitset.rs#L53-L60.
                if b & 0x0fff_ffff_ffff_ffff == *b {
                    Ok(())
                } else {
                    Err(())
                }
            }
            ScVal::Object(_) => todo!(),
            // Other variants are always valid.
            ScVal::U32(_) | ScVal::I32(_) | ScVal::Static(_) | ScVal::Status(_) => Ok(()),
        }
    }
}
