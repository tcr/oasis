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

pub struct FuncInner {
    pub body: Box<FuncFn>,
    pub scope: Ac,
}

pub enum Mem {
    VecMem(OVec<Expr>),
    FuncMem(FuncInner),
    SpecialMem(Box<SpecialFn>),
    ScopeMem(Scope),
    Deallocated,
}

impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Mem::VecMem(..) => write!(f, "VecMem({:p})", self),
            &Mem::FuncMem(..) => write!(f, "FuncMem({:p})", self),
            &Mem::SpecialMem(..) => write!(f, "SpecialMem({:p})", self),
            &Mem::ScopeMem(..) => write!(f, "ScopeMem({:p})", self),
            &Mem::Deallocated => write!(f, "**DEALLOCATED**({:p})", self),
        }
    }
}

impl Mem {
    pub fn as_vec(&self) -> &OVec<Expr> {
        match self {
            &Mem::VecMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_vec_mut(&mut self) -> &mut OVec<Expr> {
        match self {
            &mut Mem::VecMem(ref mut inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_func(&self) -> &FuncInner {
        match self {
            &Mem::FuncMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_special(&self) -> &Box<SpecialFn> {
        match self {
            &Mem::SpecialMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_scope(&self) -> &Scope {
        match self {
            &Mem::ScopeMem(ref inner) => inner,
            _ => panic!("Cannot dereference {:?}", self),
        }
    }

    pub fn wrap_fn(target: Box<FuncFn>, scope: Ac) -> Mem {
        Mem::FuncMem(FuncInner {
            body: target,
            scope: scope,
        })
    }

    pub fn wrap_special(target: Box<SpecialFn>) -> Mem {
        Mem::SpecialMem(target)
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
                Expr::List(value.iter().map(|x| {
                    Expr::from_ast(ctx, x)
                }).collect())
            }
            &Ast::Str(ref value) => Expr::Str(value.clone()),
            &Ast::Null => Expr::Null,
        }
    }

    pub fn new_atom(key: &str) -> Expr {
        Expr::Atom(key.to_owned())
    }

    pub fn as_list<'a>(&'a self) -> &'a Vec<Expr> {
        match self {
            &Expr::List(ref inner) => inner,
            _ => unreachable!(),
        }
    }

    pub fn as_vec<'a>(&'a self) -> &'a OVec<Expr> {
        match self {
            &Expr::Vec(ref alloc) => {
                alloc.get().as_vec()
            }
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
            &Expr::Vec(ref inner) => Some(inner),
            &Expr::Func(ref inner) => Some(inner),
            &Expr::Special(ref inner) => Some(inner),
            _ => None,
        }
    }
}
