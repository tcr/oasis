use alloc::Allocator;
use scope::Mem;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AllocOut {
    inner: Rc<Mem>,
}

impl Hash for AllocOut {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self as *const AllocOut) as usize).hash(state);
    }
}

impl PartialEq for AllocOut {
    fn eq(&self, other: &Self) -> bool {
        (self as *const AllocOut) == (other as *const AllocOut)
    }
}

impl Eq for AllocOut { }

impl AllocOut {
    pub fn new(item: Mem) -> AllocOut {
        AllocOut {
            inner: Rc::new(item),
        }
    }

    pub fn get<'a>(&'a self) -> &'a Mem {
        &*self.inner
    }

    pub fn id(&self) -> String {
        format!("{:p}", self)
    }
}

pub struct RcArena;

impl RcArena {
    pub fn new() -> RcArena {
        RcArena
    }
}

impl Allocator for RcArena {
    type RefType = Mem;
    type RefOut = AllocOut;

    fn pin(&mut self, item: Mem) -> AllocOut {
        AllocOut::new(item)
    }
}
