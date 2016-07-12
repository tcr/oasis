use ast::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

// Switch to Funcs in Scope

pub type ScopeRef = Rc<RefCell<Box<Scope>>>;

pub enum ScopeValue {
    FuncValue(&'static Fn(ScopeRef, Vec<Expr>) -> Expr),
    MacroValue(&'static Fn(ScopeRef, Vec<Expr>) -> Expr),
    DynFuncValue(Rc<RefCell<Box<Fn(ScopeRef, Vec<Expr>) -> Expr>>>),
    ExprValue(Expr),
}

pub struct Scope {
    parent: Option<ScopeRef>,
    scope: HashMap<Expr, ScopeValue>,
}

impl Scope {
    pub fn new(parent: Option<ScopeRef>) -> ScopeRef {
        Rc::new(RefCell::new(Box::new(Scope {
            parent: parent,
            scope: HashMap::new()
        })))
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
            let (func, dynfunc, do_eval) = scope.borrow().lookup(&x, |value| {
                match value {
                    Some(&ScopeValue::FuncValue(func)) => {
                        (Some(func), None, true)
                    }
                    Some(&ScopeValue::MacroValue(func)) => {
                        (Some(func), None, false)
                    }
                    Some(&ScopeValue::ExprValue(ref value)) => {
                        panic!("Called uncallable value: {:?}", value);
                    }
                    Some(&ScopeValue::DynFuncValue(ref func)) => {
                        (None, Some(func.clone()), true)
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

            if let Some(func) = func {
                func(scope, args)
            } else if let Some(dynfunc) = dynfunc {
                let call = dynfunc.borrow();
                call(scope, args)
            } else {
                unreachable!();
            }
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
