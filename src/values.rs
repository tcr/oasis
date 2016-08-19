use ac::Ac;
use ac::types::OVec;
use ast::Ast;
use std::fmt;
use scope::*;

/// The type of a function that can be called from user code.
pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
/// The type of a "special form" that can be invoked in user code.
pub type SpecialFn = Fn(&mut Context, Ac, Vec<Expr>) -> Expr;

/// An identifier that uniquely identifies an allocated object.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct AcId(pub String);

/// A heap-allocated object. These reside on the heap and are tracked by an
/// Allocator, which is unique for each program Context.
pub enum Mem {
    /// A vector.
    Vec(OVec<Expr>),
    /// A function invokable from code.
    Func(Box<FuncFn>, Ac),
    /// A "special form" invokable from code.
    Special(Box<SpecialFn>),
    /// A scope object, which is tracked on the heap.
    Scope(Scope),
    /// This value indicates a memory value which has been deallocated, but
    /// for debugging purposes has not been removed (just replaced with this
    /// value).
    Deallocated,
}

impl Mem {
    /// Interpret a heap-allocated object as a Vec<>, or panic.
    pub fn as_vec(&self) -> &OVec<Expr> {
        match self {
            &Mem::Vec(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    /// Interpret a heap-allocated object as a mutable Vec<>, or panic.
    pub fn as_vec_mut(&mut self) -> &mut OVec<Expr> {
        match self {
            &mut Mem::Vec(ref mut inner) => inner,
            _ => unimplemented!(),
        }
    }

    /// Interpret a heap-allocated object as a code function, or panic.
    pub fn as_func(&self) -> (&Box<FuncFn>, &Ac) {
        match self {
            &Mem::Func(ref func, ref ac) => (func, ac),
            _ => unimplemented!(),
        }
    }

    /// Interpret a heap-allocated object as a "special form", or panic.
    pub fn as_special(&self) -> &Box<SpecialFn> {
        match self {
            &Mem::Special(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    /// Interpret a heap-allocated object as a scope object, or panic.
    pub fn as_scope(&self) -> &Scope {
        match self {
            &Mem::Scope(ref inner) => inner,
            _ => panic!("Cannot dereference {:?}", self),
        }
    }
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

/// Trait for an arena that can allocate a Mem object. For a single-threaded
/// system, this may simply create a reference counted object on the heap.
/// For a more advanced allocator, this may create an object whose usage is
/// traced through the program and released when unused.
pub trait Allocator {
    type RefType;
    type RefOut;

    fn pin(&mut self, Self::RefType) -> Self::RefOut;
}

/// Expr object. Contains any AST literal (integer, atoms, strings, s-exprs...),
/// valid heap-allocated objects (vectors, functions), and a return value that
/// indicates a tail call optimization should occur.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Expr {
    Int(i32),
    Atom(String),
    Str(String),
    Null,
    List(Vec<Expr>),

    // Valid heap-allocated expression values.
    Vec(Ac),
    Func(Ac),
    Special(Ac),

    /// Return value from a tail-call optimized function. Contains the ID of
    /// the function to recurse and the list of arguments it takes.
    TailCall(AcId, Vec<Expr>),
}

impl Expr {
    /// Converts an AST object into its corresponding expression values. As
    /// the AST is a strict subset of Expr, this cannot fail,
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

    /// Attempts to get the list an s-expression wraps, or panics if it cannot
    /// be used in this way.
    pub fn as_list(&self) -> &Vec<Expr> {
        match self {
            &Expr::List(ref inner) => inner,
            _ => panic!("Attempted to use {:?} as list", self),
        }
    }

    /// Attempts to get the vector an expression wraps, or panics if it cannot
    /// be used in this way.
    pub fn as_vec(&self) -> &OVec<Expr> {
        match self {
            &Expr::Vec(ref alloc) => alloc.get().as_vec(),
            _ => panic!("Attempted to use {:?} as vec", self),
        }
    }

    /// Converts an expression to its "truthy" boolean value.
    pub fn as_bool(&self) -> bool {
        match self {
            &Expr::Int(0) | &Expr::Null => false,
            _ => true,
        }
    }

    /// Converts an expression to an integer, or 0 if the value is not an int.
    pub fn as_int(&self) -> i32 {
        match self {
            &Expr::Int(value) => value,
            _ => 0,
        }
    }

    /// Converts an expression to a String.
    pub fn as_string(&self) -> String {
        match self {
            &Expr::Str(ref value) => value.clone(),
            &Expr::Int(value) => format!("{}", value),
            rest => format!("{:?}", rest),
        }
    }

    /// Unwraps a heap-allocated object and returns the allocation backing it.
    /// Otherwise, returns a None value.
    pub fn get_mem(&self) -> Option<&Ac> {
        match self {
            &Expr::Vec(ref inner) |
            &Expr::Func(ref inner) |
            &Expr::Special(ref inner) => Some(inner),
            _ => None,
        }
    }
}
