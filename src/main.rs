// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm
// TODO http://www.stefankrause.net/wp/?p=14

extern crate rand;
extern crate strfmt;

#[macro_use] pub mod scope;
pub mod ast;
pub mod lisp;

use rand::Rng;
use scope::*;
use std::collections::HashMap;
use std::io::{self, Read};
use std::mem;
use strfmt::strfmt;

fn special_def(ctx: &mut Context, scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let value = eval(ctx, scope.clone(), args.remove(0));
    scope.borrow_mut().set(key, value);
    Expr::Null
}

fn special_defn(ctx: &mut Context, scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    use std::rc::Rc;
    use std::sync::RwLock;

    let key = args.remove(0);
    let names: Vec<Expr> = if let Expr::SExpr(content) = args.remove(0) {
        (**content.borrow()).clone()
    } else {
        vec![]
    };

    let content = args;
    let parent_scope = scope.clone();
    let inner_ref: Rc<RwLock<Option<FuncFnId>>> = Rc::new(RwLock::new(None));
    let outer_ref = inner_ref.clone();

    let closure: Alloc<FuncFn> = alloc!(ctx, move |ctx: &mut Context, mut args: Vec<Expr>| {
        // Check for TCO.
        let fn_ptr = inner_ref.read().unwrap();
        let fn_id = fn_ptr.clone().expect("No FunFnId for this function.");
        assert!(args.iter()
            .all(|x| {
                match x {
                    &Expr::TailCall(..) => false,
                    _ => true
                }
            }), "Found tail call expr in args position");

        if ctx.callstack.iter().rev()
            .take_while(|x| x.1)
            .position(|x| x.0 == fn_id).is_some() {
            // Return early with evaluated arguments.
            return Expr::TailCall(fn_id, args);
        }

        // Otherwise, add to call stack and evaluate.
        let pos = ctx.callstack.len();
        ctx.callstack.push((fn_id.clone(), false));

        // Evaluate contents.
        let mut res = Expr::Null;
        loop {
            // We are not in tail-call position.
            ctx.callstack[pos].1 = false;

            // Create inner function bindings.
            let s2 = Scope::new(ctx, Some(parent_scope.clone()));
            for (item, value) in names.iter().zip(args) {
                s2.borrow_mut().set((*item).clone(), value.clone());
            }

            let len = content.len();
            for (i, statement) in content.iter().enumerate() {
                // When we are evaluating the last statement, change our Context
                // to indicate we are in terminal position
                if i + 1 == len {
                    ctx.callstack[pos].1 = true;
                }

                res = eval(ctx, s2.clone(), statement.clone());
            }

            // Evaluate tail call expressions if they match this function.
            if match res {
                Expr::TailCall(ref inner_fn_id, _) => {
                    *inner_fn_id == fn_id
                },
                _ => false,
            } {
                if let Expr::TailCall(_, inner_args) = mem::replace(&mut res, Expr::Null) {
                    args = inner_args;
                    continue;
                }
            }

            break;
        }

        ctx.callstack.pop();
        res
    });

    // Store unique closure ID.
    *outer_ref.write().unwrap() = Some(funcfn_id(&closure));

    scope.borrow_mut().set(key, Expr::Func(closure));
    Expr::Null
}

fn special_if(ctx: &mut Context, scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let if_val = args.remove(0);
    let then_val = args.remove(0);
    let else_val = args.remove(0);

    if eval(ctx, scope.clone(), if_val).as_bool() {
        eval(ctx, scope.clone(), then_val)
    } else {
        eval(ctx, scope.clone(), else_val)
    }
}

fn special_let(ctx: &mut Context, scope: ScopeRef, mut args: Vec<Expr>) -> Expr {
    let bindings = if let Expr::SExpr(content) = args.remove(0) {
        (**content.borrow()).clone()
    } else {
        vec![]
    };
    let content = args;

    let s2 = Scope::new(ctx, Some(scope.clone()));
    for win in bindings[..].chunks(2) {
        let item = win[0].clone();
        let value = win[1].clone();
        let value = eval(ctx, s2.clone(), value);
        s2.borrow_mut().set(item, value);
    }

    let mut res = Expr::Null;
    for statement in content.iter() {
        res = eval(ctx, s2.clone(), statement.clone());
    }
    res
}

fn eval_add(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(args.iter()
        .map(|x| x.as_int())
        .fold(0, |sum, val| sum + val))
}

fn eval_sub(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], args.iter().nth(1)) {
        (&Expr::Int(a), Some(&Expr::Int(b))) => a - b,
        (&Expr::Int(a), None) => -a,
        _ => 0,
    })
}

