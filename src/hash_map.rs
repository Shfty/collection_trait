use std::{collections::HashMap, hash::Hash};

use crate::Collection;

impl<'a, K, V> Collection<'a, K> for HashMap<K, V>
where
    K: 'a + Copy + Eq + Hash,
    V: 'a,
{
    type Item = V;
    type Iter = std::iter::Map<std::collections::hash_map::Iter<'a, K, V>, fn((&'a K, &'a V)) -> (K, &'a V)>;
    type KeyIter = std::iter::Copied<std::collections::hash_map::Keys<'a, K, V>>;

    fn get(&'a self, key: &K) -> Option<&'a Self::Item> {
        HashMap::get(self, key)
    }

    fn get_unchecked(&'a self, key: &K) -> &'a Self::Item {
        &self[key]
    }

    fn insert(&mut self, key: K, value: Self::Item) -> Option<Self::Item> {
        HashMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &K) -> Option<Self::Item> {
        HashMap::remove(self, key)
    }

    fn iter(&'a self) -> Self::Iter {
        HashMap::iter(self).map(|(key, value)| (*key, value))
    }

    fn keys(&'a self) -> Self::KeyIter {
        self.keys().copied()
    }

    fn contains_key(&'a self, key: &K) -> bool {
        HashMap::contains_key(self, key)
    }
}
