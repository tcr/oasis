use alloc::*;
use ast::*;
use gc::*;
use types::{OVec, OMap};
use std::fmt;
use std::sync::{Arc, RwLock};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FuncFnId(pub String);

pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
pub type SpecialFn = Fn(&mut Context, Gc, Vec<Expr>) -> Expr;

pub struct FuncInner {
    pub body: Box<FuncFn>,
    pub scope: Gc,
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

    pub fn wrap_fn(target: Box<FuncFn>, scope: Gc) -> Mem {
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
    Vec(Gc),
    Func(Gc),
    Special(Gc),
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

    //pub fn as_vec_mut<'a>(&'a mut self) -> RefMut<'a, OVec<Expr>> {
    //    match self {
    //        &mut Expr::Vec(ref mut alloc) => {
    //            x.as_vec_mut()
    //            })
    //        }
    //        _ => panic!("Attempted to use {:?} as mutable vec", self),
    //    }
    //}

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

    pub fn get_mem(&self) -> Option<&Gc> {
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
    pub roots: OVec<Gc>,
}

pub struct Context {
    pub callstack: Vec<(FuncFnId, bool)>,
    pub alloc: Arc<RwLock<GcArena>>,
    pub state: ContextState,
}

impl Context {
    pub fn new() -> Context {
        Context {
            callstack: vec![],
            alloc: Arc::new(RwLock::new(GcArena::new())),
            state: ContextState {
                roots: OVec::new(),
            }
        }
    }

    pub fn mark_roots(&mut self) {
        let len = self.state.roots.len();
        for i in 0..len {
            self.state.roots.get(i, |value| {
                GcArena::mark(value);
            });
        }
    }

    pub fn allocate(&self, value: Mem) -> Gc {
        self.alloc.write().unwrap().pin(GcRef::new(value))
    }
}

pub fn funcfn_id(closure: &Gc) -> FuncFnId {
    FuncFnId(closure.id())
}

pub struct Scope {
    pub parent: Option<Gc>,
    pub scope: OMap<Expr, Expr>,
}

impl Scope {
    pub fn new(ctx: &mut Context, parent: Option<Gc>) -> Gc {
        ctx.allocate(Mem::ScopeMem(Scope {
            parent: parent,
            scope: OMap::new(),
        }))
    }

    pub fn set(&self, key: Expr, value: Expr) {
        self.scope.insert(key, value.clone());

        // TODO probably cannot rely on this if detached from scope
        if let Some(ref mem) = value.get_mem() {
            // GC_ATTACH
            mem.set_rooted(true);
        }
    }

    pub fn set_atom(&self, key: &str, value: Expr) {
        self.set(Expr::Atom(key.to_owned()), value)
    }

    pub fn lookup<F, T>(&self, key: &Expr, inner: F) -> Option<T>
        where F: Fn(Option<&Expr>) -> T
    {
        if let Some(value) = self.scope.search(key, |value| {
            inner(Some(value))
        }) {
            Some(value)
        } else {
            match self.parent {
                Some(ref parent) => {
                    parent.get().as_scope().lookup(key, inner)
                }
                None => None,
            }
        }
    }
}

pub fn eval_expr(ctx: &mut Context, scope: Gc, x: Expr, args: Vec<Expr>) -> Expr {
    match x {
        Expr::Atom(..) => {
            let (func, special): (Option<AllocRef<_>>, Option<AllocRef<_>>) = scope.get()
                .as_scope()
                .lookup(&x, |value| {
                    match value {
                        Some(&Expr::Func(ref func)) => {
                            GcArena::mark(func);
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

            // TODO delete attachment to root?
            ctx.state.roots.push(scope.clone());
            //scope.set_completed(true)

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
                let call = func.get();
                let call = call.as_func();
                let call = &call.body;
                call(ctx, args)
            } else if let Some(special) = special {
                let call = special.get();
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

pub fn eval(ctx: &mut Context, scope: Gc, expr: Expr) -> Expr {
    match expr {
        Expr::List(args) => {
            let mut args = args.clone();
            let term = args.remove(0);
            eval_expr(ctx, scope, term, args)
        }
        Expr::Atom(..) => {
            //println!("why is scope scope {:?}", scope);
            scope.get()
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