fn eval_mul(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a * b,
        _ => 0,
    })
}

fn eval_div(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a / b,
        _ => 0,
    })
}

fn eval_bitshiftleft(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a << b,
        _ => 0,
    })
}

fn eval_eq(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let a = args.remove(0);
    let b = args.remove(0);

    if a == b {
        Expr::Int(1)
    } else {
        Expr::Int(0)
    }
}

fn eval_le(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let a = args.remove(0);
    let b = args.remove(0);

    if a.as_int() < b.as_int() {
        Expr::Int(1)
    } else {
        Expr::Int(0)
    }
}

fn eval_vec(ctx: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::SExpr(alloc!(ctx, args))
}

fn eval_index(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);
    let key = args.remove(0);

    let value_vec = value.as_vec();
    value_vec[key.as_int() as usize].clone()
}

fn eval_first(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);

    let value_vec = value.as_vec();
    value_vec[0].clone()
}

fn eval_rest(ctx: &mut Context, mut args: Vec<Expr>) -> Expr {
    args.remove(0);
    Expr::SExpr(alloc!(ctx, args))
}

fn eval_nullq(_: &mut Context, args: Vec<Expr>) -> Expr {
    match &args[0] {
        &Expr::Null => Expr::Int(1),
        _ => Expr::Int(0),
    }
}

fn eval_println(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let fmt = args.remove(0).as_string();

    let mut vars = HashMap::new();
    for (i, value) in args.iter().enumerate() {
        vars.insert(format!("{}", i), value.as_string());
    }

    println!("{}", strfmt(&fmt, &vars).unwrap());
    Expr::Null
}

fn eval_concat(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let mut list = args.remove(0);
    let add = args.remove(0);

    list.as_vec_mut().push(add);
    list
}

fn eval_random(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let n = args.remove(0);

    let mut rng = rand::thread_rng();
    Expr::Int(rng.gen_range(0, n.as_int()))
}

fn eval_list(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let list = args.remove(0);

    let vec = list.as_vec();
    Expr::Int(vec.len() as i32)
}

fn main() {
    run().unwrap();
}

fn run() -> io::Result<()> {
    let mut content = String::new();
    try!(io::stdin().read_to_string(&mut content));

    let ast = lisp::parse_Exprs(&content).unwrap();

    let mut ctx = Context::new();
    let s = Scope::new(&mut ctx, None);
    {
        let mut s = s.borrow_mut();

        s.set_atom("def", Expr::Special(alloc!(ctx, special_def)));
        s.set_atom("defn", Expr::Special(alloc!(ctx, special_defn)));
        s.set_atom("if", Expr::Special(alloc!(ctx, special_if)));
        s.set_atom("let", Expr::Special(alloc!(ctx, special_let)));

        s.set_atom("+", Expr::Func(alloc!(ctx, eval_add)));
        s.set_atom("-", Expr::Func(alloc!(ctx, eval_sub)));
        s.set_atom("*", Expr::Func(alloc!(ctx, eval_mul)));
        s.set_atom("/", Expr::Func(alloc!(ctx, eval_div)));
        s.set_atom("<<", Expr::Func(alloc!(ctx, eval_bitshiftleft)));
        s.set_atom("=", Expr::Func(alloc!(ctx, eval_eq)));
        s.set_atom("<", Expr::Func(alloc!(ctx, eval_le)));
        s.set_atom("vec", Expr::Func(alloc!(ctx, eval_vec)));
        s.set_atom("index", Expr::Func(alloc!(ctx, eval_index)));
        s.set_atom("first", Expr::Func(alloc!(ctx, eval_first)));
        s.set_atom("rest", Expr::Func(alloc!(ctx, eval_rest)));
        s.set_atom("null?", Expr::Func(alloc!(ctx, eval_nullq)));
        s.set_atom("println", Expr::Func(alloc!(ctx, eval_println)));
        s.set_atom("concat", Expr::Func(alloc!(ctx, eval_concat)));
        s.set_atom("random", Expr::Func(alloc!(ctx, eval_random)));
        s.set_atom("len", Expr::Func(alloc!(ctx, eval_list)));
    }

    let mut res = Expr::Null;
    let exprs: Vec<Expr> = ast.iter().map(|x| Expr::from_ast(&mut ctx, x)).collect();
    for statement in exprs {
        res = eval(&mut ctx, s.clone(), statement);
    }

    // Uncomment to print final value.
    let _ = res;
    // println!("{:?}", res);

    println!("");
    println!("allocated objects: {:?}", ctx.alloc.size());

    Ok(())
}
