// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm

pub mod lisp;
pub mod ast;
#[macro_use] pub mod scope;

use ast::*;
use scope::*;
use std::env;

fn eval_add(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a + b,
        _ => 0
    })
}

fn eval_sub(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a - b,
        _ => 0
    })
}

fn eval_mul(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a * b,
        _ => 0
    })
}

fn eval_div(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a / b,
        _ => 0
    })
}

fn eval_def(scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let value = args.remove(0);
    scope.borrow_mut().set(key, ScopeValue::ExprValue(value));
    Expr::Null
}

fn eval_vec(_: ScopeRef, args: Vec<Expr>) -> Expr {
    Expr::SExpr(args)
}

fn eval_index(_: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);
    let key = args.remove(0);
    value.as_vec()[key.as_int() as usize].clone()
}

fn eval_defn(scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let names = if let Expr::SExpr(content) = args.remove(0) {
        content
    } else {
        vec![]
    };

    let content = args;
    let parent_scope = scope.clone();
    let closure: Alloc<ExprFn> = alloc!(move |_, args: Vec<Expr>| {
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

    scope.borrow_mut().set(key, ScopeValue::FuncValue(closure));
    Expr::Null
}

fn main() {
    let content = env::args().nth(1).unwrap();
    let parse = lisp::parse_Exprs(&content).unwrap();

    let s = Scope::new(None);
    {
        let mut s = s.borrow_mut();
        s.set(Expr::new_atom("true"), ScopeValue::ExprValue(Expr::Int(1)));
        s.set(Expr::new_atom("+"), ScopeValue::new_fn(eval_add));
        s.set(Expr::new_atom("-"), ScopeValue::new_fn(eval_sub));
        s.set(Expr::new_atom("*"), ScopeValue::new_fn(eval_mul));
        s.set(Expr::new_atom("/"), ScopeValue::new_fn(eval_div));
        s.set(Expr::new_atom("def"), ScopeValue::new_macro(eval_def));
        s.set(Expr::new_atom("defn"), ScopeValue::new_macro(eval_defn));
        s.set(Expr::new_atom("vec"), ScopeValue::new_fn(eval_vec));
        s.set(Expr::new_atom("index"), ScopeValue::new_fn(eval_index));
    }

    let mut res = Expr::Int(-1);
    for statement in parse {
        res = eval(s.clone(), statement, eval_expr);
    }

    println!("{:?}", res);
}
