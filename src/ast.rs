#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Expr {
    Int(i32),
    Atom(String),
    SExpr(Vec<Box<Expr>>),
    Str(String),
}

impl Expr {
    pub fn new_atom(key: &str) -> Expr {
        Expr::Atom(key.to_owned())
    }
}
