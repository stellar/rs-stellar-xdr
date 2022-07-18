use crate::{ScMap, ScObject, ScVal};

pub trait Validate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

impl Validate for ScMap {
    type Error = ();

    fn validate(&self) -> Result<(), Self::Error> {
        // TODO: Validate that the map is sorted and has no duplicates, or find a way to guarantee this to be the case.
        todo!()
    }
}

impl Validate for ScVal {
    type Error = ();

    fn validate(&self) -> Result<(), Self::Error> {
        // u63, bitset, symbol, obj has to be some, ScMap
        match self {
            ScVal::U63(i) => {
                // U63 is defined as valid per https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046.md#host-value-type.
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

            ScVal::Object(None) => Err(()),

            ScVal::Object(Some(o)) => match o {
                ScObject::Map(m) => m.validate(),

                // Other variants of ScObject are always valid.
                ScObject::Vec(_)
                | ScObject::U64(_)
                | ScObject::I64(_)
                | ScObject::I64(_)
                | ScObject::Binary(_)
                | ScObject::BigInt(_)
                | ScObject::Hash(_)
                | ScObject::PublicKey(_) => Ok(()),
            },

            // Other variants of ScVal are always valid.
            ScVal::U32(_) | ScVal::I32(_) | ScVal::Static(_) | ScVal::Status(_) => Ok(()),
        }
    }
}
