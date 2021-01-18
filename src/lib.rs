//! Module holding the `Collection` abstraction and structs that implement it.

mod btree_map;
mod hash_map;
mod slice;
mod mut_slice;
mod vec;
mod vec_deque;

pub use btree_map::*;
pub use hash_map::*;
pub use slice::*;
pub use mut_slice::*;
pub use vec::*;
pub use vec_deque::*;

/// Trait abstraction over a key-value collection.
pub trait Collection<'a, Key> {
    type Item: 'a;
    type KeyIter: 'a + Iterator<Item = Key>;

    fn get(&'a self, key: &Key) -> Option<&Self::Item>;
    fn get_unchecked(&'a self, key: &Key) -> &Self::Item {
        self.get(key).unwrap()
    }
    fn insert(&mut self, key: Key, value: Self::Item) -> Option<Self::Item>;
    fn remove(&mut self, key: &Key) -> Option<Self::Item>;
    fn keys(&'a self) -> Self::KeyIter;
    fn contains_key(&'a self, key: &Key) -> bool;
}
