use ast::*;
use alloc::*;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::{RefCell, Ref, RefMut, BorrowState};
use std::collections::HashMap;
use ctrie::hamt::HAMT;
use std::ops::Index;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FuncFnId(pub String);

pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
pub type SpecialFn = Fn(&mut Context, Alloc, Vec<Expr>) -> Expr;

pub struct FuncInner {
    pub body: Box<FuncFn>,
    pub scope: Alloc,
}

#[derive(Clone)]
pub struct VecObject<T: Sized + Clone> {
    pub inner: HAMT<usize, RefCell<T>>, //TODO not pub
    length: Arc<AtomicUsize>,
}

impl<T: Sized + Clone> VecObject<T> {
    pub fn new() -> VecObject<T> {
        VecObject {
            inner: HAMT::new(),
            length: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn len(&self) -> usize {
        self.length.load(Ordering::Relaxed)
    }

    pub fn new_from(mut input: Vec<T>) -> VecObject<T> {
        let mut vec = VecObject::new();
        let len = input.len();
        for i in 0..len {
            vec.inner.insert(i, RefCell::new(input.remove(0)));
        }
        vec.length.store(len, Ordering::Relaxed);
        vec
    }

    pub fn get<F: Fn(&RefCell<T>) -> R, R>(&self, key: usize, callback: F) -> Option<R> {
        self.inner.search(&key, callback)
    }

    pub fn push(&mut self, item: T) {
        let pos = self.length.fetch_add(1, Ordering::Relaxed);
        self.inner.insert(pos, RefCell::new(item));
    }

    pub fn pop(&mut self) {
        if self.length.load(Ordering::Relaxed) > 0 {
            let new_len = self.length.fetch_sub(1, Ordering::Relaxed) - 1;
            self.inner.remove(new_len);
        }
    }
}

pub enum GcMem {
    VecMem(VecObject<Expr>),
    FuncMem(FuncInner),
    SpecialMem(Box<SpecialFn>),
    ScopeMem(Scope),
    Deallocated,
}

impl fmt::Debug for GcMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GcMem::VecMem(..) => write!(f, "VecMem({:p})", self),
            &GcMem::FuncMem(..) => write!(f, "FuncMem({:p})", self),
            &GcMem::SpecialMem(..) => write!(f, "SpecialMem({:p})", self),
            &GcMem::ScopeMem(..) => write!(f, "ScopeMem({:p})", self),
            &GcMem::Deallocated => write!(f, "**DEALLOCATED**({:p})", self),
        }
    }
}

