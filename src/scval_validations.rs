#![allow(clippy::missing_errors_doc)]

use crate::{ScMap, ScObject, ScVal};

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
        // TODO: Is this sufficient? Is this sufficient for all future variants
        // of ScVal? Should instead we serialize keys to XDR bytes and compare
        // those?
        // Check the map is sorted by key, and there are no keys that are
        // duplicates.
        if self.windows(2).all(|w| w[0].key < w[1].key) {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use crate::{ScMap, ScMapEntry, ScObject, ScVal, Validate};
    use alloc::vec;

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

    #[test]
    fn map() {
        // Values and objects in the data model have a total order. When
        // comparing two values A and B:
        //  - If A is a positive int64 and B is not, A is less than B.
        assert_eq!(
            ScVal::Object(Some(ScObject::Map(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::U63(0),
                        val: ScVal::U32(1),
                    },
                    ScMapEntry {
                        key: ScVal::U63(1),
                        val: ScVal::U63(1),
                    }
                ]
                .try_into()
                .unwrap()
            ))))
            .validate(),
            Ok(())
        );
        //  - If A and B are both positive int64 values, they are ordered by the
        //  normal int64 order.
        //  - If A and B are both tagged and if A has a lesser tag than B, A is
        //  less than B.
        //  - If A and B are both equally tagged, then:
        //    - If they have tag 0, they are ordered by the normal uint32 order
        //    on their low 32 bits.
        //    - If they have tag 1, they are ordered by the normal int32 order
        //    on their low 32 bits.
        //    - If they have tag 2, 5 or 6 or 7 they are ordered by the normal
        //    uint64 order on the zero-extension of their low 60 bits.
        //    - If they have tag 4 they are ordered by the lexicographical order
        //    of their Unicode string value.
        //    - If they have tag 3 they are ordered by calling obj_cmp(A, B)
        //    which performs deep object comparison.
        //
        // Deep object comparison can be accessed by either guest or host: it is
        // provided to guests as a host function via the host environment
        // interface. It performs a recursive structural comparison of objects
        // and values embedded in objects using the following rules:
        //  - If A and B have different object types, they are ordered by object
        //  type code.
        //  - If A and B are boxes, their values are ordered by the value rules
        //  above.
        //  - If A and B are vectors, they are ordered by lexicographic
        //  extension of the value order
        //  - If A and B are maps, they are ordered lexicographically as ordered
        //  vectors of (key, value) pairs
        //  - If A and B are int64 or uint64, they are ordered using the normal
        //  order for those types
        //  - If A and B are binary, they are ordered using the lexicograhical
        //  order of their respective bytes
        assert_eq!(
            ScVal::Object(Some(ScObject::Map(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::U63(0),
                        val: ScVal::U63(1),
                    },
                    ScMapEntry {
                        key: ScVal::U63(1),
                        val: ScVal::U63(1),
                    }
                ]
                .try_into()
                .unwrap()
            ))))
            .validate(),
            Ok(())
        );
        assert_eq!(
            ScVal::Object(Some(ScObject::Map(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::U63(2),
                        val: ScVal::U63(1),
                    },
                    ScMapEntry {
                        key: ScVal::U63(1),
                        val: ScVal::U63(1),
                    }
                ]
                .try_into()
                .unwrap()
            ))))
            .validate(),
            Err(())
        );
        assert_eq!(
            ScVal::Object(Some(ScObject::Map(ScMap(
                vec![
                    ScMapEntry {
                        key: ScVal::U32(1),
                        val: ScVal::U63(1),
                    },
                    ScMapEntry {
                        key: ScVal::U63(2),
                        val: ScVal::U63(1),
                    },
                ]
                .try_into()
                .unwrap()
            ))))
            .validate(),
            Err(())
        );
    }
}
