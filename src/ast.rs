#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Ast {
    Int(i32),
    Atom(String),
    SExpr(Vec<Ast>),
    Str(String),
    Null,
}
