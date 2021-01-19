use std::collections::BTreeMap;

use crate::Collection;

impl<'a, K, V> Collection<'a, K> for BTreeMap<K, V>
where
    K: 'a + Copy + Ord,
    V: 'a,
{
    type Item = V;
    type Iter = std::iter::Map<
        std::collections::btree_map::Iter<'a, K, V>,
        fn((&'a K, &'a V)) -> (K, &'a V),
    >;
    type KeyIter = std::iter::Copied<std::collections::btree_map::Keys<'a, K, V>>;

    fn get(&'a self, key: &K) -> Option<&'a Self::Item> {
        BTreeMap::get(self, key)
    }

    fn get_unchecked(&'a self, key: &K) -> &Self::Item {
        &self[key]
    }

    fn insert(&mut self, key: K, value: Self::Item) -> Option<Self::Item> {
        BTreeMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &K) -> Option<Self::Item> {
        BTreeMap::remove(self, key)
    }

    fn iter(&'a self) -> Self::Iter {
        BTreeMap::iter(self).map(|(key, value)| (*key, value))
    }

    fn keys(&'a self) -> Self::KeyIter {
        self.keys().copied()
    }

    fn contains_key(&'a self, key: &K) -> bool {
        BTreeMap::contains_key(self, key)
    }
}
