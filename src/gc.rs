use alloc::*;
use scope::Expr;
use scope::Mem;
use std::cell::RefCell;

pub type AllocInterior = GcRef<Mem>;
pub type Alloc = AllocRef<AllocInterior>;

pub struct AllocArena {
    arena: Vec<*mut AllocInterior>,
}

unsafe impl Send for AllocArena { }
unsafe impl Sync for AllocArena { }

impl Allocator for AllocArena {
    type RefType = AllocInterior;

    fn pin(&mut self, item: AllocInterior) -> Alloc {
        self.arena.push(Box::into_raw(Box::new(item)) as *mut _);
        AllocRef::new(*self.arena.last().unwrap())
    }
}

impl AllocArena {
    pub fn new() -> AllocArena {
        AllocArena {
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
                    //TODO let container: Box<AllocInterior> = Box::from_raw(*item);
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
            AllocArena::mark(alloc);
        }
    }

    pub fn mark(value: &Alloc) {
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
                    inner.scope.each(|_, v| {
                        values.borrow_mut().push(v.clone());
                    });
                    // Now mark them.
                    for value in values.into_inner() {
                        AllocArena::mark_expr(&value);
                    }

                    if let Some(ref parent) = inner.parent {
                        //println!("parent");
                        AllocArena::mark(parent);
                        //println!("done parent");
                    }
                }
                &Mem::VecMem(ref inner) => {
                    println!("vec mem");
                    for i in 0..inner.len() {
                        inner.get(i, |value| {
                            AllocArena::mark_expr(value);
                        });
                    }
                }
                &Mem::FuncMem(ref inner) => {
                    println!("func mem");
                    AllocArena::mark(&inner.scope);
                }
                _ => { }
            }
        //}
    }
}
