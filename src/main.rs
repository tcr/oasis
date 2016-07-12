// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm

pub mod lisp;
pub mod ast;
pub mod scope;

use ast::*;
use scope::*;
use std::env;

fn eval_add(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a + b,
        _ => 0
    })
}

static EVAL_ADD: fn(ScopeRef, Vec<Expr>) -> Expr = eval_add;

fn eval_sub(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a - b,
        _ => 0
    })
}

static EVAL_SUB: fn(ScopeRef, Vec<Expr>) -> Expr = eval_sub;

fn eval_mul(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a * b,
        _ => 0
    })
}

static EVAL_MUL: fn(ScopeRef, Vec<Expr>) -> Expr = eval_mul;

fn eval_div(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a / b,
        _ => 0
    })
}

static EVAL_DIV: fn(ScopeRef, Vec<Expr>) -> Expr = eval_div;

fn eval_def(scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let value = args.remove(0);
    scope.borrow_mut().set(key, ScopeValue::ExprValue(value));
    Expr::Null
}

static EVAL_DEF: fn(ScopeRef, Vec<Expr>) -> Expr = eval_def;

fn eval_defn(scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let names = args.remove(0);
    let s2 = Scope::new(Some(scope.clone()));
    let closure = |scope, args| {
        println!("called!");
        Expr::Null
    };
    scope.borrow_mut().set(key, ScopeValue::DynFuncValue(Box::new(closure)));
    Expr::Null
}

static EVAL_DEFN: fn(ScopeRef, Vec<Expr>) -> Expr = eval_defn;

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
        s.set(Expr::new_atom("def"), ScopeValue::MacroValue(&EVAL_DEF));
        s.set(Expr::new_atom("defn"), ScopeValue::MacroValue(&EVAL_DEFN));
    }
    //s2.borrow().lookup(&Expr::Atom("true".to_owned()), |expr| {
    //    println!("lookup {:?}", expr);
    //});

    let mut res = Expr::Int(-1);
    for statement in parse {
        res = eval(s2.clone(), *statement, eval_expr);
    }

    println!("{:?}", res);
}
