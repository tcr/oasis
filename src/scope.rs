use ast::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

// Switch to Funcs in Scope

#[derive(Clone)]
pub enum ScopeValue {
    FuncValue(&'static Fn(&mut Scope, Expr, Expr) -> Expr),
    ExprValue(Expr),
}

pub struct Scope {
    parent: Option<Rc<RefCell<Box<Scope>>>>,
    scope: HashMap<Expr, ScopeValue>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Box<Scope>>>>) -> Rc<RefCell<Box<Scope>>> {
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

pub fn eval_expr(scope: &mut Scope, x: Expr, args: Vec<Box<Expr>>) -> Expr {
    use ast::Expr::*;
    let mut args: Vec<Expr> = args
        .into_iter()
        .map(|x| scope.eval(*x, eval_expr))
        .collect();

    match x {
        Atom(..) => {
            let (func, a, b) = scope.lookup(&x, move |value| {
                match value {
                    Some(&ScopeValue::FuncValue(func)) => {
                        (func, args.remove(0), args.remove(0))
                    }
                    Some(&ScopeValue::ExprValue(ref value)) => {
                        panic!("Called uncallable value: {:?}", value);
                    }
                    None => {
                        panic!("Called value that doesn't exist");
                    }
                }
            }).unwrap();

            func(scope, a, b)
        },
        _ => unreachable!(),
    }
}

impl Scope {
    pub fn eval<F>(&mut self, expr: Expr, inner: F) -> Expr
    where F: Fn(&mut Scope, Expr, Vec<Box<Expr>>) -> Expr {
        match expr {
            Expr::SExpr(mut args) => {
                let term = args.remove(0);
                inner(self, *term, args)
            },
            Expr::Atom(..) => {
                if let Some(ScopeValue::ExprValue(inner)) = self.lookup(&expr, |x| {
                    x.expect("Eval failed to find named value").clone()
                }) {
                    inner
                } else {
                    unreachable!("Cannot evaluate value {:?}", expr);
                }
            },
            _ => expr,
        }
    }
}
