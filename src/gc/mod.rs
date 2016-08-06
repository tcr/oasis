use alloc::*;
use scope::Expr;
use scope::Mem;
use std::cell::RefCell;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct GcState<T> {
    pub debug_str: String,
    marked: AtomicBool,
    rooted: AtomicBool,
    freed: AtomicBool,
    seen: AtomicBool,
    inner: T,
}

impl<T> GcState<T> {
    pub fn new(item: T) -> GcState<T> where T: Debug {
        let debug_str = format!("{:?}", item);
        GcState {
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

pub struct Ac {
    ptr: *const GcState<Mem>,
}

impl Ac {
    pub fn new(ptr: *const GcState<Mem>) -> Ac {
        Ac {
            ptr: ptr
        }
    }
}

impl PartialEq for Ac {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for Ac { }

impl fmt::Debug for Ac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ac({:p})", self.ptr)
    }
}

impl Hash for Ac {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl Clone for Ac {
    fn clone(&self) -> Ac {
        Ac {
            ptr: self.ptr,
        }
    }
}

impl Deref for Ac {
    type Target = GcState<Mem>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

pub struct GcArena {
    arena: Vec<*mut GcState<Mem>>,
}

unsafe impl Send for GcArena { }
unsafe impl Sync for GcArena { }

impl Allocator for GcArena {
    type RefType = Mem;
    type RefOut = Ac;

    fn pin(&mut self, item: Mem) -> Ac {
        self.arena.push(Box::into_raw(Box::new(GcState::new(item))) as *mut _);
        Ac::new(*self.arena.last().unwrap())
    }
}

impl GcArena {
    pub fn new() -> GcArena {
        GcArena {
            arena: vec![],
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
                let seen = (**item).seen();
                (**item).set_seen(true);

                // Only drop complete and unmarked elements.
                if seen && (**item).rooted() && (**item).marked() == false {
                    //println!("***  {:p} {:?}", &*(**item).borrow(), (**item).debug_str);
                    //TODO let container: Box<GcState<Mem>> = Box::from_raw(*item);
                    //TODO drop(container);
                    (**item).set_freed(true);
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
        //            *(**item).borrow_mut() = Mem::Deallocated;
        //        }
        //    }
        //}
    }

    /// Rough, poor estimate for arena size.
    pub fn size(&self) -> usize {
        self.arena.len()
    }

    pub fn mark_expr(value: &Expr) {
        if let Some(ref alloc) = value.get_mem() {
            GcArena::mark(alloc);
        }
    }

    pub fn mark(value: &Ac) {
        if value.freed() {
            panic!("Attempted to mark freed object: {:?}", value.get());
        }
        if value.marked() {
            return;
        }

        //println!("marking start... {:?}", value);
        value.set_marked(true);

        //if value.borrow_state() != BorrowState::Unused {
            //println!("*** active borrow state on mem, ignoring: {:?}", value.borrow_state())
        //} else {
            match value.get() {
                &Mem::ScopeMem(ref inner) => {
                    println!("scope mem");
                    //println!("marking scope: {:?}", value);

                    // Collect scope values.
                    let values = RefCell::new(vec![]);
                    inner.scope.borrow().each(|_, v| {
                        values.borrow_mut().push(v.clone());
                    });
                    // Now mark them.
                    for value in values.into_inner() {
                        GcArena::mark_expr(&value);
                    }

                    if let Some(ref parent) = inner.parent {
                        //println!("parent");
                        GcArena::mark(parent);
                        //println!("done parent");
                    }
                }
                &Mem::VecMem(ref inner) => {
                    println!("vec mem");
                    for i in 0..inner.len() {
                        inner.get(i, |value| {
                            GcArena::mark_expr(value);
                        });
                    }
                }
                &Mem::FuncMem(ref inner) => {
                    println!("func mem");
                    GcArena::mark(&inner.scope);
                }
                _ => { }
            }
        //}
    }
}
