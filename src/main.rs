// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm

pub mod lisp;
pub mod ast;
pub mod scope;

use ast::*;
use scope::*;
use std::env;

fn eval_add(_: &mut Scope, a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a + b,
        _ => 0
    })
}

static EVAL_ADD: fn(&mut Scope, Expr, Expr) -> Expr = eval_add;

fn eval_sub(_: &mut Scope, a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a - b,
        _ => 0
    })
}

static EVAL_SUB: fn(&mut Scope, Expr, Expr) -> Expr = eval_sub;

fn eval_mul(_: &mut Scope, a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a * b,
        _ => 0
    })
}

static EVAL_MUL: fn(&mut Scope, Expr, Expr) -> Expr = eval_mul;

fn eval_div(_: &mut Scope, a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a / b,
        _ => 0
    })
}

static EVAL_DIV: fn(&mut Scope, Expr, Expr) -> Expr = eval_div;

fn eval_def(scope: &mut Scope, a: Expr, b: Expr) -> Expr {
    match a {
        Expr::Str(key) => {
            println!("way {:?}", key);
            scope.set(Expr::Atom(key), ScopeValue::ExprValue(b));
        }
        _ => unreachable!("Dont do that")
    }
    Expr::Null
}

static EVAL_DEF: fn(&mut Scope, Expr, Expr) -> Expr = eval_def;

fn main() {
    let content = env::args().nth(1).unwrap();
    let parse = lisp::parse_Exprs(&content).unwrap();

    let s = Scope::new(None);
    let s2 = Scope::new(Some(s.clone()));
    {
        let mut s = s.borrow_mut();
        s.set(Expr::new_atom("true"), ScopeValue::ExprValue(Expr::Int(1)));
        s.set(Expr::new_atom("+"), ScopeValue::FuncValue(&EVAL_ADD));
        s.set(Expr::new_atom("-"), ScopeValue::FuncValue(&EVAL_SUB));
        s.set(Expr::new_atom("*"), ScopeValue::FuncValue(&EVAL_MUL));
        s.set(Expr::new_atom("/"), ScopeValue::FuncValue(&EVAL_DIV));
        s.set(Expr::new_atom("def"), ScopeValue::FuncValue(&EVAL_DEF));
    }
    //s2.borrow().lookup(&Expr::Atom("true".to_owned()), |expr| {
    //    println!("lookup {:?}", expr);
    //});

    let mut res = Expr::Int(-1);
    for statement in parse {
        res = s2.borrow_mut().eval(*statement, eval_expr);
    }

    println!("{:?}", res);
}