impl GcMem {
    pub fn as_vec(&self) -> &VecObject<Expr> {
        match self {
            &GcMem::VecMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_vec_mut(&mut self) -> &mut VecObject<Expr> {
        match self {
            &mut GcMem::VecMem(ref mut inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_func(&self) -> &FuncInner {
        match self {
            &GcMem::FuncMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_special(&self) -> &Box<SpecialFn> {
        match self {
            &GcMem::SpecialMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_scope(&mut self) -> &mut Scope {
        match self {
            &mut GcMem::ScopeMem(ref mut inner) => inner,
            _ => panic!("Cannot dereference {:?}", self),
        }
    }

    pub fn wrap_fn(target: Box<FuncFn>, scope: Alloc) -> GcMem {
        GcMem::FuncMem(FuncInner {
            body: target,
            scope: scope,
        })
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
    Vec(Alloc),
    Func(Alloc),
    Special(Alloc),
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

    pub fn as_vec<'a>(&'a self) -> Ref<'a, VecObject<Expr>> {
        match self {
            &Expr::Vec(ref inner) => {
                Ref::map(inner.borrow(), |x| {
                    x.as_vec()
                })
            }
            _ => panic!("Attempted to use {:?} as vec", self),
        }
    }

    pub fn as_vec_mut<'a>(&'a mut self) -> RefMut<'a, VecObject<Expr>> {
        match self {
            &mut Expr::Vec(ref mut inner) => {
                RefMut::map(inner.borrow_mut(), |x| {
                    x.as_vec_mut()
                })
            }
            _ => panic!("Attempted to use {:?} as mutable vec", self),
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

    pub fn get_mem(&self) -> Option<&Alloc> {
        match self {
            &Expr::Vec(ref inner) => Some(inner),
            &Expr::Func(ref inner) => Some(inner),
            &Expr::Special(ref inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ContextState {
    pub roots: VecObject<Alloc>,
}

pub struct Context {
    pub callstack: Vec<(FuncFnId, bool)>,
    pub alloc: Arc<RwLock<AllocArena>>,
    pub state: ContextState,
}

impl Context {
    pub fn new() -> Context {
        Context {
            callstack: vec![],
            alloc: Arc::new(RwLock::new(AllocArena::new())),
            state: ContextState {
                roots: VecObject::new(),
            }
        }
    }

    pub fn mark_roots(&mut self) {
        let len = self.state.roots.len();
        for i in 0..len {
            self.state.roots.get(i, |value| {
                AllocArena::mark_refcell(value);
            });
        }
    }
}

pub fn funcfn_id(closure: &Alloc) -> FuncFnId {
    FuncFnId(closure.id())
}

pub struct Scope {
    pub parent: Option<Alloc>,
    pub scope: HAMT<Expr, RefCell<Expr>>,
}

impl Scope {
    pub fn new(ctx: &mut Context, parent: Option<Alloc>) -> Alloc {
        alloc!(ctx, GcMem::ScopeMem(Scope {
            parent: parent,
            scope: HAMT::new(),
        }))
    }

    pub fn set(&mut self, key: Expr, value: Expr) {
        self.scope.insert(key, RefCell::new(value));
    }

    pub fn set_atom(&mut self, key: &str, value: Expr) {
        self.scope.insert(Expr::Atom(key.to_owned()), RefCell::new(value));
    }

    pub fn lookup<F, T>(&mut self, key: &Expr, mut inner: F) -> Option<T>
        where F: Fn(Option<&Expr>) -> T
    {
        if let Some(value) = self.scope.search(key, |value| {
            inner(Some(&*value.borrow()))
        }) {
            Some(value)
        } else {
            match self.parent {
                Some(ref parent) => {
                    parent.borrow_mut().as_scope().lookup(key, inner)
                }
                None => None,
            }
        }
    }
}

pub fn eval_expr(ctx: &mut Context, scope: Alloc, x: Expr, args: Vec<Expr>) -> Expr {
    match x {
        Expr::Atom(..) => {
            let (func, special): (Option<AllocRef<_>>, Option<AllocRef<_>>) = scope
                .borrow_mut()
                .as_scope()
                .lookup(&x, |value| {
                    match value {
                        Some(&Expr::Func(ref func)) => {
                            AllocArena::mark(func);
                            (Some(func.clone()), None)
                        }
                        Some(&Expr::Special(ref func)) => {
                            (None, Some(func.clone()))
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

            ctx.state.roots.push(scope.clone());

            ctx.callstack.push((FuncFnId("0x0".to_owned()), false));
            let args: Vec<Expr> = args.into_iter()
                .map(|x| if func.is_some() {
                    eval(ctx, scope.clone(), x)
                } else {
                    x
                })
                .collect();
            ctx.callstack.pop();

            let ret = if let Some(func) = func {
                let call = func.borrow();
                let call = call.as_func();
                let call = &call.body;
                call(ctx, args)
            } else if let Some(special) = special {
                let call = special.borrow();
                let call = call.as_special();
                call(ctx, scope, args)
            } else {
                Expr::Null
            };

            ctx.state.roots.pop();
            ret
        }
        _ => {
            panic!("Attempted to evaluate non-atom: {:?}", x);
        }
    }
}

pub fn eval(ctx: &mut Context, scope: Alloc, expr: Expr) -> Expr {
    match expr {
        Expr::List(args) => {
            let mut args = args.clone();
            let term = args.remove(0);
            eval_expr(ctx, scope, term, args)
        }
        Expr::Atom(..) => {
            scope.borrow_mut()
                .as_scope()
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
