use std::{convert::{TryFrom, TryInto}, fmt::Debug, ops::Range};

use crate::Collection;

/// __Note:__ `Vec` collections have no mechanism for tracking keys, so a key is valid if it lies within 0..Vec::len().
impl<'a, K, V> Collection<'a, K> for Vec<V>
where
    K: 'a + Copy + TryInto<usize> + TryFrom<usize>,
    <K as std::convert::TryInto<usize>>::Error: Debug,
    <K as TryFrom<usize>>::Error: Debug,
    V: 'a + Default,
{
    type Item = V;
    type Iter = std::iter::Map<std::iter::Enumerate<std::slice::Iter<'a, V>>, fn((usize, &V)) -> (K, &V)>;
    type KeyIter = std::iter::Map<Range<usize>, fn(usize) -> K>;

    fn get(&self, key: &K) -> Option<&Self::Item> {
        <[_]>::get(self, (*key).try_into().unwrap())
    }

    fn get_unchecked(&self, key: &K) -> &Self::Item {
        &self[(*key).try_into().unwrap()]
    }

    fn insert(&mut self, key: K, value: Self::Item) -> Option<Self::Item> {
        if self.contains_key(&key) {
            let key = key.try_into().unwrap();
            self.push(value);
            let item = self.swap_remove(key);
            Some(item)
        } else {
            let key = key.try_into().unwrap();
            self.resize_with(key + 1, Default::default);
            self[key] = value;
            None
        }
    }

    fn remove(&mut self, key: &K) -> Option<Self::Item> {
        if self.contains_key(key) {
            Some(Vec::remove(self, (*key).try_into().unwrap()))
        } else {
            None
        }
    }

    fn iter(&'a self) -> Self::Iter {
        <[_]>::iter(self).enumerate().map(|(key, value)| (key.try_into().unwrap(), value))
    }

    fn keys(&'a self) -> Self::KeyIter {
        (0..<[_]>::len(self)).map(|key| key.try_into().unwrap())
    }

    fn contains_key(&self, key: &K) -> bool {
        (*key).try_into().unwrap() < <[_]>::len(self)
    }
}
