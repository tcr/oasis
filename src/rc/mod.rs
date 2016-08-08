//! The main.rs function dictates which memory management implementation will
//! be used by importing the module as "ac" (i.e. `pub use rc as ac`). We are
//! required in this module to export the types `Ac`, `AcArena`,
//! `types::OVec`, and `types::OMap`. This allows us to swap out memory
//! management strategies without changing any code, as long as a given module
//! implements these types by name.

pub mod types;

use std::hash::{Hash, Hasher};
use std::rc::Rc;
use uuid::Uuid;
use values::{Mem, AcId, Allocator};

/// Implements the "Ac" type for the reference-counted (aka "rc") implementation
/// of our heap. We only need to track two things here: the inner value of our
/// refcell (which can be cloned inside of a single thread without issue) and
/// a unique identifier for this value that can be compared againsta a cloned
/// allocation (Ac) value. We can then implement the necessary traits for Ac:
/// Clone, Hash, and Eq.
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

impl Eq for Ac {}

impl Ac {
    /// Wrap the Mem in an allocated object and generates a new unique ID.
    pub fn new(item: Mem) -> Ac {
        Ac {
            inner: Rc::new(item),
            priv_id: Uuid::new_v4().hyphenated().to_string(),
        }
    }

    /// Get a reference to the inner Mem object.
    pub fn get(&self) -> &Mem {
        &*self.inner
    }

    /// Returns the unique ID for this allocated object.
    pub fn id(&self) -> AcId {
        AcId(self.priv_id.clone())
    }
}

/// The AcArena struct for our reference counting implementation does not
/// require any global tracking of the values we allocated; because reference
/// counted objects track their own internal counts, they will be Deallocated
/// as soon as they are no longer in use.
pub struct AcArena;

impl AcArena {
    pub fn new() -> AcArena {
        AcArena
    }
}

impl Allocator for AcArena {
    type RefType = Mem;
    type RefOut = Ac;

    /// Implements the Allocator function for our reference counting arena.
    /// This simply creates a new allocation object and returns it, trusting it
    /// to do its own automatic memory management.
    fn pin(&mut self, item: Mem) -> Ac {
        Ac::new(item)
    }
}
