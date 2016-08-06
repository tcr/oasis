use alloc::Allocator;
use scope::Mem;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AllocOut {
    inner: Rc<Mem>,
    priv_id: String,
}

impl Hash for AllocOut {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.priv_id.hash(state);
    }
}

impl PartialEq for AllocOut {
    fn eq(&self, other: &Self) -> bool {
        self.priv_id == other.priv_id
    }
}

impl Eq for AllocOut { }

impl AllocOut {
    pub fn new(item: Mem) -> AllocOut {
        AllocOut {
            inner: Rc::new(item),
            priv_id: Uuid::new_v4().hyphenated().to_string(),
        }
    }

    pub fn get<'a>(&'a self) -> &'a Mem {
        &*self.inner
    }

    pub fn id(&self) -> String {
        self.priv_id.clone()
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
