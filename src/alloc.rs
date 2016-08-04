use scope::Mem;
use std::cell::{RefCell, Ref, RefMut, BorrowState};
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use scope::Expr;

pub struct AllocRef<T> {
    ptr: *const T,
}

impl<T> AllocRef<T> {
    pub fn new(ptr: *const T) -> AllocRef<T> {
        AllocRef {
            ptr: ptr
        }
    }
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

//impl<T> DerefMut for AllocRef<T> {
//    fn deref_mut(&mut self) -> &mut T {
//        unsafe {
//            &mut *self.ptr
//        }
//    }
//}

pub struct GcRef<T> {
    pub debug_str: String,
    marked: AtomicBool,
    rooted: AtomicBool,
    freed: AtomicBool,
    seen: AtomicBool,
    inner: T,
}

impl<T> GcRef<T> {
    pub fn new(item: T) -> GcRef<T> where T: Debug {
        let debug_str = format!("{:?}", item);
        GcRef {
            inner: item,
            debug_str: debug_str,
            marked: AtomicBool::new(false),
            rooted: AtomicBool::new(false),
            freed: AtomicBool::new(false),
            seen: AtomicBool::new(false),
        }
    }

    pub fn marked(&self) -> bool {
        self.marked.load(Ordering::Relaxed)
    }

    pub fn set_marked(&self, value: bool) {
        self.marked.store(value, Ordering::Relaxed);
    }

    pub fn rooted(&self) -> bool {
        self.rooted.load(Ordering::Relaxed)
    }

    pub fn set_rooted(&self, value: bool) {
        self.rooted.store(value, Ordering::Relaxed);
    }

    pub fn freed(&self) -> bool {
        self.freed.load(Ordering::Relaxed)
    }

    pub fn set_freed(&self, value: bool) {
        self.freed.store(value, Ordering::Relaxed);
    }

    pub fn seen(&self) -> bool {
        self.seen.load(Ordering::Relaxed)
    }

    pub fn set_seen(&self, value: bool) {
        self.seen.store(value, Ordering::Relaxed);
    }

    pub fn get<'a>(&'a self) -> &'a T {
        if self.freed() {
            println!("Attempted to load freed object: {:p}", self);
        }
        &self.inner
    }

    //pub fn borrow_mut(&self) -> RefMut<T> {
    //    self.inner.borrow_mut()
    //}
    //
    //pub fn borrow_state(&self) -> BorrowState {
    //    self.inner.borrow_state()
    //}

    pub fn id(&self) -> String {
        // TODO more unique IDs
        format!("{:p}", self)
    }
}

pub trait Allocator {
    type RefType;
    fn pin(&mut self, Self::RefType) -> AllocRef<Self::RefType>;
}
