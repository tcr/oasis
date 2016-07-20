use scope::GcMem;
use std::cell::{RefCell, Ref, RefMut, BorrowState};
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use scope::Expr;

pub type AllocInterior = GcRef<GcMem>;
pub type Alloc = AllocRef<AllocInterior>;

/// Allocate objects.
macro_rules! alloc {
    ( $ctx:expr, $x:expr ) => {
        {
            $ctx.alloc.write().unwrap().pin(GcRef::new($x))
        }
    };
}

pub struct AllocRef<T> {
    ptr: *const T,
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
    inner: RefCell<T>,
    pub debug_str: String,
    marked: AtomicBool,
    young: AtomicBool,
}

impl<T> GcRef<T> {
    pub fn new(item: T) -> GcRef<T> where T: Debug {
        let debug_str = format!("{:?}", item);
        GcRef {
            inner: RefCell::new(item),
            debug_str: debug_str,
            marked: AtomicBool::new(false),
            young: AtomicBool::new(true),
        }
    }

    pub fn marked(&self) -> bool {
        self.marked.load(Ordering::Relaxed)
    }

    pub fn set_marked(&self, value: bool) {
        self.marked.store(value, Ordering::Relaxed);
    }

    pub fn young(&self) -> bool {
        self.young.load(Ordering::Relaxed)
    }

    pub fn set_young(&self, value: bool) {
        self.young.store(value, Ordering::Relaxed);
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

unsafe impl Send for AllocArena { }
unsafe impl Sync for AllocArena { }

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
                (**item).set_marked(false);
            }
        }
    }

    pub fn sweep(&mut self) {
        self.arena.retain(|item| {
            unsafe {
                // Switch youngness to not tag new elements.
                let young = (**item).young();
                (**item).set_young(false);

                if !young && (**item).marked() == false {
                    //println!("***  {:p} {:?}", &*(**item).borrow(), (**item).debug_str);
                    //TODO let container: Box<AllocInterior> = Box::from_raw(*item);
                    //TODO drop(container);
                    false
                } else {
                    true
                }
            }
        });

        //for item in self.arena.iter_mut() {
        //    use std::cell::{Ref, RefMut, BorrowState};
        //    unsafe {
        //        if (**item).marked == false {
        //            if (**item).borrow_state() != BorrowState::Unused {
        //                println!("Cannot free borrowed object!");
        //            }
        //            *(**item).borrow_mut() = GcMem::Deallocated;
        //        }
        //    }
        //}
    }

    /// Rough, poor estimate for arena size.
    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn mark_expr(value: &Expr) {
        match value {
            &Expr::Func(ref inner) => {
                if !inner.marked() {
                    //println!("fn");
                    AllocArena::mark(inner);
                }
            }
            &Expr::Special(ref inner) => {
                if !inner.marked() {
                    //println!("special");
                    AllocArena::mark(inner);
                }
            }
            &Expr::Vec(ref inner) => {
                if !inner.marked() {
                    AllocArena::mark(inner);
                }
            }
            _ => {
                //println!("???");
            }
        }
    }

    pub fn mark_refcell(value: &RefCell<Alloc>) {
        let mut root = value.borrow();
        if !root.marked() {
            AllocArena::mark(&*root);
        }
    }

    pub fn mark(value: &Alloc) {
        //println!("marking start... {:?}", value);
        value.set_marked(true);

        if value.borrow_state() != BorrowState::Unused {
            //println!("*** active borrow state on mem, ignoring: {:?}", value.borrow_state())
        } else {
            match *value.borrow() {
                GcMem::ScopeMem(ref inner) => {
                    //println!("marking scope: {:?}", value);

                    // Collect scope values.
                    let mut values = RefCell::new(vec![]);
                    inner.scope.each(|k, v| {
                        values.borrow_mut().push(v.clone());
                    });
                    // Now mark them.
                    for value in values.into_inner() {
                        AllocArena::mark_expr(&*value.borrow());
                    }

                    if let Some(ref parent) = inner.parent {
                        //println!("parent");
                        if !parent.marked() {
                            AllocArena::mark(parent);
                        }
                        //println!("done parent");
                    }
                }
                GcMem::VecMem(ref inner) => {
                    for i in 0..inner.len() {
                        inner.get(i, |value| {
                            AllocArena::mark_expr(&*value.borrow());
                        });
                    }
                }
                GcMem::FuncMem(ref inner) => {
                    AllocArena::mark(&inner.scope);
                }
                _ => { }
            }
        }
    }
}
