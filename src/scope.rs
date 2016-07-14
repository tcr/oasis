use ast::*;
use std::any::Any;
use std::cell::{RefCell, Ref, RefMut};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::cmp::Eq;
use std::ops::{Deref, DerefMut};

pub type AllocInterior<T> = RefCell<Box<T>>;
pub type Alloc<T> = AllocRef<AllocInterior<T>>;

/// Allocate objects.
macro_rules! alloc {
    ( $ctx:expr, $x:expr ) => {
        {
            use std::cell::RefCell;
            $ctx.pin(RefCell::new(Box::new($x)))
        }
    };
}

pub struct AllocRef<T> {
    ptr: *mut T,
}

impl<T> PartialEq for AllocRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for AllocRef<T> { }

impl<T> fmt::Debug for AllocRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AllocRef({:p})", self.ptr)
    }
}

impl<T> Hash for AllocRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl<T> Clone for AllocRef<T> {
    fn clone(&self) -> AllocRef<T> {
        AllocRef {
            ptr: self.ptr
        }
    }
}

impl<T> Deref for AllocRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T> DerefMut for AllocRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.ptr
        }
    }
}

pub struct AllocArena {
    arena: Vec<*mut RefCell<Box<Any>>>,
}

impl AllocArena {
    pub fn new() -> AllocArena {
        AllocArena {
            arena: vec![],
        }
    }

    pub fn pin<T: ?Sized>(&mut self, item: AllocInterior<T>) -> Alloc<T> {
        unsafe {
            self.arena.push(Box::into_raw(Box::new(item)) as *mut _);
            AllocRef {
                ptr: mem::transmute(*self.arena.last().unwrap()),
            }
        }
    }

    /// Kinda rough estimate for arena size.
    pub fn size(&self) -> usize {
        self.arena.len()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FuncFnId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Expr {
    Int(i32),
    Atom(String),
    SExpr(Alloc<Vec<Expr>>),
    Str(String),
    Null,
    TailCall(FuncFnId, Vec<Expr>),
    Func(Alloc<FuncFn>),
    Macro(Alloc<MacroFn>),
}

impl Expr {
    pub fn from_ast(ctx: &mut Context, ast: &Ast) -> Expr {
        match ast {
            &Ast::Int(value) => Expr::Int(value),
            &Ast::Atom(ref value) => Expr::Atom(value.clone()),
            &Ast::SExpr(ref value) => {
                let exprs: Vec<Expr> = value.iter().map(|x| {
                    Expr::from_ast(ctx, x)
                }).collect();
                Expr::SExpr(alloc!(ctx, exprs))
            }
            &Ast::Str(ref value) => Expr::Str(value.clone()),
            &Ast::Null => Expr::Null,
        }
    }

    pub fn new_atom(key: &str) -> Expr {
        Expr::Atom(key.to_owned())
    }

    pub fn as_vec<'a>(&'a self) -> Ref<'a, Box<Vec<Expr>>> {
        match self {
            &Expr::SExpr(ref inner) => inner.borrow(),
            _ => unreachable!(),
        }
    }

    pub fn as_vec_mut<'a>(&'a mut self) -> RefMut<'a, Box<Vec<Expr>>> {
        match self {
            &mut Expr::SExpr(ref mut inner) => inner.borrow_mut(),
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

pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
pub type MacroFn = Fn(&mut Context, ScopeRef, Vec<Expr>) -> Expr;

pub type ScopeRef = Alloc<Scope>;

pub struct Context {
    pub callstack: Vec<(FuncFnId, bool)>,
    pub alloc: AllocArena,
}

impl Context {
    pub fn new() -> Context {
        Context {
            callstack: vec![],
            alloc: AllocArena::new(),
        }
    }

    pub fn pin<T: ?Sized>(&mut self, item: AllocInterior<T>) -> Alloc<T> {
        self.alloc.pin(item)
    }
}

pub fn funcfn_id(closure: &Alloc<FuncFn>) -> FuncFnId {
    let ref boxed_fn: Box<FuncFn> = *closure.borrow();
    FuncFnId(format!("{:p}", &*boxed_fn))
}

pub struct Scope {
    parent: Option<ScopeRef>,
    scope: HashMap<Expr, Expr>,
}

impl Scope {
    pub fn new(ctx: &mut Context, parent: Option<ScopeRef>) -> ScopeRef {
        alloc!(ctx, Scope {
            parent: parent,
            scope: HashMap::new(),
        })
    }

    pub fn set(&mut self, key: Expr, value: Expr) -> Option<Expr> {
        self.scope.insert(key, value)
    }

    pub fn set_atom(&mut self, key: &str, value: Expr) -> Option<Expr> {
        self.scope.insert(Expr::Atom(key.to_owned()), value)
    }

    pub fn lookup<F, T>(&self, key: &Expr, mut inner: F) -> Option<T>
        where F: FnMut(Option<&Expr>) -> T
    {
        match self.scope.get(key) {
            Some(ref value) => Some(inner(Some(value))),
            None => {
                match self.parent {
                    Some(ref parent) => parent.borrow().lookup(key, inner),
                    None => None,
                }
            }
        }
    }
}

pub fn eval_expr(ctx: &mut Context, scope: ScopeRef, x: Expr, args: Vec<Expr>) -> Expr {
    use self::Expr::*;

    match x {
        Atom(..) => {
            let (func, mac, do_eval) = scope.borrow()
                .lookup(&x, |value| {
                    match value {
                        Some(&Expr::Func(ref func)) => {
                            (Some(func.clone()), None, true)
                        }
                        Some(&Expr::Macro(ref func)) => {
                            (None, Some(func.clone()), false)
                        }
                        Some(ref value) => {
                            panic!("Called uncallable value: {:?}", value);
                        }
                        _ => {
                            panic!("Called value that doesn't exist");
                        }
                    }
                })
                .expect(&format!("Could not eval unknown atom {:?}", x));

            ctx.callstack.push((FuncFnId("0x0".to_owned()), false));
            let args: Vec<Expr> = args.into_iter()
                .map(|x| if do_eval {
                    eval(ctx, scope.clone(), x)
                } else {
                    x
                })
                .collect();
            ctx.callstack.pop();

            if let Some(func) = func {
                let call = func.borrow();
                call(ctx, args)
            } else if let Some(mac) = mac {
                let call = mac.borrow();
                call(ctx,scope, args)
            } else {
                Expr::Null
            }
        }
        _ => unreachable!(),
    }
}

pub fn eval(ctx: &mut Context, scope: ScopeRef, expr: Expr) -> Expr {
    match expr {
        Expr::SExpr(args) => {
            let mut args: Vec<Expr> = {
                (**args.borrow_mut()).clone()
            };
            let term = args.remove(0);
            eval_expr(ctx, scope, term, args)
        }
        Expr::Atom(..) => {
            scope.borrow()
                .lookup(&expr, |x| {
                    if let Some(inner) = x {
                        inner.clone()
                    } else {
                        unreachable!("Cannot evaluate value {:?}", expr);
                    }
                })
                .expect(&format!("Eval failed to find named value: {:?}", expr))
        }
        _ => expr,
    }
}
