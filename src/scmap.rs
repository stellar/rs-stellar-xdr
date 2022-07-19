#![allow(clippy::result_unit_err)]
#![allow(clippy::missing_errors_doc)]

use crate::{ScMap, ScMapEntry, ScVal, Validate};
extern crate alloc;
use alloc::vec::Vec;

impl ScMap {
    pub fn sorted_from_entries<I: Iterator<Item = ScMapEntry>>(entries: I) -> Result<ScMap, ()> {
        let mut v: Vec<ScMapEntry> = entries.collect();
        v.sort_by(|a, b| a.key.cmp(&b.key));
        let m = ScMap(v.try_into()?);
        // `validate` here will further check that there are no duplicates.
        m.validate()?;
        Ok(m)
    }

    pub fn sorted_from_pairs<K, V, I: Iterator<Item = (K, V)>>(pairs: I) -> Result<ScMap, ()>
    where
        ScVal: From<K> + From<V>,
    {
        Self::sorted_from_entries(pairs.map(|(key, val)| ScMapEntry {
            key: key.into(),
            val: val.into(),
        }))
    }

    pub fn sorted_from<K, V, II: IntoIterator<Item = (K, V)>>(src: II) -> Result<ScMap, ()>
    where
        ScVal: From<K> + From<V>,
    {
        Self::sorted_from_pairs(src.into_iter())
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
    fn scmap_from_bad_pairs() {
        let pairs: Vec<(u32, u32)> = vec![(3, 4), (3, 5), (5, 6), (1, 2)];
        let scm = ScMap::sorted_from(pairs);
        assert!(scm.is_err());
    }
}
