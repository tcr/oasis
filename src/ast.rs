#[derive(Debug)]
pub enum Expr {
    Int(i32),
    Atom(String),
    SExpr(Vec<Box<Expr>>),
}

impl Expr {
    pub fn eval<F>(self, inner: F) -> Expr
    where F: Fn(Expr, Vec<Box<Expr>>) -> Expr {
        match self {
            Expr::SExpr(mut args) => {
                let term = args.remove(0);
                inner(*term, args)
            },
            _ => self,
        }
    }
}
