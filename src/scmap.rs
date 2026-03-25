#![allow(clippy::missing_errors_doc)]

use super::{Error, ScMap, ScMapEntry, ScVal, Validate};
extern crate alloc;
use alloc::vec::Vec;

impl ScMap {
    pub fn sorted_from_entries<I, E>(entries: I) -> Result<ScMap, Error>
    where
        E: TryInto<ScMapEntry>,
        I: Iterator<Item = E>,
    {
        let mut v = entries
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Error::Invalid)?;
        // TODO: Add tests that prove order consistency of ScVal with RawVal. https://github.com/stellar/rs-stellar-xdr/issues/117
        v.sort_by(|a, b| a.key.cmp(&b.key));
        let m = ScMap(v.try_into()?);
        // `validate` will further check that there are no duplicates.
        m.validate()?;
        Ok(m)
    }

    pub fn sorted_from_pairs<K, V, I>(pairs: I) -> Result<ScMap, Error>
    where
        K: TryInto<ScVal>,
        V: TryInto<ScVal>,
        I: Iterator<Item = (K, V)>,
    {
        Self::sorted_from_entries(pairs)
    }

    pub fn sorted_from<I, E>(src: I) -> Result<ScMap, Error>
    where
        E: TryInto<ScMapEntry>,
        I: IntoIterator<Item = E>,
    {
        Self::sorted_from_entries(src.into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::{collections::BTreeMap, vec};

    #[test]
    fn scmap_from_map() -> Result<(), ()> {
        let mut m: BTreeMap<u32, u32> = BTreeMap::new();
        m.insert(1, 2);
        m.insert(5, 6);
        m.insert(3, 4);
        let scm = ScMap::sorted_from(m)?;
        assert_eq!(scm.0.first().unwrap().key, 1u32.into());
        assert_eq!(scm.0.last().unwrap().key, 5u32.into());
        Ok(())
    }

    #[test]
    fn scmap_from_pairs() -> Result<(), ()> {
        let pairs: Vec<(u32, u32)> = vec![(3, 4), (5, 6), (1, 2)];
        let scm = ScMap::sorted_from(pairs)?;
        assert_eq!(scm.0.first().unwrap().key, 1u32.into());
        assert_eq!(scm.0.last().unwrap().key, 5u32.into());
        Ok(())
    }

    #[test]
    fn scmap_from_pairs_containing_duplicate_keys() {
        let pairs: Vec<(u32, u32)> = vec![(3, 4), (3, 5), (5, 6), (1, 2)];
        let scm = ScMap::sorted_from(pairs);
        assert!(scm.is_err());
    }
}
