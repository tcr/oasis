// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm

pub mod lisp;
pub mod ast;

use ast::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct Scope {
    parent: Option<Rc<RefCell<Box<Scope>>>>,
    scope: HashMap<Expr, Expr>,
}

impl Scope {
    fn new(parent: Option<Rc<RefCell<Box<Scope>>>>) -> Rc<RefCell<Box<Scope>>> {
        Rc::new(RefCell::new(Box::new(Scope {
            parent: parent,
            scope: HashMap::new()
        })))
    }

    pub fn lookup<F, T>(&self, key: &Expr, mut inner: F) -> Option<T>
    where F: FnMut(Option<&Expr>) -> T {
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

fn eval_add(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a + b,
        _ => 0
    })
}

static EVAL_ADD: fn(Expr, Expr) -> Expr = eval_add;

fn eval_sub(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a - b,
        _ => 0
    })
}

static EVAL_SUB: fn(Expr, Expr) -> Expr = eval_sub;

fn eval_mul(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a * b,
        _ => 0
    })
}

static EVAL_MUL: fn(Expr, Expr) -> Expr = eval_mul;

fn eval_div(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a / b,
        _ => 0
    })
}

static EVAL_DIV: fn(Expr, Expr) -> Expr = eval_div;

fn eval_expr(scope: &Scope, x: Expr, args: Vec<Box<Expr>>) -> Expr {
    use ast::Expr::*;
    let mut args: Vec<Expr> = args
        .into_iter()
        .map(|x| scope.eval(*x, eval_expr))
        .collect();

    match x {
        Atom(..) => {
            scope.lookup(&x, move |value| {
                match value {
                    Some(value) => {
                        value.call(args.remove(0), args.remove(0))
                    }
                    None => unreachable!(),
                }
            }).unwrap()
        },
        _ => unreachable!(),
    }
}

impl Scope {
    pub fn eval<F>(&self, expr: Expr, inner: F) -> Expr
    where F: Fn(&Scope, Expr, Vec<Box<Expr>>) -> Expr {
        match expr {
            Expr::SExpr(mut args) => {
                let term = args.remove(0);
                inner(self, *term, args)
            },
            _ => expr,
        }
    }
}

fn main() {
    let mut parse = lisp::parse_Exprs("(+ (* 22 44) 66)").unwrap();

    let s = Scope::new(None);
    let s2 = Scope::new(Some(s.clone()));
    {
        let mut s = s.borrow_mut();
        s.scope.insert(Expr::Atom("true".to_owned()), Expr::Int(1));
        s.scope.insert(Expr::Atom("+".to_owned()), Expr::Func(FnHolder::new(&EVAL_ADD as &'static _)));
        s.scope.insert(Expr::Atom("-".to_owned()), Expr::Func(FnHolder::new(&EVAL_SUB as &'static _)));
        s.scope.insert(Expr::Atom("*".to_owned()), Expr::Func(FnHolder::new(&EVAL_MUL as &'static _)));
        s.scope.insert(Expr::Atom("/".to_owned()), Expr::Func(FnHolder::new(&EVAL_DIV as &'static _)));
    }
    s2.borrow().lookup(&Expr::Atom("true".to_owned()), |expr| {
        println!("lookup {:?}", expr);
    });

    let res = s2.borrow().eval(*parse.remove(0), eval_expr);

    println!("{:?}", res);
}