#![allow(clippy::missing_errors_doc)]

use crate::{ScMap, ScObject, ScVal};

pub trait Validate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

impl Validate for ScVal {
    type Error = ();

    fn validate(&self) -> Result<(), Self::Error> {
        match self {
            ScVal::U63(i) => {
                // U63 is defined as valid per https://stellar.org/protocol/cap-46#comparison.
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

impl Validate for ScMap {
    type Error = ();

    fn validate(&self) -> Result<(), Self::Error> {
        // TODO: Validate that the map is sorted and has no duplicates, or find
        // a way to guarantee this to be the case.
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{ScVal, Validate};

    #[test]
    fn u63() {
        assert_eq!(ScVal::U63(0).validate(), Ok(()));
        assert_eq!(ScVal::U63(1).validate(), Ok(()));
        assert_eq!(ScVal::U63(i64::MAX).validate(), Ok(()));
        assert_eq!(ScVal::U63(-1).validate(), Err(()));
    }

    #[test]
    fn symbol() {
        assert_eq!(ScVal::Symbol("".try_into().unwrap()).validate(), Ok(()));
        assert_eq!(ScVal::Symbol("a0A_".try_into().unwrap()).validate(), Ok(()));
        assert_eq!(ScVal::Symbol("]".try_into().unwrap()).validate(), Err(()));
    }

    #[test]
    fn bitset() {
        assert_eq!(ScVal::Bitset(0x0000_0000_0000_0000).validate(), Ok(()));
        assert_eq!(ScVal::Bitset(0x0fff_ffff_ffff_ffff).validate(), Ok(()));
        assert_eq!(ScVal::Bitset(0x1000_0000_0000_0000).validate(), Err(()));
        assert_eq!(ScVal::Bitset(0x1fff_ffff_ffff_ffff).validate(), Err(()));
    }
}
