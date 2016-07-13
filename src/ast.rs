#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FuncFnId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Expr {
    Int(i32),
    Atom(String),
    SExpr(Vec<Expr>),
    Str(String),
    Null,
    TailCall(FuncFnId, Vec<Expr>),
}

impl Expr {
    pub fn new_atom(key: &str) -> Expr {
        Expr::Atom(key.to_owned())
    }

    pub fn as_vec<'a>(&'a self) -> &'a Vec<Expr> {
        match self {
            &Expr::SExpr(ref inner) => inner,
            _ => unreachable!(),
        }
    }

    pub fn as_vec_mut<'a>(&'a mut self) -> &'a mut Vec<Expr> {
        match self {
            &mut Expr::SExpr(ref mut inner) => inner,
            _ => unreachable!(),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            &Expr::Int(0) | &Expr::Null => false,
            _ => true,
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            &Expr::Int(value) => value,
            _ => 0,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            &Expr::Str(ref value) => value.clone(),
            &Expr::Int(value) => format!("{}", value),
            rest => format!("{:?}", rest),
        }
    }
}
