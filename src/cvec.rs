use ctrie::hamt::HAMT;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct CVec<T: Sized + Clone> {
    pub inner: HAMT<usize, T>, //TODO not pub
    length: Arc<AtomicUsize>,
}

impl<T: Sized + Clone> CVec<T> {
    pub fn new() -> CVec<T> {
        CVec {
            inner: HAMT::new(),
            length: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn len(&self) -> usize {
        self.length.load(Ordering::Relaxed)
    }

    pub fn new_from(mut input: Vec<T>) -> CVec<T> {
        let vec = CVec::new();
        let len = input.len();
        for i in 0..len {
            vec.inner.insert(i, input.remove(0));
        }
        vec.length.store(len, Ordering::Relaxed);
        vec
    }

    pub fn get<F: Fn(&T) -> R, R>(&self, key: usize, callback: F) -> Option<R> {
        self.inner.search(&key, callback)
    }

    pub fn push(&self, item: T) {
        let pos = self.length.fetch_add(1, Ordering::Relaxed);
        self.inner.insert(pos, item);
    }

    pub fn pop(&self) {
        if self.length.load(Ordering::Relaxed) > 0 {
            let new_len = self.length.fetch_sub(1, Ordering::Relaxed) - 1;
            self.inner.remove(new_len);
        }
    }
}
