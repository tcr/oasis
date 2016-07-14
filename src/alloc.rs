use std::any::Any;
use std::cell::{RefCell, Ref, RefMut};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::cmp::Eq;
use std::ops::{Deref, DerefMut};

pub type AllocInterior<T> = GcRef<Box<T>>;
pub type Alloc<T> = AllocRef<AllocInterior<T>>;

/// Allocate objects.
macro_rules! alloc {
    ( $ctx:expr, $x:expr ) => {
        {
            $ctx.pin(GcRef::new(Box::new($x)))
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
            ptr: self.ptr,
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

pub struct GcRef<T> {
    inner: RefCell<T>,
    marked: bool,
}

impl<T> GcRef<T> {
    pub fn new(item: T) -> GcRef<T> {
        GcRef {
            inner: RefCell::new(item),
            marked: false,
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn reset(&mut self) {
        self.marked = false;
    }
}

pub struct AllocArena {
    arena: Vec<*mut GcRef<Box<Any>>>,
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

    pub fn reset(&mut self) {
        for item in self.arena.iter_mut() {
            unsafe {
                (**item).reset();
            }
        }
    }

    pub fn sweep(&mut self) {
        self.arena.retain(|item| {
            unsafe {
                if (**item).marked == false {
                    println!("what's going on here");
                    let container: Box<GcRef<Box<Any>>> = Box::from_raw(*item);
                    drop(container);
                    false
                } else {
                    true
                }
            }
        });
    }

    /// Rough, poor estimate for arena size.
    pub fn size(&self) -> usize {
        self.arena.len()
    }
}
