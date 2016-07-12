use ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Alloc<T> = Rc<RefCell<Box<T>>>;

/// Allocate objects.
macro_rules! alloc {
    ( $x:expr ) => {
        {
            use std::cell::RefCell;
            use std::rc::Rc;
            Rc::new(RefCell::new(Box::new($x)))
        }
    };
}

pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
pub type MacroFn = Fn(&mut Context, ScopeRef, Vec<Expr>) -> Expr;

pub type ScopeRef = Alloc<Scope>;

pub type Context = Vec<FuncFnId>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncFnId(String);

pub fn create_callstack() -> Context {
    vec![]
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
    pub fn new(parent: Option<ScopeRef>) -> ScopeRef {
        alloc!(Scope {
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
                        Some(&ScopeValue::Func(ref func)) => (Some(func.clone()), None, true),
                        Some(&ScopeValue::Macro(ref func)) => (None, Some(func.clone()), false),
                        Some(&ScopeValue::Expr(ref value)) => {
                            panic!("Called uncallable value: {:?}", value);
                        }
                        _ => {
                            panic!("Called value that doesn't exist");
                        }
                    }
                })
                .expect(&format!("Could not eval unknown atom {:?}", x));

            let args: Vec<Expr> = args.into_iter()
                .map(|x| if do_eval {
                    eval(ctx, scope.clone(), x)
                } else {
                    x
                })
                .collect();

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
