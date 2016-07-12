use ast::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

pub type ExprFn = Fn(ScopeRef, Vec<Expr>) -> Expr;

pub type ScopeRef = Alloc<Scope>;

pub enum ScopeValue {
    FuncValue(Alloc<Fn(ScopeRef, Vec<Expr>) -> Expr>),
    MacroValue(Alloc<Fn(ScopeRef, Vec<Expr>) -> Expr>),
    ExprValue(Expr),
}

impl ScopeValue {
    pub fn new_fn<F>(f: F) -> ScopeValue
    where F: Fn(ScopeRef, Vec<Expr>) -> Expr + 'static {
        ScopeValue::FuncValue(alloc!(f))
    }

    pub fn new_macro<F>(f: F) -> ScopeValue
    where F: Fn(ScopeRef, Vec<Expr>) -> Expr + 'static {
        ScopeValue::MacroValue(alloc!(f))
    }
}

pub struct Scope {
    parent: Option<ScopeRef>,
    scope: HashMap<Expr, ScopeValue>,
}

impl Scope {
    pub fn new(parent: Option<ScopeRef>) -> ScopeRef {
        alloc!(Scope {
            parent: parent,
            scope: HashMap::new()
        })
    }

    pub fn set(&mut self, key: Expr, value: ScopeValue) -> Option<ScopeValue> {
        self.scope.insert(key, value)
    }

    pub fn lookup<F, T>(&self, key: &Expr, mut inner: F) -> Option<T>
    where F: FnMut(Option<&ScopeValue>) -> T {
        match self.scope.get(key) {
            Some(ref value) => {
                Some(inner(Some(value)))
            }
            None => {
                match self.parent {
                    Some(ref parent) => {
                        parent.borrow().lookup(key, inner)
                    }
                    None => None,
                }
            }
        }
    }
}

pub fn eval_expr(scope: ScopeRef, x: Expr, args: Vec<Expr>) -> Expr {
    use ast::Expr::*;

    match x {
        Atom(..) => {
            let (func, do_eval) = scope.borrow().lookup(&x, |value| {
                match value {
                    Some(&ScopeValue::FuncValue(ref func)) => {
                        (func.clone(), true)
                    }
                    Some(&ScopeValue::MacroValue(ref func)) => {
                        (func.clone(), false)
                    }
                    Some(&ScopeValue::ExprValue(ref value)) => {
                        panic!("Called uncallable value: {:?}", value);
                    }
                    _ => {
                        panic!("Called value that doesn't exist");
                    }
                }
            }).expect("Could not eval unknown atom");

            let args: Vec<Expr> = args
                .into_iter()
                .map(|x| if do_eval {
                    eval(scope.clone(), x, eval_expr)
                } else {
                    x
                })
                .collect();

            let call = func.borrow();
            call(scope, args)
        },
        _ => unreachable!(),
    }
}

pub fn eval<F>(scope: ScopeRef, expr: Expr, inner: F) -> Expr
where F: Fn(ScopeRef, Expr, Vec<Expr>) -> Expr {
    match expr {
        Expr::SExpr(mut args) => {
            let term = args.remove(0);
            inner(scope, term, args)
        },
        Expr::Atom(..) => {
            scope.borrow().lookup(&expr, |x| {
                if let Some(&ScopeValue::ExprValue(ref inner)) = x {
                    inner.clone()
                } else {
                    unreachable!("Cannot evaluate value {:?}", expr);
                }
            }).expect("Eval failed to find named value")
        },
        _ => expr,
    }
}
