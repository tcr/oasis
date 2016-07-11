// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm

pub mod lisp;
pub mod ast;
pub mod scope;

use ast::*;
use scope::*;
use std::env;

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

fn main() {
    let content = env::args().nth(1).unwrap();
    let mut parse = lisp::parse_Exprs(&content).unwrap();

    let s = Scope::new(None);
    let s2 = Scope::new(Some(s.clone()));
    {
        let mut s = s.borrow_mut();
        s.set(Expr::new_atom("true"), ScopeValue::ExprValue(Expr::Int(1)));
        s.set(Expr::new_atom("+"), ScopeValue::FuncValue(&EVAL_ADD as &'static _));
        s.set(Expr::new_atom("-"), ScopeValue::FuncValue(&EVAL_SUB as &'static _));
        s.set(Expr::new_atom("*"), ScopeValue::FuncValue(&EVAL_MUL as &'static _));
        s.set(Expr::new_atom("/"), ScopeValue::FuncValue(&EVAL_DIV as &'static _));
    }
    //s2.borrow().lookup(&Expr::Atom("true".to_owned()), |expr| {
    //    println!("lookup {:?}", expr);
    //});

    let res = s2.borrow_mut().eval(*parse.remove(0), eval_expr);

    println!("{:?}", res);
}
