use std::collections::HashMap;
use std::cmp;
use std::hash;

#[derive(Clone)]
pub struct OVec<T> {
    inner: Vec<T>,
}

impl<T> OVec<T> {
    pub fn new() -> OVec<T> {
        OVec { inner: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn new_from(input: Vec<T>) -> OVec<T> {
        OVec { inner: input }
    }

    pub fn get<F: Fn(&T) -> R, R>(&self, key: usize, callback: F) -> Option<R> {
        self.inner.get(key).map(|value| callback(&value))
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    pub fn pop(&mut self) {
        self.inner.pop();
    }
}

pub struct OMap<K, V> {
    inner: HashMap<K, V>,
}

impl<K: hash::Hash + cmp::Eq + Clone, V: Clone> OMap<K, V> {
    pub fn new() -> OMap<K, V> {
        OMap { inner: HashMap::new() }
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.inner.insert(k, v);
    }

    /// Guaranteed to find any value existent in the hamt before and during
    /// the function call.
    pub fn search<'a, R, F: Fn(&V) -> R + 'a>(&self, key: &K, callback: F) -> Option<R> {
        self.inner.get(key).map(|value| callback(&value))
    }

    /// Guaranteed to find any value existent in the hamt before and during
    /// the function call.
    pub fn each<F: Fn(&K, &V)>(&self, callback: F) {
        for (k, v) in &self.inner {
            callback(k, v);
        }
    }
}
