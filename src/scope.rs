use ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::mem;
use std::ptr::Unique;
use std::any::Any;

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

pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
pub type MacroFn = Fn(&mut Context, ScopeRef, Vec<Expr>) -> Expr;

pub type ScopeRef = Alloc<Scope>;

pub struct Context {
    pub callstack: Vec<(FuncFnId, bool)>,
    pub alloc: Vec<Unique<Box<RefCell<Box<Any>>>>>,
}

pub struct AllocRef<T> {
    ptr: *mut T,
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

impl Context {
    pub fn new() -> Context {
        Context {
            callstack: vec![],
            alloc: vec![],
        }
    }

    pub fn pin<T: ?Sized>(&mut self, item: AllocInterior<T>) -> Alloc<T> {
        unsafe {
            self.alloc.push(mem::transmute(Unique::new(Box::into_raw(box item))));
            AllocRef {
                ptr: mem::transmute(self.alloc.last_mut().unwrap().get_mut()),
            }
        }
    }
}

pub fn funcfn_id(closure: &Alloc<FuncFn>) -> FuncFnId {
    let ref boxed_fn: Box<FuncFn> = *closure.borrow();
    FuncFnId(format!("{:p}", &*boxed_fn))
}

pub enum ScopeValue {
    Func(Alloc<FuncFn>),
    Macro(Alloc<MacroFn>),
    Expr(Expr),
}

pub struct Scope {
    parent: Option<ScopeRef>,
    scope: HashMap<Expr, ScopeValue>,
}

impl Scope {
    pub fn new(ctx: &mut Context, parent: Option<ScopeRef>) -> ScopeRef {
        alloc!(ctx, Scope {
            parent: parent,
            scope: HashMap::new(),
        })
    }

    pub fn set(&mut self, key: Expr, value: ScopeValue) -> Option<ScopeValue> {
        self.scope.insert(key, value)
    }

    pub fn set_atom(&mut self, key: &str, value: ScopeValue) -> Option<ScopeValue> {
        self.scope.insert(Expr::Atom(key.to_owned()), value)
    }

    pub fn lookup<F, T>(&self, key: &Expr, mut inner: F) -> Option<T>
        where F: FnMut(Option<&ScopeValue>) -> T
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
    use ast::Expr::*;

    match x {
        Atom(..) => {
            let (func, mac, do_eval) = scope.borrow()
                .lookup(&x, |value| {
                    match value {
                        Some(&ScopeValue::Func(ref func)) => {
                            (Some(func.clone()), None, true)
                        }
                        Some(&ScopeValue::Macro(ref func)) => {
                            (None, Some(func.clone()), false)
                        }
                        Some(&ScopeValue::Expr(ref value)) => {
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
        Expr::SExpr(mut args) => {
            let term = args.remove(0);
            eval_expr(ctx, scope, term, args)
        }
        Expr::Atom(..) => {
            scope.borrow()
                .lookup(&expr, |x| {
                    if let Some(&ScopeValue::Expr(ref inner)) = x {
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
