use scope::GcMem;
use std::cell::{RefCell, Ref, RefMut, BorrowState};
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use std::ops::{Deref, DerefMut};

pub type AllocInterior = GcRef<GcMem>;
pub type Alloc = AllocRef<AllocInterior>;

/// Allocate objects.
macro_rules! alloc {
    ( $ctx:expr, $x:expr ) => {
        {
            $ctx.pin(GcRef::new($x))
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
    pub debug_str: String,
    pub marked: bool,
}

impl<T> GcRef<T> {
    pub fn new(item: T) -> GcRef<T> where T: Debug {
        let debug_str = format!("{:?}", item);
        GcRef {
            inner: RefCell::new(item),
            debug_str: debug_str,
            marked: false,
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    pub fn borrow_state(&self) -> BorrowState {
        self.inner.borrow_state()
    }

    pub fn id(&self) -> String {
        // TODO more unique IDs
        format!("{:p}", self)
    }
}

pub struct AllocArena {
    arena: Vec<*mut AllocInterior>,
}

static mut ctx_tracker: usize = 0;

impl AllocArena {
    pub fn new() -> AllocArena {
        AllocArena {
            arena: vec![],
        }
    }

    pub fn pin(&mut self, item: AllocInterior) -> Alloc {
        self.arena.push(Box::into_raw(Box::new(item)) as *mut _);
        AllocRef {
            ptr: *self.arena.last().unwrap(),
        }
    }

    pub fn reset(&mut self) {
        for item in self.arena.iter_mut() {
            unsafe {
                (**item).marked = false;
            }
        }
    }

    pub fn sweep(&mut self) {
        self.arena.retain(|item| {
            unsafe {
                if (**item).marked == false {
                    //println!("***  {:p} {:?}", &*(**item).borrow(), (**item).debug_str);
                    let container: Box<AllocInterior> = Box::from_raw(*item);
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
