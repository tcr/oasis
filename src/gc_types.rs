use cvec::CVec;
use ctrie::hamt::HAMT;

pub type OVec<T> = CVec<T>;
pub type OMap<K, V> = HAMT<K, V>;
