// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm

pub mod lisp;
pub mod ast;
pub mod scope;

use ast::*;
use scope::*;
use std::cell::RefCell;
use std::rc::Rc;
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

fn eval_vec(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::SExpr(args)
}

static EVAL_VEC: fn(ScopeRef, Vec<Expr>) -> Expr = eval_vec;

fn eval_index(_: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);
    let key = args.remove(0);
    value.as_vec()[key.as_int() as usize].clone()
}

static EVAL_INDEX: fn(ScopeRef, Vec<Expr>) -> Expr = eval_index;

fn eval_defn(scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let names = if let Expr::SExpr(content) = args.remove(0) {
        content
    } else {
        vec![]
    };

    let content = args;
    let parent_scope = scope.clone();
    let closure = Box::new(move |_, args: Vec<Expr>| {
        let s2 = Scope::new(Some(parent_scope.clone()));
        for (item, value) in names.iter().zip(args) {
            s2.borrow_mut().set((*item).clone(), ScopeValue::ExprValue(value.clone()));
        }

        let mut res = Expr::Null;
        for statement in content.iter() {
            res = eval(s2.clone(), statement.clone(), eval_expr);
        }
        res
    });

    scope.borrow_mut().set(key, ScopeValue::DynFuncValue(Rc::new(RefCell::new(closure))));
    Expr::Null
}

static EVAL_DEFN: fn(ScopeRef, Vec<Expr>) -> Expr = eval_defn;

fn main() {
    let content = env::args().nth(1).unwrap();
    let parse = lisp::parse_Exprs(&content).unwrap();

    let s = Scope::new(None);
    {
        let mut s = s.borrow_mut();
        s.set(Expr::new_atom("true"), ScopeValue::ExprValue(Expr::Int(1)));
        s.set(Expr::new_atom("+"), ScopeValue::FuncValue(&EVAL_ADD));
        s.set(Expr::new_atom("-"), ScopeValue::FuncValue(&EVAL_SUB));
        s.set(Expr::new_atom("*"), ScopeValue::FuncValue(&EVAL_MUL));
        s.set(Expr::new_atom("/"), ScopeValue::FuncValue(&EVAL_DIV));
        s.set(Expr::new_atom("def"), ScopeValue::MacroValue(&EVAL_DEF));
        s.set(Expr::new_atom("defn"), ScopeValue::MacroValue(&EVAL_DEFN));
        s.set(Expr::new_atom("vec"), ScopeValue::FuncValue(&EVAL_VEC));
        s.set(Expr::new_atom("index"), ScopeValue::FuncValue(&EVAL_INDEX));
    }

    let mut res = Expr::Int(-1);
    for statement in parse {
        res = eval(s.clone(), statement, eval_expr);
    }

    println!("{:?}", res);
}
