use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::cmp::Eq;
use std::ops::{Deref, DerefMut};

pub type AllocInterior<T> = RefCell<Box<T>>;
pub type Alloc<T> = AllocRef<AllocInterior<T>>;

/// Allocate objects.
macro_rules! alloc {
    ( $ctx:expr, $x:expr ) => {
        {
            use std::cell::RefCell;
            $ctx.pin(RefCell::new(Box::new($x)))
        }
    };
}

pub struct AllocRef<T> {
    ptr: *mut T,
}

impl<T> PartialEq for AllocRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for AllocRef<T> { }

impl<T> fmt::Debug for AllocRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AllocRef({:p})", self.ptr)
    }
}

impl<T> Hash for AllocRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl<T> Clone for AllocRef<T> {
    fn clone(&self) -> AllocRef<T> {
        AllocRef {
            ptr: self.ptr
        }
    }
}

impl<T> Deref for AllocRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T> DerefMut for AllocRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.ptr
        }
    }
}

pub struct AllocArena {
    arena: Vec<*mut RefCell<Box<Any>>>,
}

impl AllocArena {
    pub fn new() -> AllocArena {
        AllocArena {
            arena: vec![],
        }
    }

    pub fn pin<T: ?Sized>(&mut self, item: AllocInterior<T>) -> Alloc<T> {
        unsafe {
            self.arena.push(Box::into_raw(Box::new(item)) as *mut _);
            AllocRef {
                ptr: mem::transmute(*self.arena.last().unwrap()),
            }
        }
    }

    /// Kinda rough estimate for arena size.
    pub fn size(&self) -> usize {
        self.arena.len()
    }
}
