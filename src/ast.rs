#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Ast {
    Int(i32),
    Atom(String),
    List(Vec<Ast>),
    Str(String),
    Null,
}
