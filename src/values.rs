use ac::Ac;
use ac::types::OVec;
use ast::*;
use std::fmt;
use scope::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FuncFnId(pub String);

pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
pub type SpecialFn = Fn(&mut Context, Ac, Vec<Expr>) -> Expr;

pub fn funcfn_id(closure: &Ac) -> FuncFnId {
    FuncFnId(closure.id())
}

pub enum Mem {
    Vec(OVec<Expr>),
    Func(Box<FuncFn>, Ac),
    Special(Box<SpecialFn>),
    Scope(Scope),
    Deallocated,
}

impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Mem::Vec(..) => write!(f, "Mem::Vec({:p})", self),
            &Mem::Func(..) => write!(f, "Mem::Func({:p})", self),
            &Mem::Special(..) => write!(f, "Mem::Special({:p})", self),
            &Mem::Scope(..) => write!(f, "Mem::Scope({:p})", self),
            &Mem::Deallocated => write!(f, "**DEALLOCATED**({:p})", self),
        }
    }
}

impl Mem {
    pub fn as_vec(&self) -> &OVec<Expr> {
        match self {
            &Mem::Vec(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_vec_mut(&mut self) -> &mut OVec<Expr> {
        match self {
            &mut Mem::Vec(ref mut inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_func(&self) -> (&Box<FuncFn>, &Ac) {
        match self {
            &Mem::Func(ref func, ref ac) => (func, ac),
            _ => unimplemented!(),
        }
    }

    pub fn as_special(&self) -> &Box<SpecialFn> {
        match self {
            &Mem::Special(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_scope(&self) -> &Scope {
        match self {
            &Mem::Scope(ref inner) => inner,
            _ => panic!("Cannot dereference {:?}", self),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Expr {
    Int(i32),
    Atom(String),
    Str(String),
    Null,
    TailCall(FuncFnId, Vec<Expr>),
    List(Vec<Expr>),

    // Allocations
    Vec(Ac),
    Func(Ac),
    Special(Ac),
}

impl Expr {
    pub fn from_ast(ctx: &mut Context, ast: &Ast) -> Expr {
        match ast {
            &Ast::Int(value) => Expr::Int(value),
            &Ast::Atom(ref value) => Expr::Atom(value.clone()),
            &Ast::List(ref value) => {
                Expr::List(value.iter()
                    .map(|x| Expr::from_ast(ctx, x))
                    .collect())
            }
            &Ast::Str(ref value) => Expr::Str(value.clone()),
            &Ast::Null => Expr::Null,
        }
    }

    pub fn new_atom(key: &str) -> Expr {
        Expr::Atom(key.to_owned())
    }

    pub fn as_list(&self) -> &Vec<Expr> {
        match self {
            &Expr::List(ref inner) => inner,
            _ => unreachable!(),
        }
    }

    pub fn as_vec(&self) -> &OVec<Expr> {
        match self {
            &Expr::Vec(ref alloc) => alloc.get().as_vec(),
            _ => panic!("Attempted to use {:?} as vec", self),
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

    pub fn get_mem(&self) -> Option<&Ac> {
        match self {
            &Expr::Vec(ref inner) |
            &Expr::Func(ref inner) |
            &Expr::Special(ref inner) => Some(inner),
            _ => None,
        }
    }
}
