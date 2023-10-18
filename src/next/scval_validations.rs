#![allow(clippy::missing_errors_doc)]

use super::{Error, ScMap, ScVal};

pub trait Validate {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
}

impl Validate for ScVal {
    type Error = Error;

    fn validate(&self) -> Result<(), <Self as Validate>::Error> {
        match self {
            ScVal::U32(_)
            | ScVal::I32(_)
            | ScVal::Error(_)
            | ScVal::Bool(_)
            | ScVal::Void
            | ScVal::U64(_)
            | ScVal::I64(_)
            | ScVal::Timepoint(_)
            | ScVal::Duration(_)
            | ScVal::U128(_)
            | ScVal::I128(_)
            | ScVal::U256(_)
            | ScVal::I256(_)
            | ScVal::Bytes(_)
            | ScVal::String(_)
            | ScVal::Address(_)
            | ScVal::LedgerKeyContractInstance
            | ScVal::LedgerKeyNonce(_)
            | ScVal::ContractInstance(_) => Ok(()),

            ScVal::Vec(Some(v)) => {
                for e in v.iter() {
                    e.validate()?;
                }
                Ok(())
            }

            ScVal::Symbol(s) => {
                // Symbol is defined as valid per https://github.com/stellar/rs-stellar-contract-env/blob/94c1717516c8fad4ad65caa148183b9fcbc408db/stellar-contract-env-common/src/symbol.rs#L107-L111.
                if s.iter()
                    .all(|c| matches!(*c as char, '_' | '0'..='9' | 'A'..='Z' | 'a'..='z'))
                {
                    Ok(())
                } else {
                    Err(Error::Invalid)
                }
            }
            ScVal::Vec(None) | ScVal::Map(None) => Err(Error::Invalid),
            ScVal::Map(Some(m)) => m.validate(),
        }
    }
}

impl Validate for ScMap {
    type Error = Error;

    fn validate(&self) -> Result<(), Self::Error> {
        // Check every element for validity itself.
        for pair in self.iter() {
            pair.key.validate()?;
            pair.val.validate()?;
        }
        // Check the map is sorted by key, and there are no keys that are
        // duplicates.
        if self.windows(2).all(|w| w[0].key < w[1].key) {
            Ok(())
        } else {
            Err(Error::Invalid)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::next::ScSymbol;

    use super::{Error, ScVal, Validate};

    #[test]
    fn symbol() {
        assert_eq!(
            ScVal::Symbol(ScSymbol("".try_into().unwrap())).validate(),
            Ok(())
        );
        assert_eq!(
            ScVal::Symbol(ScSymbol("a0A_".try_into().unwrap())).validate(),
            Ok(())
        );
        assert_eq!(
            ScVal::Symbol(ScSymbol("]".try_into().unwrap())).validate(),
            Err(Error::Invalid)
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn map() {
        use super::super::{ScMap, ScMapEntry};
        extern crate alloc;
        use alloc::vec;
        // Maps should be sorted by key and have no duplicates. The sort order
        // is just the "normal" sort order on ScVal emitted by derive(PartialOrd).
        assert_eq!(
            ScVal::Map(Some(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::I64(0),
                        val: ScVal::U32(1),
                    },
                    ScMapEntry {
                        key: ScVal::I64(1),
                        val: ScVal::I64(1),
                    }
                ]
                .try_into()
                .unwrap()
            )))
            .validate(),
            Ok(())
        );
        assert_eq!(
            ScVal::Map(Some(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::I64(0),
                        val: ScVal::I64(1),
                    },
                    ScMapEntry {
                        key: ScVal::I64(1),
                        val: ScVal::I64(1),
                    }
                ]
                .try_into()
                .unwrap()
            )))
            .validate(),
            Ok(())
        );
        assert_eq!(
            ScVal::Map(Some(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::I64(2),
                        val: ScVal::I64(1),
                    },
                    ScMapEntry {
                        key: ScVal::I64(1),
                        val: ScVal::I64(1),
                    }
                ]
                .try_into()
                .unwrap()
            )))
            .validate(),
            Err(Error::Invalid)
        );
        assert_eq!(
            ScVal::Map(Some(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::I64(2),
                        val: ScVal::I64(1),
                    },
                    ScMapEntry {
                        key: ScVal::U32(1),
                        val: ScVal::I64(1),
                    },
                ]
                .try_into()
                .unwrap()
            )))
            .validate(),
            Err(Error::Invalid)
        );
    }
}
