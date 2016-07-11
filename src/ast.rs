#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Expr {
    Int(i32),
    Atom(String),
    SExpr(Vec<Box<Expr>>),
    //Func(Box<Fn(Expr) -> Expr>),
}
