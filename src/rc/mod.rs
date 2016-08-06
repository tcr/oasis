use alloc::Allocator;
use scope::Mem;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Ac {
    inner: Rc<Mem>,
    priv_id: String,
}

impl Hash for Ac {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.priv_id.hash(state);
    }
}

impl PartialEq for Ac {
    fn eq(&self, other: &Self) -> bool {
        self.priv_id == other.priv_id
    }
}

impl Eq for Ac { }

impl Ac {
    pub fn new(item: Mem) -> Ac {
        Ac {
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

pub struct AcArena;

impl AcArena {
    pub fn new() -> AcArena {
        AcArena
    }
}

impl Allocator for AcArena {
    type RefType = Mem;
    type RefOut = Ac;

    fn pin(&mut self, item: Mem) -> Ac {
        Ac::new(item)
    }
}
