// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm

extern crate rand;

pub mod lisp;
pub mod ast;
#[macro_use] pub mod scope;

use ast::*;
use scope::*;
use std::io::{self, Read};
use rand::Rng;

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
    scope.borrow_mut().set(key, ScopeValue::Expr(value));
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

fn eval_first(_: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);
    value.as_vec()[0].clone()
}

fn eval_rest(_: ScopeRef, mut args: Vec<Expr>) -> Expr {
    args.remove(0);
    Expr::SExpr(args)
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
            s2.borrow_mut().set((*item).clone(), ScopeValue::Expr(value.clone()));
        }

        let mut res = Expr::Null;
        for statement in content.iter() {
            res = eval(s2.clone(), statement.clone(), eval_expr);
        }
        res
    });

    scope.borrow_mut().set(key, ScopeValue::Func(closure));
    Expr::Null
}

fn eval_nullq(_: ScopeRef, args: Vec<Expr>) -> Expr {
    match &args[0] {
        &Expr::Null => Expr::Int(1),
        _ => Expr::Int(0),
    }
}

fn eval_println(_: ScopeRef, args: Vec<Expr>) -> Expr {
    println!("{:?}", args);
    Expr::Null
}

fn truthy(expr: &Expr) -> bool {
    match expr {
        &Expr::Int(0) | &Expr::Null => false,
        _ => true,
    }
}

fn eval_if(scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let if_val = args.remove(0);
    let then_val = args.remove(0);
    let else_val = args.remove(0);

    if truthy(&eval(scope.clone(), if_val, eval_expr)) {
        eval(scope.clone(), then_val, eval_expr)
    } else {
        eval(scope.clone(), else_val, eval_expr)
    }
}

fn eval_eq(_: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let a = args.remove(0);
    let b = args.remove(0);

    if a == b {
        Expr::Int(1)
    } else {
        Expr::Int(0)
    }
}

fn eval_concat(_: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let mut list = args.remove(0);
    let add = args.remove(0);

    list.as_vec_mut().push(add);
    list
}

fn eval_random(_: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let n = args.remove(0);

    let mut rng = rand::thread_rng();
    Expr::Int(rng.gen_range(0, n.as_int()))
}

fn main() {
    let _ = run();
}

fn run() -> io::Result<()> {
    let mut content = String::new();
    try!(io::stdin().read_to_string(&mut content));

    let parse = lisp::parse_Exprs(&content).unwrap();

    let s = Scope::new(None);
    {
        let mut s = s.borrow_mut();
        s.set_atom("+", ScopeValue::Func(alloc!(eval_add)));
        s.set_atom("-", ScopeValue::Func(alloc!(eval_sub)));
        s.set_atom("*", ScopeValue::Func(alloc!(eval_mul)));
        s.set_atom("/", ScopeValue::Func(alloc!(eval_div)));
        s.set_atom("def", ScopeValue::Macro(alloc!(eval_def)));
        s.set_atom("defn", ScopeValue::Macro(alloc!(eval_defn)));
        s.set_atom("vec", ScopeValue::Func(alloc!(eval_vec)));
        s.set_atom("index", ScopeValue::Func(alloc!(eval_index)));
        s.set_atom("first", ScopeValue::Func(alloc!(eval_first)));
        s.set_atom("rest", ScopeValue::Func(alloc!(eval_rest)));
        s.set_atom("null?", ScopeValue::Func(alloc!(eval_nullq)));
        s.set_atom("println", ScopeValue::Func(alloc!(eval_println)));
        s.set_atom("if", ScopeValue::Macro(alloc!(eval_if)));
        s.set_atom("=", ScopeValue::Func(alloc!(eval_eq)));
        s.set_atom("concat", ScopeValue::Func(alloc!(eval_concat)));
        s.set_atom("random", ScopeValue::Func(alloc!(eval_random)));
    }

    let mut res = Expr::Null;
    for statement in parse {
        res = eval(s.clone(), statement, eval_expr);
    }

    println!("{:?}", res);

    Ok(())
}
