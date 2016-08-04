pub trait Allocator {
    type RefType;
    type RefOut;

    fn pin(&mut self, Self::RefType) -> Self::RefOut;
}
