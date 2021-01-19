use crate::Collection;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::Range,
};

type MutSliceCollectionIterator<'a, K, V> =
    std::iter::Map<std::iter::Enumerate<std::slice::Iter<'a, V>>, fn((usize, &V)) -> (K, &V)>;

/// __Note:__ Slice collections are of fixed length, and will panic if calling `insert` or `remove`.
impl<'a, K, V> Collection<'a, K> for &mut [V]
where
    K: 'a + Copy + TryInto<usize> + TryFrom<usize>,
    <K as std::convert::TryInto<usize>>::Error: Debug,
    <K as TryFrom<usize>>::Error: Debug,
    V: 'a,
{
    type Item = V;
    type Iter = MutSliceCollectionIterator<'a, K, V>;
    type KeyIter = std::iter::Map<Range<usize>, fn(usize) -> K>;

    fn get(&'a self, key: &K) -> Option<&'a Self::Item> {
        <[_]>::get(self, (*key).try_into().unwrap())
    }

    fn get_unchecked(&'a self, key: &K) -> &'a Self::Item {
        &self[(*key).try_into().unwrap()]
    }

    fn insert(&mut self, key: K, value: Self::Item) -> Option<Self::Item> {
        if self.contains_key(&key) {
            self[key.try_into().unwrap()] = value;
            None
        } else {
            panic!("Can't extend a slice collection")
        }
    }

    fn remove(&mut self, _key: &K) -> Option<Self::Item> {
        panic!("Can't remove from a slice collection")
    }

    fn iter(&'a self) -> Self::Iter {
        <[_]>::iter(self)
            .enumerate()
            .map(|(key, value)| (key.try_into().unwrap(), value))
    }

    fn keys(&'a self) -> Self::KeyIter {
        (0..<[_]>::len(self)).map(|key| key.try_into().unwrap())
    }

    fn contains_key(&'a self, key: &K) -> bool {
        (*key).try_into().unwrap() < <[_]>::len(self)
    }
}
